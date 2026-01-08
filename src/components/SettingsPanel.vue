<script setup lang="ts">
import { onMounted, ref } from "vue";
import {
  NCard,
  NSpace,
  NButton,
  NDescriptions,
  NDescriptionsItem,
  NSelect,
  NDivider,
  NText,
  NTooltip,
  NSpin,
  NCollapse,
  NCollapseItem,
  useMessage,
} from "naive-ui";
import { FolderOpenOutline, CopyOutline } from "@vicons/ionicons5";
import { useSettingsStore } from "@/stores/settings";
import { invoke } from "@tauri-apps/api/core";

const settingsStore = useSettingsStore();
const message = useMessage();

// 本地状态
const selectedMirror = ref("");
const debugInfo = ref("");
const debugLoading = ref(false);

// 初始化
onMounted(async () => {
  await settingsStore.loadSettings();
  selectedMirror.value = settingsStore.nodeDistMirror;
});

// 复制到剪贴板
async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    message.success("已复制到剪贴板");
  } catch (e) {
    message.error("复制失败");
  }
}

// 打开 fnm 目录
async function handleOpenFnmDir() {
  const success = await settingsStore.openFnmDirectory();
  if (!success) {
    message.error(settingsStore.error || "打开目录失败");
  }
}

// 镜像源选项
const mirrorOptions = settingsStore.mirrorOptions.map((m) => ({
  label: `${m.label} (${m.value})`,
  value: m.value,
}));

// 生成 export 命令
function getExportCommand() {
  return `export FNM_NODE_DIST_MIRROR="${selectedMirror.value}"`;
}

// 调试 fnm 查找
async function runDebug() {
  debugLoading.value = true;
  try {
    debugInfo.value = await invoke<string>("debug_fnm_lookup");
  } catch (e) {
    debugInfo.value = `调试命令执行失败: ${e}`;
  } finally {
    debugLoading.value = false;
  }
}
</script>

<template>
  <div class="settings-panel">
    <NSpin :show="settingsStore.loading">
      <NCard title="fnm 配置信息" size="small">
        <NDescriptions label-placement="left" :column="1" bordered>
          <NDescriptionsItem label="安装目录">
            <NSpace align="center">
              <NText code style="word-break: break-all">
                {{ settingsStore.fnmDir || "未知" }}
              </NText>
              <NTooltip>
                <template #trigger>
                  <NButton
                    quaternary
                    circle
                    size="tiny"
                    @click="copyToClipboard(settingsStore.fnmDir)"
                  >
                    <template #icon>
                      <CopyOutline />
                    </template>
                  </NButton>
                </template>
                复制路径
              </NTooltip>
              <NTooltip>
                <template #trigger>
                  <NButton
                    quaternary
                    circle
                    size="tiny"
                    @click="handleOpenFnmDir"
                  >
                    <template #icon>
                      <FolderOpenOutline />
                    </template>
                  </NButton>
                </template>
                打开目录
              </NTooltip>
            </NSpace>
          </NDescriptionsItem>

          <NDescriptionsItem label="系统架构">
            <NText code>{{ settingsStore.arch || "未知" }}</NText>
          </NDescriptionsItem>

          <NDescriptionsItem label="版本文件策略">
            <NText code>{{ settingsStore.versionFileStrategy }}</NText>
          </NDescriptionsItem>

          <NDescriptionsItem label="Corepack">
            <NText
              :type="settingsStore.corepackEnabled ? 'success' : 'default'"
            >
              {{ settingsStore.corepackEnabled ? "已启用" : "未启用" }}
            </NText>
          </NDescriptionsItem>

          <NDescriptionsItem label="解析 Engines">
            <NText :type="settingsStore.resolveEngines ? 'success' : 'default'">
              {{ settingsStore.resolveEngines ? "已启用" : "未启用" }}
            </NText>
          </NDescriptionsItem>

          <NDescriptionsItem label="日志级别">
            <NText code>{{ settingsStore.loglevel }}</NText>
          </NDescriptionsItem>
        </NDescriptions>
      </NCard>

      <NDivider />

      <NCard title="镜像源配置" size="small">
        <NSpace vertical :size="12">
          <NText depth="3" style="font-size: 13px">
            选择 Node.js 下载镜像源，国内用户建议使用淘宝或腾讯镜像以加速下载。
          </NText>

          <NSelect
            v-model:value="selectedMirror"
            :options="mirrorOptions"
            placeholder="选择镜像源"
            disabled
          />

          <NText depth="3" style="font-size: 12px">
            💡 提示：镜像源配置需要通过设置环境变量 FNM_NODE_DIST_MIRROR
            来更改。
          </NText>

          <NSpace style="margin-top: 8px">
            <NButton size="small" @click="copyToClipboard(getExportCommand())">
              复制 export 命令
            </NButton>
          </NSpace>
        </NSpace>
      </NCard>

      <NDivider />

      <NCollapse>
        <NCollapseItem title="调试信息" name="debug">
          <NSpace vertical :size="12">
            <NSpace>
              <NButton
                size="small"
                :loading="debugLoading"
                @click="runDebug"
              >
                运行 fnm 检测诊断
              </NButton>
              <NButton
                v-if="debugInfo"
                size="small"
                @click="copyToClipboard(debugInfo)"
              >
                复制调试信息
              </NButton>
            </NSpace>
            <pre
              v-if="debugInfo"
              style="
                background: #1a1a1a;
                padding: 12px;
                border-radius: 4px;
                font-size: 12px;
                overflow-x: auto;
                white-space: pre-wrap;
                word-break: break-all;
              "
            >{{ debugInfo }}</pre>
          </NSpace>
        </NCollapseItem>
      </NCollapse>
    </NSpin>
  </div>
</template>

<style scoped>
.settings-panel {
  padding: 16px;
}
</style>
