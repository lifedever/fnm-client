<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { NSpace, NText, NDivider, NTag, NButton, NTooltip } from "naive-ui";
import { LogoGithub } from "@vicons/ionicons5";
import { useVersionStore } from "@/stores/version";
import { getVersion } from "@tauri-apps/api/app";
import { openUrl } from "@tauri-apps/plugin-opener";

const versionStore = useVersionStore();
const appVersion = ref("");

onMounted(async () => {
  appVersion.value = await getVersion();
});

const statusText = computed(() => {
  const current = versionStore.currentVersion;
  if (current) {
    return `Node ${current}`;
  }
  return "未检测到 Node 版本";
});

const installedCount = computed(() => versionStore.installedVersions.length);

function openGitHub() {
  openUrl("https://github.com/lifedever/fnm-client");
}
</script>

<template>
  <div class="status-bar">
    <NSpace align="center" justify="space-between" style="width: 100%">
      <NSpace align="center" :size="16">
        <NSpace align="center" :size="4">
          <span class="label">当前版本:</span>
          <NTag type="success" size="small" round>
            {{ statusText }}
          </NTag>
        </NSpace>

        <NDivider vertical />

        <NSpace align="center" :size="4">
          <span class="label">已安装:</span>
          <NText>{{ installedCount }} 个版本</NText>
        </NSpace>
      </NSpace>

      <NSpace align="center" :size="8">
        <NText depth="3" style="font-size: 12px"> fnm GUI v{{ appVersion }} </NText>
        <NTooltip>
          <template #trigger>
            <NButton quaternary circle size="tiny" @click="openGitHub">
              <template #icon>
                <LogoGithub />
              </template>
            </NButton>
          </template>
          GitHub
        </NTooltip>
      </NSpace>
    </NSpace>
  </div>
</template>

<style scoped>
.status-bar {
  padding: 8px 16px;
  background: var(--n-color-modal);
  border-top: 1px solid var(--n-border-color);
  font-size: 13px;
}

.label {
  color: var(--n-text-color-3);
}
</style>
