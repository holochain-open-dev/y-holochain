<template>
  <div style="margin-bottom: 2rem;">
    <button @click="router.back()">Back</button>
  </div>
  <div v-if="!loading" style="width: 100%">
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="margin-right: 4px"><strong>Title: </strong></span>
      <span style="white-space: pre-line">{{ document?.title }} </span>
    </div>
    <div style="width: 100%" id="editor-container"></div>

    <AgentsForDocument :document-hash="decodeHashFromBase64($props.documentHashB64)" />
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
import { inject, ComputedRef, ref, computed, onMounted, onUnmounted, onBeforeUnmount } from "vue";
import { decode } from "@msgpack/msgpack";
import { AppAgentWebsocket, Record } from "@holochain/client";
import { Document } from "./types";
import "@material/mwc-circular-progress";
import "@material/mwc-icon-button";
import "@material/mwc-snackbar";
import { HolochainProvider } from "../../../../../../dist/";
import * as Y from "yjs";
import { QuillBinding } from "y-quill";
import Quill from "quill";
import { decodeHashFromBase64 } from "@holochain/client";
import AgentsForDocument from "./AgentsForDocument.vue";
import { useRouter} from "vue-router";

const client = (inject("client") as ComputedRef<AppAgentWebsocket>).value;
const props = defineProps<{
  documentHashB64: string;
}>();
const emit = defineEmits(["document-deleted"]);
const router = useRouter();

const record = ref<Record>();
const loading = ref(true);
const errorSnackbar = ref();
let ydoc: Y.Doc | undefined;
let provider: HolochainProvider | undefined;
const document = computed(() => {
  if (!record.value) return undefined;
  return decode((record.value.entry as any).Present.entry) as Document;
});

onMounted(async () => {
  if (props.documentHashB64 === undefined) {
    throw new Error(
      `The documentHash input is required for the DocumentDetail element`,
    );
  }

  await fetchDocument();

  ydoc = new Y.Doc();
  const ydoctext = ydoc.getText("quill");

  var editor = new Quill("#editor-container", {
    modules: {
      toolbar: [
        [{ header: [1, 2, false] }],
        ["bold", "italic", "underline"],
        ["image", "code-block"],
      ],
    },
    placeholder: "Start collaborating...",
    theme: "snow", // or 'bubble'
  });

  // Optionally specify an Awareness instance, if supported by the Provider
  new QuillBinding(ydoctext, editor);

  provider = new HolochainProvider(
    ydoc,
    client,
    "demo",
    "yjs",
    decodeHashFromBase64(props.documentHashB64),
  );
});

onUnmounted(() => {
  if(provider) {
    provider.destroy();
  }
})

const fetchDocument = async () => {
  loading.value = true;
  record.value = undefined;

  record.value = await client.callZome({
    cap_secret: null,
    role_name: "demo",
    zome_name: "yjs",
    fn_name: "get_document",
    payload: decodeHashFromBase64(props.documentHashB64),
  });

  loading.value = false;
};
</script>
