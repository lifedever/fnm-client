<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import {
  NCard,
  NTabs,
  NTabPane,
  NSpace,
  NButton,
  NInput,
  NSwitch,
  NEmpty,
  NSpin,
  NScrollbar,
  NTag,
  NPopconfirm,
  NTooltip,
  NGrid,
  NGi,
  useMessage,
} from "naive-ui";
import {
  RefreshOutline,
  DownloadOutline,
  TrashOutline,
  CheckmarkCircleOutline,
  FolderOpenOutline,
} from "@vicons/ionicons5";
import { useVersionStore } from "@/stores/version";
import type { NodeVersion } from "@/types/fnm";

const versionStore = useVersionStore();
const message = useMessage();

// 本地状态
const searchKeyword = ref("");
const ltsOnly = ref(false);
const activeTab = ref<"installed" | "remote">("installed");
const installingVersion = ref<string | null>(null);

// 分页状态
const PAGE_SIZE = 10;
const displayCount = ref(PAGE_SIZE);

// 搜索防抖
let searchTimeout: ReturnType<typeof setTimeout> | null = null;

// 计算属性 - 已安装版本（本地过滤）
const filteredInstalledVersions = computed(() => {
  return versionStore.getFilteredVersions("installed", {
    ltsOnly: ltsOnly.value,
    keyword: searchKeyword.value,
  });
});

// 计算属性 - 远程版本（分页显示）
const paginatedRemoteVersions = computed(() => {
  const versions = versionStore.remoteVersions;
  return versions.slice(0, displayCount.value);
});

const hasMoreRemoteVersions = computed(() => {
  return displayCount.value < versionStore.remoteVersions.length;
});

const remainingCount = computed(() => {
  return versionStore.remoteVersions.length - displayCount.value;
});

// 初始化
onMounted(async () => {
  await versionStore.fetchInstalledVersions();
});

// 监听 Tab 切换，懒加载远程版本
watch(activeTab, (newVal) => {
  if (newVal === "remote" && versionStore.remoteVersions.length === 0) {
    displayCount.value = PAGE_SIZE;
    versionStore.fetchRemoteVersions({ lts: ltsOnly.value });
  }
});

// 搜索（防抖）
function handleSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  searchTimeout = setTimeout(() => {
    if (activeTab.value === "remote") {
      displayCount.value = PAGE_SIZE;
      versionStore.fetchRemoteVersions({
        lts: ltsOnly.value,
        filter: searchKeyword.value || undefined,
      });
    }
    // 已安装版本使用本地过滤，不需要额外处理
  }, 500);
}

// 加载更多
function loadMore() {
  displayCount.value += PAGE_SIZE;
}

// 刷新版本列表
async function handleRefresh() {
  if (activeTab.value === "installed") {
    await versionStore.fetchInstalledVersions();
    message.success("已刷新本地版本列表");
  } else {
    displayCount.value = PAGE_SIZE;
    await versionStore.fetchRemoteVersions({
      lts: ltsOnly.value,
      filter: searchKeyword.value || undefined,
    });
    message.success("已刷新远程版本列表");
  }
}

// LTS 切换时重新获取远程版本
watch(ltsOnly, (newVal) => {
  if (activeTab.value === "remote") {
    displayCount.value = PAGE_SIZE;
    versionStore.fetchRemoteVersions({
      lts: newVal,
      filter: searchKeyword.value || undefined,
    });
  }
});

// 设为默认
async function handleSetDefault(version: NodeVersion) {
  const success = await versionStore.setDefaultVersion(version.name);
  if (success) {
    message.success(`已将 ${version.name} 设为默认版本`);
  } else {
    message.error(versionStore.error || "设置默认版本失败");
  }
}

// 安装版本
import type { MessageReactive } from "naive-ui";
let loadingMessage: MessageReactive | null = null;

async function handleInstall(version: NodeVersion) {
  installingVersion.value = version.name;
  loadingMessage = message.loading(`正在安装 ${version.name}...`, {
    duration: 0,
  });

  const success = await versionStore.installVersion(version.name);

  if (loadingMessage) {
    loadingMessage.destroy();
    loadingMessage = null;
  }
  installingVersion.value = null;

  if (success) {
    message.success(`${version.name} 安装成功`);
  } else {
    message.error(versionStore.error || "安装失败");
  }
}

// 卸载版本
async function handleUninstall(version: NodeVersion) {
  if (version.isCurrent) {
    message.warning("无法卸载当前正在使用的版本");
    return;
  }

  const success = await versionStore.uninstallVersion(version.name);
  if (success) {
    message.success(`${version.name} 已卸载`);
  } else {
    message.error(versionStore.error || "卸载失败");
  }
}

// 打开目录
async function handleOpenDir(version: NodeVersion) {
  const success = await versionStore.openVersionDirectory(version.name);
  if (!success) {
    message.error(versionStore.error || "打开目录失败");
  }
}
</script>

