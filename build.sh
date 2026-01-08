#!/bin/bash

# fnm-client 打包脚本
# 用法: ./build.sh [platform]
# platform: macos | windows | linux | all (默认为当前平台)

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║       fnm-client 打包脚本              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# 检查依赖
check_dependencies() {
    echo -e "${YELLOW}[1/4] 检查依赖...${NC}"
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        echo -e "${RED}❌ 未找到 Node.js，请先安装${NC}"
        exit 1
    fi
    echo -e "  ✅ Node.js $(node -v)"
    
    # 检查 pnpm
    if ! command -v pnpm &> /dev/null; then
        echo -e "${RED}❌ 未找到 pnpm，请先安装: npm install -g pnpm${NC}"
        exit 1
    fi
    echo -e "  ✅ pnpm $(pnpm -v)"
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ 未找到 Rust/Cargo，请先安装: https://rustup.rs${NC}"
        exit 1
    fi
    echo -e "  ✅ Cargo $(cargo --version | cut -d' ' -f2)"
    
    echo ""
}

# 安装依赖
install_dependencies() {
    echo -e "${YELLOW}[2/4] 安装依赖...${NC}"
    pnpm install
    echo ""
}

# 构建前端
build_frontend() {
    echo -e "${YELLOW}[3/4] 构建前端...${NC}"
    pnpm exec vue-tsc --noEmit
    echo -e "  ✅ TypeScript 检查通过"
    echo ""
}

# 构建 Tauri 应用
build_tauri() {
    echo -e "${YELLOW}[4/4] 构建 Tauri 应用...${NC}"
    
    PLATFORM=${1:-""}
    
    case "$PLATFORM" in
        macos|darwin)
            echo -e "  📦 目标平台: macOS"
            pnpm tauri build --target universal-apple-darwin
            ;;
        windows|win)
            echo -e "  📦 目标平台: Windows"
            pnpm tauri build --target x86_64-pc-windows-msvc
            ;;
        linux)
            echo -e "  📦 目标平台: Linux"
            pnpm tauri build --target x86_64-unknown-linux-gnu
            ;;
        all)
            echo -e "  📦 目标平台: 全部"
            echo -e "${YELLOW}  ⚠️ 交叉编译需要额外配置，建议在各平台分别构建${NC}"
            pnpm tauri build
            ;;
        *)
            echo -e "  📦 目标平台: 当前系统"
            pnpm tauri build
            ;;
    esac
    
    echo ""
}

# 显示构建结果
show_result() {
    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║           ✅ 构建完成！                ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}输出目录:${NC}"
    
    BUNDLE_DIR="$PROJECT_DIR/src-tauri/target/release/bundle"
    
    if [ -d "$BUNDLE_DIR" ]; then
        # macOS
        if [ -d "$BUNDLE_DIR/dmg" ]; then
            echo -e "  📁 DMG: $BUNDLE_DIR/dmg/"
            ls -la "$BUNDLE_DIR/dmg/" 2>/dev/null | grep -E "\.dmg$" | awk '{print "     └── " $NF}'
        fi
        
        if [ -d "$BUNDLE_DIR/macos" ]; then
            echo -e "  📁 App: $BUNDLE_DIR/macos/"
        fi
        
        # Windows
        if [ -d "$BUNDLE_DIR/msi" ]; then
            echo -e "  📁 MSI: $BUNDLE_DIR/msi/"
        fi
        
        if [ -d "$BUNDLE_DIR/nsis" ]; then
            echo -e "  📁 NSIS: $BUNDLE_DIR/nsis/"
        fi
        
        # Linux
        if [ -d "$BUNDLE_DIR/deb" ]; then
            echo -e "  📁 DEB: $BUNDLE_DIR/deb/"
        fi
        
        if [ -d "$BUNDLE_DIR/appimage" ]; then
            echo -e "  📁 AppImage: $BUNDLE_DIR/appimage/"
        fi
    else
        echo -e "  📁 $BUNDLE_DIR"
    fi
    
    echo ""
}

# 主流程
main() {
    check_dependencies
    install_dependencies
    build_frontend
    build_tauri "$1"
    show_result
}

# 显示帮助
if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    echo "用法: ./build.sh [platform]"
    echo ""
    echo "参数:"
    echo "  macos    - 构建 macOS 版本"
    echo "  windows  - 构建 Windows 版本"  
    echo "  linux    - 构建 Linux 版本"
    echo "  all      - 构建所有平台（需要交叉编译支持）"
    echo "  (空)     - 构建当前平台版本"
    echo ""
    echo "示例:"
    echo "  ./build.sh         # 构建当前平台"
    echo "  ./build.sh macos   # 构建 macOS 版本"
    exit 0
fi

main "$1"
