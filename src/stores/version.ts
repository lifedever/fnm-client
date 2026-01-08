import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { NodeVersion, VersionFilter } from '@/types/fnm'
import { parseInstalledVersions, parseRemoteVersions, filterVersions, compareVersions } from '@/utils/version-parser'

export const useVersionStore = defineStore('version', () => {
  // 状态
  const installedVersions = ref<NodeVersion[]>([])
  const remoteVersions = ref<NodeVersion[]>([])
  const currentVersion = ref<string>('')
  const loading = ref(false)
  const remoteLoading = ref(false)
  const error = ref<string | null>(null)

  // 计算属性
  const sortedInstalledVersions = computed(() => {
    return [...installedVersions.value].sort((a, b) => compareVersions(a.name, b.name))
  })

  const sortedRemoteVersions = computed(() => {
    return [...remoteVersions.value].sort((a, b) => compareVersions(a.name, b.name))
  })

  const defaultVersion = computed(() => {
    return installedVersions.value.find(v => v.isDefault)
  })

  // 获取已安装版本
  async function fetchInstalledVersions() {
    loading.value = true
    error.value = null

    try {
      const [listResult, current] = await Promise.all([
        invoke<string>('list_installed_versions'),
        invoke<string>('get_current_version')
      ])

      currentVersion.value = current.trim()
      installedVersions.value = parseInstalledVersions(listResult, currentVersion.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to fetch installed versions:', e)
    } finally {
      loading.value = false
    }
  }

  // 获取远程版本
  async function fetchRemoteVersions(filter?: VersionFilter) {
    remoteLoading.value = true
    error.value = null

    try {
      const result = await invoke<string>('list_remote_versions', {
        ltsOnly: filter?.lts ?? false
      })

      const installedNames = installedVersions.value.map(v => v.name)
      remoteVersions.value = parseRemoteVersions(result, installedNames)
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to fetch remote versions:', e)
    } finally {
      remoteLoading.value = false
    }
  }

  // 安装版本
  async function installVersion(version: string): Promise<boolean> {
    loading.value = true
    error.value = null

    try {
      await invoke('install_version', { version })
      await fetchInstalledVersions()
      // 更新远程版本的安装状态
      const installedNames = installedVersions.value.map(v => v.name)
      remoteVersions.value = remoteVersions.value.map(v => ({
        ...v,
        isInstalled: installedNames.includes(v.name)
      }))
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to install version:', e)
      return false
    } finally {
      loading.value = false
    }
  }

  // 卸载版本
  async function uninstallVersion(version: string): Promise<boolean> {
    loading.value = true
    error.value = null

    try {
      await invoke('uninstall_version', { version })
      await fetchInstalledVersions()
      // 更新远程版本的安装状态
      const installedNames = installedVersions.value.map(v => v.name)
      remoteVersions.value = remoteVersions.value.map(v => ({
        ...v,
        isInstalled: installedNames.includes(v.name)
      }))
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to uninstall version:', e)
      return false
    } finally {
      loading.value = false
    }
  }

  // 切换版本
  async function useVersion(version: string): Promise<boolean> {
    error.value = null

    try {
      await invoke('use_version', { version })
      currentVersion.value = version
      // 更新 isCurrent 状态
      installedVersions.value = installedVersions.value.map(v => ({
        ...v,
        isCurrent: v.name === version
      }))
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to use version:', e)
      return false
    }
  }

  // 设置默认版本
  async function setDefaultVersion(version: string): Promise<boolean> {
    error.value = null

    try {
      await invoke('set_default_version', { version })
      // 更新 isDefault 状态
      installedVersions.value = installedVersions.value.map(v => ({
        ...v,
        isDefault: v.name === version
      }))
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to set default version:', e)
      return false
    }
  }

  // 打开版本目录
  async function openVersionDirectory(version: string): Promise<boolean> {
    error.value = null

    try {
      await invoke('open_version_directory', { version })
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to open version directory:', e)
      return false
    }
  }

  // 筛选版本
  function getFilteredVersions(
    source: 'installed' | 'remote',
    options: { ltsOnly?: boolean; keyword?: string }
  ): NodeVersion[] {
    const versions = source === 'installed' ? sortedInstalledVersions.value : sortedRemoteVersions.value
    return filterVersions(versions, {
      ltsOnly: options.ltsOnly,
      installedOnly: source === 'installed',
      keyword: options.keyword
    })
  }

  return {
    // 状态
    installedVersions,
    remoteVersions,
    currentVersion,
    loading,
    remoteLoading,
    error,

    // 计算属性
    sortedInstalledVersions,
    sortedRemoteVersions,
    defaultVersion,

    // 方法
    fetchInstalledVersions,
    fetchRemoteVersions,
    installVersion,
    uninstallVersion,
    useVersion,
    setDefaultVersion,
    openVersionDirectory,
    getFilteredVersions
  }
})
