<script setup lang="ts">
import { ref, onMounted } from "vue";
import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NLayoutFooter,
  NMenu,
  NIcon,
  darkTheme,
} from "naive-ui";
import type { MenuOption } from "naive-ui";
import { h } from "vue";
import {
  LayersOutline,
  SettingsOutline,
  InformationCircleOutline,
} from "@vicons/ionicons5";

import VersionList from "@/components/VersionList.vue";
import SettingsPanel from "@/components/SettingsPanel.vue";
import StatusBar from "@/components/StatusBar.vue";
import { useVersionStore } from "@/stores/version";

const versionStore = useVersionStore();

// 当前选中的菜单
const activeMenu = ref<string>("versions");

// 菜单选项
const menuOptions: MenuOption[] = [
  {
    label: "版本管理",
    key: "versions",
    icon: () => h(NIcon, null, { default: () => h(LayersOutline) }),
  },
  {
    label: "设置",
    key: "settings",
    icon: () => h(NIcon, null, { default: () => h(SettingsOutline) }),
  },
  {
    label: "关于",
    key: "about",
    icon: () => h(NIcon, null, { default: () => h(InformationCircleOutline) }),
  },
];

// 初始化
onMounted(async () => {
  await versionStore.fetchInstalledVersions();
});

// 处理菜单选择
function handleMenuSelect(key: string) {
  activeMenu.value = key;
}
</script>

<template>
  <NConfigProvider :theme="darkTheme">
    <NMessageProvider>
      <NDialogProvider>
        <NLayout has-sider style="height: 100vh">
          <!-- 侧边栏 -->
          <NLayoutSider
            bordered
            :width="180"
            :collapsed-width="64"
            show-trigger
            collapse-mode="width"
          >
            <div class="logo">
              <span class="logo-icon">⚡</span>
              <span class="logo-text">fnm GUI</span>
            </div>
            <NMenu
              :value="activeMenu"
              :options="menuOptions"
              :collapsed-width="64"
              :collapsed-icon-size="22"
              @update:value="handleMenuSelect"
            />
          </NLayoutSider>

          <!-- 主内容区 -->
          <NLayout>
            <NLayoutContent
              style="height: calc(100vh - 50px); overflow: hidden"
            >
              <!-- 版本管理 -->
              <VersionList v-if="activeMenu === 'versions'" />

              <!-- 设置面板 -->
              <SettingsPanel v-else-if="activeMenu === 'settings'" />

              <!-- 关于页面 -->
              <div v-else-if="activeMenu === 'about'" class="about-page">
                <div class="about-content">
                  <div class="about-logo">⚡</div>
                  <h1>fnm GUI</h1>
                  <p class="version">版本 0.1.0</p>
                  <p class="description">
                    一个基于 Tauri + Vue 3 的 fnm 图形界面客户端
                  </p>
                  <div class="links">
                    <p>🚀 快速管理 Node.js 版本</p>
                    <p>📦 一键安装/卸载/切换</p>
                    <p>🎨 现代化暗色界面</p>
                  </div>
                  <p class="copyright">
                    Built with ❤️ using Tauri + Vue 3 + Naive UI
                  </p>
                </div>
              </div>
            </NLayoutContent>

            <!-- 状态栏 -->
            <NLayoutFooter bordered style="height: 50px">
              <StatusBar />
            </NLayoutFooter>
          </NLayout>
        </NLayout>
      </NDialogProvider>
    </NMessageProvider>
  </NConfigProvider>
</template>

<style>
/* 全局样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
    Ubuntu, Cantarell, sans-serif;
}

/* 禁止文本选择（桌面应用体验） */
body {
  user-select: none;
  -webkit-user-select: none;
}

/* 允许输入框选择 */
input,
textarea {
  user-select: text;
  -webkit-user-select: text;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>

<style scoped>
.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px 16px;
  font-size: 18px;
  font-weight: 600;
  border-bottom: 1px solid var(--n-border-color);
}

.logo-icon {
  font-size: 24px;
}

.logo-text {
  transition: opacity 0.3s;
}

.about-page {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
}

.about-content {
  max-width: 400px;
}

.about-logo {
  font-size: 64px;
  margin-bottom: 16px;
}

.about-content h1 {
  font-size: 32px;
  margin-bottom: 8px;
  color: var(--n-text-color);
}

.about-content .version {
  color: var(--n-text-color-3);
  margin-bottom: 24px;
}

.about-content .description {
  font-size: 15px;
  color: var(--n-text-color-2);
  margin-bottom: 24px;
}

.about-content .links {
  text-align: left;
  padding: 16px 24px;
  background: var(--n-color-modal);
  border-radius: 8px;
  margin-bottom: 24px;
}

.about-content .links p {
  padding: 4px 0;
  color: var(--n-text-color-2);
}

.about-content .copyright {
  font-size: 12px;
  color: var(--n-text-color-3);
}
</style>
