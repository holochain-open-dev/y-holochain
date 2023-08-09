import { createApp } from "vue";
import "./style.css";
import "./assets/quill.snow.css";
import { createRouter, createWebHashHistory } from "vue-router";
import QuillDocumentDetail from "./demo/yjs/QuillDocumentDetail.vue";
import TiptapDocumentDetail from "./demo/yjs/TiptapDocumentDetail.vue";
import AllDocuments from "./demo/yjs/AllDocuments.vue";
import App from "./App.vue";

const routes = [
  { name: "home", path: "/", component: AllDocuments },
  {
    name: "document/quill",
    path: "/document/quill/:documentHashB64",
    component: QuillDocumentDetail,
    props: true,
  },
  {
    name: "document/tiptap",
    path: "/document/tiptap/:documentHashB64",
    component: TiptapDocumentDetail,
    props: true,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

const app = createApp(App);
app.use(router);
app.mount("#app");
