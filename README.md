# y-holochain

[Holochain](https://holochain.org/) provider for [Yjs](https://github.com/yjs/yjs). Easily build peer-to-peer real-time collaboration software using Holochain.

Yjs already offers bindings for many popular rich text editors including [Quill](https://quilljs.com/), [ProseMirror](https://prosemirror.net/) and [TipTap](https://tiptap.dev/examples/default).

## Usage

1. Setup the zomes in your DNA
    1. hc scaffold zome yjs
    1. run `cargo add -p yjs hc_zome_yjs_coordinator`
    1. run `cargo add -p yjs_integrity hc_zome_yjs_integrity`
    1. Replace the integrity zome's `lib.rs` (its path may be similar to dnas/my-dna/zomes/integrity/yjs/src/lib.rs) and replace its contents with: `extern crate hc_zome_yjs_integrity;`
    1. Replace the coordinator zome's `lib.rs` (its path may be similar to dnas/my-dna/zomes/coordinator/yjs/src/lib.rs) and replace its contents with: `extern crate hc_zome_yjs_coordinator;`

1. Setup the Yjs provider in your front-end

```js
import { HolochainProvider } from 'y-holochain'
import { onMounted, onUnmounted } from 'vue';

let provider: HolochainProvider | undefined;
async setupYjsProvider() {
  // Create a document where this Yjs data will be stored
  const record = await client.callZome({
    zome_name: 'yjs',
    fn_name: 'create_document',
    payload: {
      title: 'My Cool Document'
    }
  });
  const documentActionHash = record.signed_action.hashed.hash;

  // Setup Yjs Doc & HolochainProvider
  const ydoc = new Y.Doc();
  provider = new HolochainProvider(
    ydoc,               // Yjs Y.Doc
    client,             // Holochain client
    "demo",             // RoleName of cell with 'yjs' zome
    "yjs",              // ZomeName of 'yjs' coordinator zome
    documentActionHash  // ActionHash of the document
  );  

  // Setup Yjs Editor Bindings of your choice
  const quill = new Quill("#my-editor-container");
  new QuillBinding(ydoc.getText('quill'), editor);
}

onMounted(() => {
  setupYjsProvider();
});

// Call destroy() when provider no longer in use
onUnmounted(() => {
  if(provider) {
    provider.destroy()
  }
});
```

YJS has bindings for many popular text & rich-text editors. See https://docs.yjs.dev/ecosystem/editor-bindings for details.
