import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FnmEnv } from '@/types/fnm'

export const useSettingsStore = defineStore('settings', () => {
  // 状态
  const fnmDir = ref('')
  const nodeDistMirror = ref('https://nodejs.org/dist')
  const versionFileStrategy = ref<'local' | 'recursive'>('local')
  const corepackEnabled = ref(false)
  const resolveEngines = ref(true)
  const arch = ref('')
  const loglevel = ref('info')
  const loading = ref(false)
  const error = ref<string | null>(null)

  // 常用镜像源列表
  const mirrorOptions = [
    { label: '官方源', value: 'https://nodejs.org/dist' },
    { label: '淘宝镜像', value: 'https://npmmirror.com/mirrors/node' },
    { label: '腾讯镜像', value: 'https://mirrors.cloud.tencent.com/nodejs-release' },
    { label: '华为镜像', value: 'https://mirrors.huaweicloud.com/nodejs' }
  ]

  // 加载设置
  async function loadSettings() {
    loading.value = true
    error.value = null

    try {
      const env = await invoke<FnmEnv>('get_fnm_env')

      fnmDir.value = env.fnmDir
      nodeDistMirror.value = env.nodeDistMirror
      versionFileStrategy.value = env.versionFileStrategy as 'local' | 'recursive'
      corepackEnabled.value = env.corepackEnabled
      resolveEngines.value = env.resolveEngines
      arch.value = env.arch
      loglevel.value = env.loglevel
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to load settings:', e)
    } finally {
      loading.value = false
    }
  }

  // 获取 fnm 目录路径
  async function getFnmDir(): Promise<string> {
    try {
      return await invoke<string>('get_fnm_dir')
    } catch (e) {
      console.error('Failed to get fnm dir:', e)
      return ''
    }
  }

  // 打开 fnm 目录
  async function openFnmDirectory(): Promise<boolean> {
    try {
      await invoke('open_fnm_directory')
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to open fnm directory:', e)
      return false
    }
  }

  return {
    // 状态
    fnmDir,
    nodeDistMirror,
    versionFileStrategy,
    corepackEnabled,
    resolveEngines,
    arch,
    loglevel,
    loading,
    error,

    // 常量
    mirrorOptions,

    // 方法
    loadSettings,
    getFnmDir,
    openFnmDirectory
  }
})
