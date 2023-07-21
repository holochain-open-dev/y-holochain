import { createApp } from "vue";
import "./style.css";
import "./assets/quill.snow.css";
import { createRouter, createWebHashHistory } from "vue-router";
import DocumentDetail from "./demo/yjs/DocumentDetail.vue";
import AllDocuments from "./demo/yjs/AllDocuments.vue";
import App from "./App.vue";

const routes = [
  { name: "home", path: "/", component: AllDocuments },
  {
    name: "document",
    path: "/document/:documentHashB64",
    component: DocumentDetail,
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
