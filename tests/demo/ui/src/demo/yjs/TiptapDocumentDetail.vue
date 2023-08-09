<template>
  <div style="margin-bottom: 2rem">
    <button @click="router.back()">Back</button>
  </div>
  <div v-if="!loading && editor" style="width: 100%">
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="margin-right: 4px"><strong>Title: </strong></span>
      <span style="white-space: pre-line">{{ document?.title }} </span>
    </div>

    <div style="border: solid black 1px;">
      <div >
        <button @click="editor.chain().focus().toggleBold().run()" :disabled="!editor.can().chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }">
          bold
        </button>
        <button @click="editor.chain().focus().toggleItalic().run()" :disabled="!editor.can().chain().focus().toggleItalic().run()" :class="{ 'is-active': editor.isActive('italic') }">
          italic
        </button>
        <button @click="editor.chain().focus().toggleStrike().run()" :disabled="!editor.can().chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">
          strike
        </button>
        <button @click="editor.chain().focus().toggleCode().run()" :disabled="!editor.can().chain().focus().toggleCode().run()" :class="{ 'is-active': editor.isActive('code') }">
          code
        </button>
        <button @click="editor.chain().focus().unsetAllMarks().run()">
          clear marks
        </button>
        <button @click="editor.chain().focus().clearNodes().run()">
          clear nodes
        </button>
        <button @click="editor.chain().focus().setParagraph().run()" :class="{ 'is-active': editor.isActive('paragraph') }">
          paragraph
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 1 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 1 }) }">
          h1
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 2 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 2 }) }">
          h2
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 3 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 3 }) }">
          h3
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 4 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 4 }) }">
          h4
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 5 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 5 }) }">
          h5
        </button>
        <button @click="editor.chain().focus().toggleHeading({ level: 6 }).run()" :class="{ 'is-active': editor.isActive('heading', { level: 6 }) }">
          h6
        </button>
        <button @click="editor.chain().focus().toggleBulletList().run()" :class="{ 'is-active': editor.isActive('bulletList') }">
          bullet list
        </button>
        <button @click="editor.chain().focus().toggleOrderedList().run()" :class="{ 'is-active': editor.isActive('orderedList') }">
          ordered list
        </button>
        <button @click="editor.chain().focus().toggleCodeBlock().run()" :class="{ 'is-active': editor.isActive('codeBlock') }">
          code block
        </button>
        <button @click="editor.chain().focus().toggleBlockquote().run()" :class="{ 'is-active': editor.isActive('blockquote') }">
          blockquote
        </button>
        <button @click="editor.chain().focus().setHorizontalRule().run()">
          horizontal rule
        </button>
        <button @click="editor.chain().focus().setHardBreak().run()">
          hard break
        </button>
        <button @click="editor.chain().focus().undo().run()" :disabled="!editor.can().chain().focus().undo().run()">
          undo
        </button>
        <button @click="editor.chain().focus().redo().run()" :disabled="!editor.can().chain().focus().redo().run()">
          redo
        </button>
      </div>
      <EditorContent :editor="editor"/>
    </div>
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
import { decodeHashFromBase64 } from "@holochain/client";
import AgentsForDocument from "./AgentsForDocument.vue";
import StarterKit from "@tiptap/starter-kit";
import Collaboration from "@tiptap/extension-collaboration";
import {Editor, EditorContent} from "@tiptap/vue-3";
import { useRouter } from "vue-router";

const client = (inject("client") as ComputedRef<AppAgentWebsocket>).value;
const props = defineProps<{
  documentHashB64: string;
}>();
const emit = defineEmits(["document-deleted"]);
const router = useRouter();

const record = ref<Record>();
const loading = ref(true);
const editor = ref();
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

  provider = new HolochainProvider(
    ydoc,
    client,
    "demo",
    "yjs",
    decodeHashFromBase64(props.documentHashB64),
  );

  editor.value = new Editor({
    extensions: [
      StarterKit.configure({
        // The Collaboration extension comes with its own history handling
        history: false,
      }),
      // Register the document with Tiptap
      Collaboration.configure({
        document: ydoc,
      }),
    ],
  })
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
