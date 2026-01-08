import type { NodeVersion } from '@/types/fnm'

/**
 * 解析 fnm list 命令的输出
 * 输出格式示例：
 * * v22.21.1 default
 * * v20.12.2 lts-latest
 * * v18.20.8
 * * system
 */
export function parseInstalledVersions(output: string, currentVersion: string): NodeVersion[] {
  const lines = output.split('\n').filter(line => line.trim())

  return lines
    .filter(line => !line.includes('system')) // 过滤掉 system
    .map(line => {
      const cleanLine = line.replace(/^\*\s*/, '').trim()
      const parts = cleanLine.split(/\s+/)
      const name = parts[0]
      const tags = parts.slice(1)

      const isDefault = tags.includes('default')
      const isLts = tags.some(tag => tag.toLowerCase().includes('lts'))
      const ltsName = tags.find(tag => tag.toLowerCase().includes('lts'))

      return {
        name,
        isInstalled: true,
        isDefault,
        isCurrent: name === currentVersion,
        isLts,
        ltsName,
        aliases: tags.filter(t => t !== 'default' && !t.toLowerCase().includes('lts'))
      }
    })
}

/**
 * 解析 fnm list-remote 命令的输出
 * 输出格式示例：
 * v22.21.1
 * v22.21.0
 * v20.12.2 (Jod)
 */
export function parseRemoteVersions(output: string, installedVersions: string[]): NodeVersion[] {
  const lines = output.split('\n').filter(line => line.trim())

  const results: NodeVersion[] = []

  for (const line of lines) {
    const cleanLine = line.trim()
    // 匹配版本号和可能的 LTS 名称，如 "v20.12.2 (Jod)"
    const match = cleanLine.match(/^(v[\d.]+)(?:\s+\(([^)]+)\))?/)

    if (match) {
      const name = match[1]
      const ltsName = match[2]

      results.push({
        name,
        isInstalled: installedVersions.includes(name),
        isDefault: false,
        isCurrent: false,
        isLts: !!ltsName,
        ltsName,
        aliases: []
      })
    }
  }

  return results
}

/**
 * 比较版本号，用于排序
 * 返回负数表示 a < b，正数表示 a > b，0 表示相等
 */
export function compareVersions(a: string, b: string): number {
  const parseVersion = (v: string) => {
    const match = v.match(/v?(\d+)\.(\d+)\.(\d+)/)
    if (!match) return [0, 0, 0]
    return [parseInt(match[1]), parseInt(match[2]), parseInt(match[3])]
  }

  const [aMajor, aMinor, aPatch] = parseVersion(a)
  const [bMajor, bMinor, bPatch] = parseVersion(b)

  if (aMajor !== bMajor) return bMajor - aMajor
  if (aMinor !== bMinor) return bMinor - aMinor
  return bPatch - aPatch
}

/**
 * 按主版本号分组版本
 */
export function groupVersionsByMajor(versions: NodeVersion[]): Map<number, NodeVersion[]> {
  const groups = new Map<number, NodeVersion[]>()

  versions.forEach(version => {
    const match = version.name.match(/v?(\d+)/)
    if (match) {
      const major = parseInt(match[1])
      if (!groups.has(major)) {
        groups.set(major, [])
      }
      groups.get(major)!.push(version)
    }
  })

  return groups
}

/**
 * 获取每个主版本的最新版本
 */
export function getLatestByMajor(versions: NodeVersion[]): NodeVersion[] {
  const groups = groupVersionsByMajor(versions)
  const result: NodeVersion[] = []

  // 按主版本号降序排列
  const sortedMajors = Array.from(groups.keys()).sort((a, b) => b - a)

  sortedMajors.forEach(major => {
    const majorVersions = groups.get(major)!
    // 每个主版本取最新的一个
    const sorted = majorVersions.sort((a, b) => compareVersions(a.name, b.name))
    if (sorted.length > 0) {
      result.push(sorted[0])
    }
  })

  return result
}

/**
 * 筛选版本
 */
export function filterVersions(
  versions: NodeVersion[],
  options: {
    ltsOnly?: boolean
    installedOnly?: boolean
    keyword?: string
  }
): NodeVersion[] {
  let result = [...versions]

  if (options.ltsOnly) {
    result = result.filter(v => v.isLts)
  }

  if (options.installedOnly) {
    result = result.filter(v => v.isInstalled)
  }

  if (options.keyword) {
    const keyword = options.keyword.toLowerCase()
    result = result.filter(v =>
      v.name.toLowerCase().includes(keyword) ||
      v.ltsName?.toLowerCase().includes(keyword) ||
      v.aliases.some(a => a.toLowerCase().includes(keyword))
    )
  }

  return result
}