<template>
  <div class="version-list">
    <!-- 工具栏 -->
    <div class="toolbar">
      <NSpace align="center" justify="space-between" style="width: 100%">
        <NSpace align="center">
          <NInput
            v-model:value="searchKeyword"
            placeholder="搜索版本..."
            clearable
            style="width: 200px"
            @input="handleSearch"
            @clear="handleSearch"
          />
          <NSpace align="center" :size="4">
            <span class="filter-label">仅 LTS</span>
            <NSwitch v-model:value="ltsOnly" size="small" />
          </NSpace>
        </NSpace>
        <NButton quaternary circle @click="handleRefresh">
          <template #icon>
            <RefreshOutline />
          </template>
        </NButton>
      </NSpace>
    </div>

    <!-- Tab 切换 -->
    <NTabs
      v-model:value="activeTab"
      type="segment"
      animated
      class="tabs-container"
    >
      <!-- 已安装版本 -->
      <NTabPane name="installed" tab="已安装">
        <NSpin :show="versionStore.loading">
          <NScrollbar class="version-scrollbar">
            <div
              v-if="filteredInstalledVersions.length === 0"
              class="empty-state"
            >
              <NEmpty description="暂无已安装的版本" />
            </div>
            <NGrid v-else :cols="1" :y-gap="8">
              <NGi
                v-for="version in filteredInstalledVersions"
                :key="version.name"
              >
                <NCard size="small" hoverable class="version-card">
                  <NSpace align="center" justify="space-between">
                    <!-- 版本信息 -->
                    <NSpace align="center">
                      <span class="version-name">{{ version.name }}</span>
                      <NTag v-if="version.isCurrent" type="success" size="small"
                        >当前</NTag
                      >
                      <NTag v-if="version.isDefault" type="warning" size="small"
                        >默认</NTag
                      >
                      <NTag v-if="version.isLts" type="info" size="small">
                        LTS{{ version.ltsName ? ` (${version.ltsName})` : "" }}
                      </NTag>
                    </NSpace>

                    <!-- 操作按钮 -->
                    <NSpace>
                      <NTooltip>
                        <template #trigger>
                          <NButton
                            size="small"
                            quaternary
                            :disabled="version.isDefault"
                            @click="handleSetDefault(version)"
                          >
                            <template #icon>
                              <CheckmarkCircleOutline />
                            </template>
                          </NButton>
                        </template>
                        设为默认版本
                      </NTooltip>

                      <NTooltip>
                        <template #trigger>
                          <NButton
                            size="small"
                            quaternary
                            @click="handleOpenDir(version)"
                          >
                            <template #icon>
                              <FolderOpenOutline />
                            </template>
                          </NButton>
                        </template>
                        打开安装目录
                      </NTooltip>

                      <NPopconfirm @positive-click="handleUninstall(version)">
                        <template #trigger>
                          <NTooltip>
                            <template #trigger>
                              <NButton
                                size="small"
                                quaternary
                                type="error"
                                :disabled="version.isCurrent"
                              >
                                <template #icon>
                                  <TrashOutline />
                                </template>
                              </NButton>
                            </template>
                            卸载此版本
                          </NTooltip>
                        </template>
                        确定要卸载 {{ version.name }} 吗？
                      </NPopconfirm>
                    </NSpace>
                  </NSpace>
                </NCard>
              </NGi>
            </NGrid>
          </NScrollbar>
        </NSpin>
      </NTabPane>

      <!-- 远程版本 -->
      <NTabPane name="remote" tab="远程版本">
        <NSpin :show="versionStore.remoteLoading">
          <NScrollbar class="version-scrollbar">
            <div v-if="paginatedRemoteVersions.length === 0" class="empty-state">
              <NEmpty description="暂无可用版本" />
            </div>
            <NGrid v-else :cols="1" :y-gap="8">
              <NGi
                v-for="version in paginatedRemoteVersions"
                :key="version.name"
              >
                <NCard size="small" hoverable class="version-card">
                  <NSpace align="center" justify="space-between">
                    <!-- 版本信息 -->
                    <NSpace align="center">
                      <span class="version-name">{{ version.name }}</span>
                      <NTag
                        v-if="version.isInstalled"
                        type="success"
                        size="small"
                        >已安装</NTag
                      >
                      <NTag v-if="version.isLts" type="info" size="small">
                        LTS{{ version.ltsName ? ` (${version.ltsName})` : "" }}
                      </NTag>
                    </NSpace>

                    <!-- 操作按钮 -->
                    <NSpace>
                      <NButton
                        v-if="!version.isInstalled"
                        size="small"
                        type="primary"
                        :loading="installingVersion === version.name"
                        :disabled="installingVersion !== null"
                        @click="handleInstall(version)"
                      >
                        <template #icon>
                          <DownloadOutline />
                        </template>
                        安装
                      </NButton>
                      <NTag v-else type="success" size="small">已安装</NTag>
                    </NSpace>
                  </NSpace>
                </NCard>
              </NGi>

              <!-- 加载更多按钮 -->
              <NGi v-if="hasMoreRemoteVersions">
                <div class="load-more">
                  <NButton
                    text
                    type="primary"
                    @click="loadMore"
                  >
                    加载更多 (还有 {{ remainingCount }} 个版本)
                  </NButton>
                </div>
              </NGi>
            </NGrid>
          </NScrollbar>
        </NSpin>
      </NTabPane>
    </NTabs>
  </div>
</template>

<style scoped>
.version-list {
  padding: 16px;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.toolbar {
  margin-bottom: 12px;
  flex-shrink: 0;
}

.filter-label {
  font-size: 13px;
  color: var(--n-text-color-3);
}

.version-scrollbar {
  flex: 1;
  max-height: calc(100vh - 200px);
}

.tabs-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.tabs-container :deep(.n-tabs-pane-wrapper) {
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.tabs-container :deep(.n-tab-pane) {
  height: 100%;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.tabs-container :deep(.n-spin-container),
.tabs-container :deep(.n-spin-content) {
  height: 100%;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.version-card {
  transition: all 0.2s ease;
}

.version-card:hover {
  transform: translateY(-1px);
}

.version-card :deep(.n-card__content) {
  padding: 8px 12px;
}

.version-name {
  font-weight: 600;
  font-size: 14px;
  font-family: "SF Mono", Monaco, monospace;
}

.empty-state {
  padding: 32px 0;
}

.load-more {
  text-align: center;
  padding: 16px 0;
}
</style>
