<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Document</span>

    <div style="margin-bottom: 16px">
      <mwc-textfield
        outlined
        label="Title"
        :value="title"
        required
        @input="title = $event.target.value"
      ></mwc-textfield>
    </div>

    <mwc-button
      raised
      label="Create Document"
      :disabled="!isDocumentValid"
      @click="createDocument"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from "vue";
import {
  AppAgentClient,
  Record,
  AgentPubKey,
  EntryHash,
  ActionHash,
  DnaHash,
} from "@holochain/client";
import { Document } from "./types";
import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-snackbar";
import { Snackbar } from "@material/mwc-snackbar";
import "@material/mwc-textfield";

export default defineComponent({
  emits: ["document-created"],
  setup() {
    const client = (inject("client") as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): {
    title: string;
  } {
    return {
      title: "",
    };
  },
  computed: {
    isDocumentValid() {
      return true && this.title !== "";
    },
  },
  mounted() {},
  methods: {
    async createDocument() {
      const document: Document = {
        title: this.title!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: "demo",
          zome_name: "yjs",
          fn_name: "create_document",
          payload: document,
        });
        this.$emit("document-created", record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs["create-error"] as Snackbar;
        errorSnackbar.labelText = `Error creating the document: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
});
</script>
