import { HolochainProvider } from "../../dist";
import * as Y from "yjs";
import { test, expect } from "vitest";
import { dhtSync, pause, runScenario } from "@holochain/tryorama";
import { AppAgentWebsocket, Record } from "@holochain/client";
import { isEqual } from "lodash-es";

async function waitFor(
  condition: () => boolean,
  timeoutMs = 10000,
  retryMs = 1000,
) {
  const start = Date.now();
  while (!condition()) {
    if (Date.now() - start > timeoutMs) {
      throw new Error("timeout");
    }
    // @ts-ignore
    await new Promise((r) => setTimeout(r, retryMs));
  }
}

test("Provider syncs doc across 2 unsynced peers", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/demo/workdir/demo.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a document
    const documentRecord: Record = await alice.cells[0].callZome({
      zome_name: "yjs",
      fn_name: "create_document",
      payload: {
        title: "My document",
      },
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob & Alice create a document on the same topic, with different contents
    const aliceDoc = new Y.Doc();
    const bobDoc = new Y.Doc();

    // Setup YJS Provider
    const aliceProvider = new HolochainProvider(
      aliceDoc,
      alice.appAgentWs as AppAgentWebsocket,
      "demo",
      "yjs",
      documentRecord.signed_action.hashed.hash,
    );
    const bobProvider = new HolochainProvider(
      bobDoc,
      bob.appAgentWs as AppAgentWebsocket,
      "demo",
      "yjs",
      documentRecord.signed_action.hashed.hash,
    );
    
    await waitFor(() => bobProvider.isReady && aliceProvider.isReady, 10000);
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice updates the document contents
    aliceDoc.getText("document").insert(0, "Hello");

    // Bob updates the document contents
    bobDoc.getText("document").insert(0, "Hola");

    // Wait for the state to be synced
    await waitFor(
      () => isEqual(Y.encodeStateVector(aliceDoc), Y.encodeStateVector(bobDoc)),
      100000,
    );

    expect(Y.encodeStateVector(aliceDoc)).toEqual(Y.encodeStateVector(bobDoc));
    expect(aliceDoc.getText("document").toString()).toEqual(
      bobDoc.getText("document").toString(),
    );
  });
});

test("Agent no longer sent signals after Provider.destroy()", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/demo/workdir/demo.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a document
    const documentRecord: Record = await alice.cells[0].callZome({
      zome_name: "yjs",
      fn_name: "create_document",
      payload: {
        title: "My document",
      },
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob & Alice create a document on the same topic, with different contents
    const aliceDoc = new Y.Doc();
    const bobDoc = new Y.Doc();

    // Setup YJS Provider
    const aliceProvider = new HolochainProvider(
      aliceDoc,
      alice.appAgentWs as AppAgentWebsocket,
      "demo",
      "yjs",
      documentRecord.signed_action.hashed.hash,
    );
    const bobProvider = new HolochainProvider(
      bobDoc,
      bob.appAgentWs as AppAgentWebsocket,
      "demo",
      "yjs",
      documentRecord.signed_action.hashed.hash,
    );
    
    await waitFor(() => bobProvider.isReady && aliceProvider.isReady, 10000);
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice updates the document contents
    aliceDoc.getText("document").insert(0, "Hello");

    // Wait for the state to be synced
    await waitFor(
      () => isEqual(Y.encodeStateVector(aliceDoc), Y.encodeStateVector(bobDoc)),
      100000,
    );
    expect(Y.encodeStateVector(aliceDoc)).toEqual(Y.encodeStateVector(bobDoc));

    // Destroy alice's provider
    await aliceProvider.destroy();

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    bobDoc.getText("document").insert(0, "Hola");

    await pause(2000);
    
    // Alice should not receive signal updates
    expect(Y.encodeStateVector(aliceDoc)).not.toEqual(Y.encodeStateVector(bobDoc));
  });
});


/*
test("provider syncs awareness", async () => {
  await runScenario(
    async (scenario) => {
      // Construct proper paths for your app.
      // This assumes app bundle created by the `hc app pack` command.
      const testAppPath = process.cwd() + "/../workdir/mewsfeed.happ";

      // Set up the app to be installed
      const appSource = { appBundleSource: { path: testAppPath } };

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      ydoc1.getText("testDoc").insert(0, "Hello")
      ydoc2.getText("testDoc").insert(0, "Hi")
    
      // Wait for the state to be synced
      try {
        await waitFor(() => Uint8ArrayEquals(Y.encodeStateVector(ydoc1), Y.encodeStateVector(ydoc2)))
      } catch (e) {
        printStates({ ydoc1, ydoc2 })
        throw e
      }
    
    
      expect(Y.encodeStateVector(ydoc1)).toEqual(Y.encodeStateVector(ydoc2))
      expect(provider1.awareness.getStates()).toEqual(provider2.awareness.getStates())
    }
  )
});
*/
