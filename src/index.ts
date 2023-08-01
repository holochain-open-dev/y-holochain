import * as Y from "yjs";
import { Observable } from "lib0/observable";
import {
  AppAgentWebsocket,
  Record,
  encodeHashToBase64,
  RoleName,
  ZomeName,
  ActionHash,
  AppSignal,
} from "@holochain/client";
import { CreateStatevectorForDocumentInput, Statevector } from "./types";
import { isEqual } from "lodash-es";
import {
  setIntervalAsync,
  clearIntervalAsync,
  SetIntervalAsyncTimer,
} from "set-interval-async";
import { decode } from "@msgpack/msgpack";

class HolochainProvider extends Observable<string> {
  ydoc: Y.Doc;
  client: AppAgentWebsocket;
  roleName: RoleName;
  zomeName: ZomeName;
  documentActionHash: ActionHash;
  publishQueue: Array<Uint8Array>;
  pollingInterval: SetIntervalAsyncTimer<unknown[]> | undefined;
  publishInterval: SetIntervalAsyncTimer<unknown[]> | undefined;

  constructor(
    ydoc: Y.Doc,
    client: AppAgentWebsocket,
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
    // this.publishInterval = setIntervalAsync(async () => {
    //   await this._publishQueued();
    // }, 50);

    // Add agent to document, so they receive signal updates
    await this._ensureAgentForDocument();

    // Read initial document state and publish to DHT
    this._publishInitialState();

    // Load initial DHT state and apply to document
    this._fetchAndApplyUpdates();

    // Publish to DHT when document is updated
    this.ydoc.on("update", this._onDocUpdate.bind(this));

    // Poll for DHT state changes and apply to document
    // this.pollingInterval = setIntervalAsync(async () => {
    //   await this._fetchAndApplyUpdates();
    // }, 1000);

    // Listen for state change signals from holochain and apply to document
    this.client.on('signal', this._onSignal.bind(this));

    console.log(
      `Initialized YJS connection for ${encodeHashToBase64(
        this.documentActionHash,
      )}`,
    );
  }

  private async _ensureAgentForDocument(): Promise<void> {
    await this.client.callZome({
      role_name: this.roleName,
      zome_name: this.zomeName,
      fn_name: 'ensure_agent_for_document',
      payload: {
        base_document_hash: this.documentActionHash,
        target_agent: this.client.myPubKey
      }
    });
  }

  private async _publishInitialState(): Promise<void> {
    const state = Y.encodeStateAsUpdate(this.ydoc);

    if (!isEqual(state, new Uint8Array([0, 0]))) {
      this.publishQueue.push(state);
    }
  }

  private _onDocUpdate(
    update: Uint8Array,
    origin: this | any,
  ): void {
    if (origin !== this) {
      console.log('on doc update');
      this.client.callZome({
        role_name: this.roleName,
        zome_name: this.zomeName,
        fn_name: "remote_signal_statevector_for_document",
        payload: {
          document_hash: this.documentActionHash,
          statevector: {
            data: update,
          },
        },
      });
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

  // private async _publishQueued(): Promise<void> {
  //   while (this.publishQueue.length > 0) {
  //     const update = this.publishQueue.pop();
  // 
  //     if ((this.client.appWebsocket.client.socket.readyState as number) !== 1)
  //       return;
  //     await this.client.callZome({
  //       role_name: this.roleName,
  //       zome_name: this.zomeName,
  //       fn_name: "remote_signal_statevector_for_document",
  //       payload: {
  //         document_hash: this.documentActionHash,
  //         statevector: {
  //           data: update,
  //         },
  //       },
  //     });
  //   }
  // }

  private async _onSignal(signal: AppSignal): Promise<void> {
    const payload = signal.payload as CreateStatevectorForDocumentInput;

    if(isEqual(payload.document_hash, this.documentActionHash)) {
      Y.applyUpdate(this.ydoc, payload.statevector.data);
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
