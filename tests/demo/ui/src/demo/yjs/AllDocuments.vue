<template>
  <div
    v-if="loading"
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <CreateDocument />

    <span v-if="error"
      >Error fetching the documents: {{ error.data.data }}.</span
    >
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <h2>All Documents</h2>
      <DocumentListItem
        v-for="(hash, i) in hashes"
        :key="i"
        :document-hash="hash"
        @document-deleted="fetchDocument()"
      >
      </DocumentListItem>
    </div>
    <span v-else>No documents found.</span>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, toRaw, ComputedRef } from "vue";
import { AppAgentClient, Record, ActionHash } from "@holochain/client";
import "@material/mwc-circular-progress";
import DocumentListItem from "./DocumentListItem.vue";
import { YjsSignal } from "./types";
import CreateDocument from "./CreateDocument.vue";

export default defineComponent({
  components: {
    DocumentListItem,
    CreateDocument,
  },
  setup() {
    const client = (inject("client") as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): {
    hashes: Array<ActionHash> | undefined;
    loading: boolean;
    error: any;
  } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined,
    };
  },
  async mounted() {
    await this.fetchDocument();
    toRaw(this.client).on("signal", (signal) => {
      if (signal.zome_name !== "yjs") return;
      const payload = signal.payload as YjsSignal;
      if (payload.type !== "EntryCreated") return;
      if (payload.app_entry.type !== "Document") return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchDocument() {
      try {
        const records: Array<Record> = await this.client.callZome({
          cap_secret: null,
          role_name: "demo",
          zome_name: "yjs",
          fn_name: "get_all_documents",
          payload: null,
        });
        this.hashes = records.map((r) => r.signed_action.hashed.hash);
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    },
  },
});
</script>
