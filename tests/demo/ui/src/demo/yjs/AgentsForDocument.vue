<template>
  <div>
    <b>Agents</b>
    <ul>
      <li v-for="agent in agents">{{ encodeHashToBase64(agent) }}</li>
    </ul> 
  </div>
</template>

<script setup lang="ts">
import {ComputedRef, inject, onMounted, onUnmounted, ref} from "vue";
import { AppAgentWebsocket, decodeHashFromBase64, encodeHashToBase64 } from "@holochain/client";

const props = defineProps<{
  documentHash: Uint8Array;
}>();

const client = (inject("client") as ComputedRef<AppAgentWebsocket>).value;

const agents = ref([]);

const fetchAgentsForDocument = async () => {
  agents.value = await client.callZome({
    cap_secret: null,
    role_name: "demo",
    zome_name: "yjs",
    fn_name: "get_agents_for_document",
    payload: props.documentHash,
  });
};

let fetchInterval: NodeJS.Timer | undefined;
onMounted(() => {
  fetchAgentsForDocument();
  fetchInterval = setInterval(fetchAgentsForDocument, 2000);
});
onUnmounted(() => {
  if(fetchInterval) {
    clearInterval(fetchInterval);
  }
})
</script>