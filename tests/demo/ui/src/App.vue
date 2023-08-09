<template>
  <div style="width: 100%">
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else style="width: 100%">
      <div id="content" style="width: 100%; display: flex; flex-direction: column; flex: 1">
        <RouterView />
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, computed } from "vue";
import { AppAgentClient, AppAgentWebsocket } from "@holochain/client";
import "@material/mwc-circular-progress";

export default defineComponent({
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
  data(): {
    client: AppAgentClient | undefined;
    loading: boolean;
  } {
    return {
      client: undefined,
      loading: true,
    };
  },
  async mounted() {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    // @ts-ignore
    this.client = await AppAgentWebsocket.connect("", "demo");

    this.loading = false;
  },
});
</script>
