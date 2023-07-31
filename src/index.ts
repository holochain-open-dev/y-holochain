import * as Y from "yjs";
import { Observable } from "lib0/observable";
import {
  AppAgentWebsocket,
  Record,
  encodeHashToBase64,
  RoleName,
  ZomeName,
  ActionHash,
} from "@holochain/client";
import { IAppAgentWebsocket } from "@holochain/tryorama";
import { Statevector } from "./types";
import { isEqual } from "lodash-es";
import {
  setIntervalAsync,
  clearIntervalAsync,
  SetIntervalAsyncTimer,
} from "set-interval-async";
import { decode } from "@msgpack/msgpack";

class HolochainProvider extends Observable<string> {
  ydoc: Y.Doc;
  client: AppAgentWebsocket | IAppAgentWebsocket;
  roleName: RoleName;
  zomeName: ZomeName;
  documentActionHash: ActionHash;
  publishQueue: Array<Uint8Array>;
  pollingInterval: SetIntervalAsyncTimer<unknown[]> | undefined;
  publishInterval: SetIntervalAsyncTimer<unknown[]> | undefined;

  constructor(
    ydoc: Y.Doc,
    client: AppAgentWebsocket | IAppAgentWebsocket,
    roleName: RoleName,
    zomeName: ZomeName,
    documentActionHash: ActionHash,
  ) {
    super();

    this.ydoc = ydoc;
    this.client = client;
    this.roleName = roleName;
    this.zomeName = zomeName;
    this.documentActionHash = documentActionHash;
    this.publishQueue = [];
    this.pollingInterval = undefined;
    this.publishInterval = undefined;

    this._init();
  }

  private async _init(): Promise<void> {
    // Publish changes to DHT, avoiding 'chain head moved' errors
    this.publishInterval = setIntervalAsync(async () => {
      await this._publishQueued();
    }, 50);

    // Read initial document state and publish to DHT
    this._publishInitialState();

    // Load initial DHT state and apply to document
    this._fetchAndApplyUpdates();

    // Publish to DHT when document is updated
    this.ydoc.on("update", this._onDocUpdate.bind(this));

    // Poll for DHT state changes and apply to document
    this.pollingInterval = setIntervalAsync(async () => {
      await this._fetchAndApplyUpdates();
    }, 1000);

    console.log(
      `Initialized YJS connection for ${encodeHashToBase64(
        this.documentActionHash,
      )}`,
    );
  }

  private async _publishInitialState(): Promise<void> {
    const state = Y.encodeStateAsUpdate(this.ydoc);

    if (!isEqual(state, new Uint8Array([0, 0]))) {
      this.publishQueue.push(state);
    }
  }

  private async _onDocUpdate(
    update: Uint8Array,
    origin: this | any,
  ): Promise<void> {
    if (origin !== this) {
      this.publishQueue.push(update);
    }
  }

  private async _fetchUpdates(): Promise<Uint8Array | undefined> {
    const updates: Record[] = await this.client.callZome({
      role_name: this.roleName,
      zome_name: this.zomeName,
      fn_name: "get_statevectors_for_document",
      payload: this.documentActionHash,
    });
    if (updates.length === 0) return undefined;

    const updateStatevectors = updates.map(
      (r) =>
        new Uint8Array(
          (decode((r.entry as any).Present.entry) as Statevector).data,
        ),
    );
    return Y.mergeUpdates(updateStatevectors);
  }

  private async _fetchAndApplyUpdates(): Promise<void> {
    const update = await this._fetchUpdates();

    if (update) {
      Y.applyUpdate(this.ydoc, update);
    }
  }

  private async _publishQueued(): Promise<void> {
    while (this.publishQueue.length > 0) {
      const update = this.publishQueue.pop();


      if ((this.client instanceof AppAgentWebsocket) && (this.client.appWebsocket.client.socket.readyState as number) !== 1)
        return;

      await this.client.callZome({
        role_name: this.roleName,
        zome_name: this.zomeName,
        fn_name: "create_statevector_for_document",
        payload: {
          document_hash: this.documentActionHash,
          statevector: {
            data: update,
          },
        },
      });
    }
  }

  destroy(): void {
    if (this.pollingInterval) {
      clearIntervalAsync(this.pollingInterval);
    }
    if (this.publishInterval) {
      clearIntervalAsync(this.publishInterval);
    }
    console.log(
      `Destroyed YJS connection for ${encodeHashToBase64(
        this.documentActionHash,
      )}`,
    );
  }
}

export { HolochainProvider };
