<template>
  <div v-if="!loading" 
    style="
      cursor: pointer;
      display: flex;
      flex-direction: row;
      margin-bottom: 16px;
      padding: 5px;
    ">
    {{ document?.title }} 
    <RouterLink
      tag="button"
      :to="{
        name: 'document/quill',
        params: { documentHashB64: encodeHashToBase64(documentHash) },
      }"
      custom
      v-slot="{ navigate }"
    >
      <button
        style="margin-left: 0.5rem;"
        @click="navigate"
        role="link"
      >
        Quill
      </button>
    </RouterLink>
    <RouterLink
      tag="button"
      :to="{
        name: 'document/tiptap',
        params: { documentHashB64: encodeHashToBase64(documentHash) },
      }"
      custom
      v-slot="{ navigate }"
    >
      <button
        style="margin-left: 0.5rem;"
        @click="navigate"
        role="link"
      >
        TipTap
      </button>
    </RouterLink>
  </div>

  <div
    v-else
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="errorSnackbar" leading> </mwc-snackbar>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, ref, computed, onMounted } from "vue";
import { decode } from "@msgpack/msgpack";
import {
  AppAgentWebsocket,
  Record,
  encodeHashToBase64,
} from "@holochain/client";
import { Document } from "./types";
import "@material/mwc-circular-progress";
import "@material/mwc-icon-button";
import "@material/mwc-snackbar";
import { RouterLink } from "vue-router";

const client = (inject("client") as ComputedRef<AppAgentWebsocket>).value;
const props = defineProps<{
  documentHash: Uint8Array;
}>();
const emit = defineEmits(["document-deleted"]);

const record = ref<Record>();
const loading = ref(true);
const errorSnackbar = ref();

const document = computed(() => {
  if (!record.value) return undefined;
  return decode((record.value.entry as any).Present.entry) as Document;
});

onMounted(async () => {
  if (props.documentHash === undefined) {
    throw new Error(
      `The documentHash input is required for the DocumentDetail element`,
    );
  }

  await fetchDocument();
});

const fetchDocument = async () => {
  loading.value = true;
  record.value = undefined;

  record.value = await client.callZome({
    cap_secret: null,
    role_name: "demo",
    zome_name: "yjs",
    fn_name: "get_document",
    payload: props.documentHash,
  });

  loading.value = false;
};
</script>
