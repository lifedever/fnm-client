// Node 版本信息
export interface NodeVersion {
  name: string           // 版本号，如 v22.21.1
  isInstalled: boolean   // 是否已安装
  isDefault: boolean     // 是否为默认版本
  isCurrent: boolean     // 是否为当前使用版本
  isLts: boolean         // 是否为 LTS 版本
  ltsName?: string       // LTS 名称，如 "Jod"
  aliases: string[]      // 别名列表
}

// fnm 环境变量配置
export interface FnmEnv {
  fnmDir: string                    // fnm 安装目录
  nodeDistMirror: string            // Node 下载镜像
  versionFileStrategy: 'local' | 'recursive'  // 版本文件策略
  corepackEnabled: boolean          // 是否启用 Corepack
  resolveEngines: boolean           // 是否解析 engines 字段
  arch: string                      // 架构 (arm64/x64)
  loglevel: string                  // 日志级别
}

// 版本筛选选项
export interface VersionFilter {
  lts?: boolean          // 只显示 LTS 版本
  installed?: boolean    // 只显示已安装版本
  keyword?: string       // 关键词搜索
}

// 操作结果
export interface OperationResult {
  success: boolean
  message: string
  data?: unknown
}

// 安装进度事件
export interface InstallProgress {
  version: string
  status: 'downloading' | 'extracting' | 'installing' | 'completed' | 'failed'
  progress?: number
  message?: string
}
