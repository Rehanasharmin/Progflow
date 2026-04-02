#!/bin/bash
#
# progflow Installation Script
# A context-aware workspace manager for Linux
#
# This script installs progflow on Linux systems (including Termux)
# It will either use an existing binary or clone and build from source
#

set -eo pipefail

# Configuration
PROGRAM_NAME="progflow"
REPO_URL="https://github.com/Rehanasharmin/Progflow.git"
INSTALL_DIRS=("/usr/local/bin" "$HOME/.local/bin")
CONFIG_DIR="$HOME/.config/flow"
REQUIRED_RUST_VERSION="1.70"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running in Termux
is_termux() {
    if [ -n "${PREFIX:-}" ] && [[ "$PREFIX" == */data/data/com.termux/* ]]; then
        return 0
    fi
    if command -v termux-open-url &>/dev/null; then
        return 0
    fi
    return 1
}

# Check if running as root
is_root() {
    [ "$(id -u)" -eq 0 ]
}

# Detect the shell
detect_shell() {
    if [ -n "${ZSH_VERSION:-}" ]; then
        echo "zsh"
    elif [ -n "${BASH_VERSION:-}" ]; then
        echo "bash"
    elif [ -n "${SH_VERSION:-}" ]; then
        echo "sh"
    else
        echo "bash"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" &>/dev/null
}

# Get the architecture
get_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        armv7l|armhf)
            echo "armv7"
            ;;
        i386|i686)
            echo "i686"
            ;;
        *)
            echo "$arch"
            ;;
    esac
}

# Get the OS
get_os() {
    local os
    os=$(uname -s)
    echo "${os,,}"
}

# Check Rust installation
check_rust() {
    if command_exists rustc; then
        local version
        version=$(rustc --version | awk '{print $2}')
        print_info "Rust found: $version"
        return 0
    fi
    return 1
}

# Install Rust
install_rust() {
    print_info "Installing Rust..."
    
    if is_termux; then
        pkg update -y
        pkg install -y rust
        return 0
    fi
    
    if command_exists apt-get; then
        if ! is_root; then
            print_warning "Rust installation requires root. Trying sudo..."
            if ! sudo apt-get update && sudo apt-get install -y rustc; then
                print_error "Failed to install Rust. Please install manually."
                return 1
            fi
        else
            apt-get update
            apt-get install -y rustc
        fi
        return 0
    fi
    
    if command_exists yum; then
        if ! is_root; then
            print_warning "Rust installation requires root. Trying sudo..."
            if ! sudo yum install -y rust; then
                print_error "Failed to install Rust. Please install manually."
                return 1
            fi
        else
            yum install -y rust
        fi
        return 0
    fi
    
    if command_exists dnf; then
        if ! is_root; then
            print_warning "Rust installation requires root. Trying sudo..."
            if ! sudo dnf install -y rust; then
                print_error "Failed to install Rust. Please install manually."
                return 1
            fi
        else
            dnf install -y rust
        fi
        return 0
    fi
    
    if command_exists pacman; then
        if ! is_root; then
            print_warning "Installing requires root. Trying sudo..."
            if ! sudo pacman -S --noconfirm rust; then
                print_error "Failed to install Rust. Please install manually."
                return 1
            fi
        else
            pacman -S --noconfirm rust
        fi
        return 0
    fi
    
    # Try rustup
    print_info "Attempting to install Rust via rustup..."
    if command_exists curl && command_exists sh; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
        return 0
    fi
    
    print_error "Could not install Rust automatically. Please install manually."
    return 1
}

# Check for build dependencies
check_build_deps() {
    print_info "Checking build dependencies..."
    
    if is_termux; then
        pkg update -y
        pkg install -y build-essential clang
        return 0
    fi
    
    local missing_deps=()
    
    if ! command_exists gcc && ! command_exists cc; then
        missing_deps+=(gcc)
    fi
    
    if ! command_exists make; then
        missing_deps+=(make)
    fi
    
    if [ ${#missing_deps[@]} -eq 0 ]; then
        print_info "Build dependencies satisfied"
        return 0
    fi
    
    print_warning "Missing dependencies: ${missing_deps[*]}"
    
    if command_exists apt-get; then
        if is_root; then
            apt-get update && apt-get install -y "${missing_deps[@]}"
        else
            sudo apt-get update && sudo apt-get install -y "${missing_deps[@]}"
        fi
        return 0
    fi
    
    if command_exists yum; then
        if is_root; then
            yum install -y "${missing_deps[@]}"
        else
            sudo yum install -y "${missing_deps[@]}"
        fi
        return 0
    fi
    
    if command_exists dnf; then
        if is_root; then
            dnf install -y "${missing_deps[@]}"
        else
            sudo dnf install -y "${missing_deps[@]}"
        fi
        return 0
    fi
    
    print_warning "Could not install build dependencies automatically"
    return 1
}

# Find the install directory
find_install_dir() {
    if is_root; then
        echo "/usr/local/bin"
        return 0
    fi
    
    if [ -d "$HOME/.local/bin" ] && [ -w "$HOME/.local/bin" ]; then
        echo "$HOME/.local/bin"
        return 0
    fi
    
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -d "$dir" ] && [ -w "$dir" ]; then
            echo "$dir"
            return 0
        fi
    done
    
    # Try to create ~/.local/bin
    if [ -w "$HOME" ]; then
        mkdir -p "$HOME/.local/bin"
        echo "$HOME/.local/bin"
        return 0
    fi
    
    print_error "No writable installation directory found"
    return 1
}

# Add to PATH if needed
add_to_path() {
    local install_dir="$1"
    local shellrc=""
    local shell=$(detect_shell)
    
    case "$shell" in
        bash)
            shellrc="$HOME/.bashrc"
            ;;
        zsh)
            shellrc="$HOME/.zshrc"
            ;;
        *)
            shellrc="$HOME/.profile"
            ;;
    esac
    
    # Check if already in PATH
    if [[ ":$PATH:" == *":$install_dir:"* ]]; then
        print_info "Already in PATH: $install_dir"
        return 0
    fi
    
    print_info "Adding $install_dir to PATH in $shellrc"
    
    local path_line="export PATH=\"$install_dir:\$PATH\""
    
    if [ -f "$shellrc" ]; then
        if ! grep -qF "$install_dir" "$shellrc" 2>/dev/null; then
            echo "" >> "$shellrc"
            echo "# Added by progflow" >> "$shellrc"
            echo "$path_line" >> "$shellrc"
            print_info "Added to $shellrc"
        fi
    else
        echo "# Added by progflow" > "$shellrc"
        echo "$path_line" >> "$shellrc"
        print_info "Created $shellrc with PATH entry"
    fi
    
    # Also add to current session
    export PATH="$install_dir:$PATH"
    print_success "$install_dir added to PATH for this session"
}

# Build from source
build_from_source() {
    local build_dir="$1"
    local install_dir="$2"
    
    print_info "Building progflow from source..."
    
    # Source rust environment if available
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi
    
    cd "$build_dir"
    
    # Build release
    if ! cargo build --release 2>&1; then
        print_error "Build failed"
        return 1
    fi
    
    # Check if binary exists
    if [ ! -f "target/release/$PROGRAM_NAME" ]; then
        print_error "Binary not found after build"
        return 1
    fi
    
    # Copy binary
    cp "target/release/$PROGRAM_NAME" "$install_dir/"
    chmod +x "$install_dir/$PROGRAM_NAME"
    
    print_success "Binary built and installed to $install_dir/$PROGRAM_NAME"
    return 0
}

# Download pre-built binary (future feature)
download_binary() {
    local install_dir="$1"
    local os="$2"
    local arch="$3"
    
    print_info "No pre-built binary available for $os-$arch"
    print_info "Will build from source instead"
    return 1
}

# Clone repository
clone_repo() {
    local clone_dir="$1"
    
    print_info "Cloning repository..."
    
    if command_exists git; then
        if ! git clone --depth 1 "$REPO_URL" "$clone_dir"; then
            print_error "Failed to clone repository"
            return 1
        fi
    else
        print_error "Git is not installed. Please install git first."
        return 1
    fi
    
    print_success "Repository cloned"
    return 0
}

# Detect if binary is already installed
find_existing_binary() {
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            echo "$dir/$PROGRAM_NAME"
            return 0
        fi
    done
    
    # Check in PATH
    local found
    found=$(command_exists "$PROGRAM_NAME" && which "$PROGRAM_NAME") || true
    if [ -n "$found" ]; then
        echo "$found"
        return 0
    fi
    
    return 1
}

# Check for updates
check_for_updates() {
    local current_binary="$1"
    
    if [ ! -d ".git" ]; then
        return 1
    fi
    
    if ! command_exists git; then
        return 1
    fi
    
    git fetch -q origin 2>/dev/null || return 1
    
    local local_hash
    local remote_hash
    
    local_hash=$(git rev-parse HEAD 2>/dev/null) || return 1
    remote_hash=$(git rev-parse origin/main 2>/dev/null) || remote_hash=$(git rev-parse origin/master 2>/dev/null) || return 1
    
    if [ "$local_hash" != "$remote_hash" ]; then
        return 0
    fi
    
    return 1
}

# Main installation function
install() {
    print_info "Starting $PROGRAM_NAME installation..."
    print_info "Detected: $(get_os)-$(get_arch)"
    
    if is_termux; then
        print_info "Running in Termux environment"
    fi
    
    # Check for existing binary
    local existing_binary
    existing_binary=$(find_existing_binary) || true
    
    if [ -n "$existing_binary" ]; then
        print_warning "$PROGRAM_NAME is already installed at $existing_binary"
        read -p "Reinstall? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Installation cancelled"
            exit 0
        fi
    fi
    
    # Check for Rust
    if ! check_rust; then
        print_warning "Rust not found. Installing..."
        if ! install_rust; then
            print_error "Failed to install Rust"
            exit 1
        fi
    fi
    
    # Source rust environment
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi
    
    # Check build dependencies
    check_build_deps || true
    
    # Find install directory
    local install_dir
    install_dir=$(find_install_dir) || {
        print_error "Could not find install directory"
        exit 1
    }
    
    print_info "Installing to: $install_dir"
    
    # Determine build source
    local build_dir=""
    local needs_build=false
    
    # Check if we have the source
    if [ -f "./Cargo.toml" ] && [ -d "./src" ]; then
        print_info "Using local source code"
        build_dir="$(pwd)"
    else
        # Need to clone
        build_dir=$(mktemp -d)
        trap "rm -rf $build_dir" EXIT
        
        if ! clone_repo "$build_dir"; then
            print_error "Failed to clone repository"
            exit 1
        fi
        needs_build=true
    fi
    
    # Build and install
    if ! build_from_source "$build_dir" "$install_dir"; then
        print_error "Installation failed"
        exit 1
    fi
    
    # Add to PATH
    add_to_path "$install_dir"
    
    # Initialize config directory
    print_info "Initializing config directory..."
    mkdir -p "$CONFIG_DIR"
    
    # Verify installation
    if "$install_dir/$PROGRAM_NAME" --version &>/dev/null; then
        print_success "$PROGRAM_NAME installed successfully!"
        print_info "Run '$PROGRAM_NAME --help' to get started"
    elif "$install_dir/$PROGRAM_NAME" --help &>/dev/null; then
        print_success "$PROGRAM_NAME installed successfully!"
        print_info "Run '$PROGRAM_NAME --help' to get started"
    else
        print_warning "Installation may have issues. Please verify manually."
    fi
    
    print_info "Installation complete!"
}

# Uninstall function
uninstall() {
    print_info "Starting $PROGRAM_NAME uninstallation..."
    
    local removed=false
    
    # Remove from install directories
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            rm -f "$dir/$PROGRAM_NAME"
            print_info "Removed $dir/$PROGRAM_NAME"
            removed=true
        fi
    done
    
    # Check for config directory
    if [ -d "$CONFIG_DIR" ]; then
        print_warning "Config directory still exists: $CONFIG_DIR"
        print_info "Remove config directory? [y/N]: "
        read -r
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_info "Removed config directory"
        fi
        removed=true
    fi
    
    if [ "$removed" = true ]; then
        print_success "$PROGRAM_NAME uninstalled successfully!"
    else
        print_warning "$PROGRAM_NAME not found"
    fi
}

# Show version
show_version() {
    local binary
    binary=$(find_existing_binary) || true
    
    if [ -n "$binary" ]; then
        echo "$binary"
        $binary --version 2>/dev/null || $binary --help | head -1
    else
        echo "$PROGRAM_NAME is not installed"
    fi
}

# Main
main() {
    case "${1:-install}" in
        install)
            install
            ;;
        uninstall|remove)
            uninstall
            ;;
        version)
            show_version
            ;;
        help|--help|-h)
            echo "Usage: $0 [COMMAND]"
            echo ""
            echo "Commands:"
            echo "  install     Install progflow (default)"
            echo "  uninstall   Remove progflow"
            echo "  version     Show version information"
            echo "  help        Show this help"
            ;;
        *)
            print_error "Unknown command: $1"
            echo "Use '$0 help' for usage information"
            exit 1
            ;;
    esac
}

main "$@"
