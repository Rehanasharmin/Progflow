#!/bin/bash
#
# progflow Installation Script
# A context-aware workspace manager for Linux (and Termux/macOS)
#
# This script installs progflow: it will use an existing binary or clone and build from source.
# Improved features:
# - Better platform detection (Termux, macOS, WSL)
# - Automatically installs git if missing
# - Automatically refreshes the shell RC file after adding to PATH
#

set -eo pipefail

# Configuration
PROGRAM_NAME="progflow"
REPO_URL="https://github.com/Rehanasharmin/Progflow.git"
INSTALL_DIRS=("/usr/local/bin" "$HOME/.local/bin")
CONFIG_DIR="$HOME/.config/flow"
REQUIRED_RUST_VERSION="1.70"
FORCE_INSTALL=false

# ------------------------- Argument Parsing -------------------------
for arg in "$@"; do
    case $arg in
        --force)
            FORCE_INSTALL=true
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
print_info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error()   { echo -e "${RED}[ERROR]${NC} $1"; }

# ------------------------- Enhanced Platform Detection -------------------------
is_termux() {
    # Termux sets a specific PREFIX and provides termux-open-url
    if [ -n "${PREFIX:-}" ] && [[ "$PREFIX" == */data/data/com.termux/* ]]; then
        return 0
    fi
    if command -v termux-open-url &>/dev/null; then
        return 0
    fi
    # Check for $TERMUX_VERSION (newer versions)
    if [ -n "${TERMUX_VERSION:-}" ]; then
        return 0
    fi
    return 1
}

is_macos() {
    [[ "$(uname -s)" == "Darwin" ]]
}

is_wsl() {
    # Check for Microsoft indication in /proc/version or /proc/sys/kernel/osrelease
    if [ -f /proc/version ] && grep -qi microsoft /proc/version 2>/dev/null; then
        return 0
    fi
    if [ -f /proc/sys/kernel/osrelease ] && grep -qi microsoft /proc/sys/kernel/osrelease 2>/dev/null; then
        return 0
    fi
    return 1
}

detect_shell() {
    if [ -n "${ZSH_VERSION:-}" ]; then
        echo "zsh"
    elif [ -n "${BASH_VERSION:-}" ]; then
        echo "bash"
    elif [ -n "${SH_VERSION:-}" ]; then
        echo "sh"
    else
        # Fallback: check SHELL environment variable
        if [[ "$SHELL" == *zsh* ]]; then
            echo "zsh"
        else
            echo "bash"
        fi
    fi
}

is_root() {
    [ "$(id -u)" -eq 0 ]
}

command_exists() {
    command -v "$1" &>/dev/null
}

get_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64)   echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        armv7l|armhf)  echo "armv7" ;;
        i386|i686)     echo "i686" ;;
        *)             echo "$arch" ;;
    esac
}

get_os() {
    local os
    os=$(uname -s)
    echo "${os,,}"
}

# ------------------------- Package Manager Detection -------------------------
# Returns a command prefix that can be used to install packages (with -y option)
# e.g. "apt-get install -y" or "pkg install -y"
get_installer_cmd() {
    if is_termux; then
        echo "pkg install -y"
        return
    fi

    if is_macos; then
        # Homebrew on macOS
        if command_exists brew; then
            echo "brew install"
            return
        fi
        # MacPorts as fallback
        if command_exists port; then
            echo "port install"
            return
        fi
        echo ""  # none found
        return
    fi

    # Linux distributions
    if command_exists apt-get; then
        if is_root; then
            echo "apt-get install -y"
        else
            echo "sudo apt-get install -y"
        fi
        return
    fi

    if command_exists dnf; then
        if is_root; then
            echo "dnf install -y"
        else
            echo "sudo dnf install -y"
        fi
        return
    fi

    if command_exists yum; then
        if is_root; then
            echo "yum install -y"
        else
            echo "sudo yum install -y"
        fi
        return
    fi

    if command_exists pacman; then
        if is_root; then
            echo "pacman -S --noconfirm"
        else
            echo "sudo pacman -S --noconfirm"
        fi
        return
    fi

    if command_exists zypper; then
        if is_root; then
            echo "zypper install -y"
        else
            echo "sudo zypper install -y"
        fi
        return
    fi

    if command_exists apk; then
        if is_root; then
            echo "apk add"
        else
            echo "sudo apk add"
        fi
        return
    fi

    # No recognized package manager
    echo ""
}

# ------------------------- Git Auto-Install -------------------------
ensure_git() {
    if command_exists git; then
        return 0
    fi

    print_warning "Git not found. Attempting to install automatically..."
    local installer
    installer=$(get_installer_cmd)

    if [ -z "$installer" ]; then
        print_error "No supported package manager found. Please install git manually."
        return 1
    fi

    print_info "Installing git using: $installer git"
    if $installer git; then
        print_success "Git installed successfully."
        return 0
    else
        print_error "Failed to install git. Please install it manually."
        return 1
    fi
}

# ------------------------- Rust Installation -------------------------
check_rust() {
    if command_exists rustc; then
        local version
        version=$(rustc --version | awk '{print $2}')
        print_info "Rust found: $version"
        return 0
    fi
    return 1
}

install_rust() {
    print_info "Installing Rust..."

    # Termux specific
    if is_termux; then
        pkg update -y
        pkg install -y rust
        return 0
    fi

    # macOS: prefer rustup or brew
    if is_macos; then
        if command_exists brew; then
            print_info "Installing Rust via Homebrew..."
            brew install rust && return 0
        fi
        # Fallback to rustup
        print_info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
        return $?
    fi

    # Linux: use system package manager if available
    local installer
    installer=$(get_installer_cmd)
    if [ -n "$installer" ]; then
        if $installer rust; then
            return 0
        fi
    fi

    # Fallback: rustup
    print_info "Trying rustup..."
    if command_exists curl; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
        return $?
    fi

    print_error "Cannot install Rust. Install it manually."
    return 1
}

# ------------------------- Build Dependencies -------------------------
check_build_deps() {
    print_info "Checking build dependencies..."

    # Termux has build-essentials via build-essential + clang
    if is_termux; then
        pkg update -y
        pkg install -y build-essential clang
        return 0
    fi

    local missing=()
    if ! command_exists gcc && ! command_exists cc; then
        missing+=(gcc)
    fi
    if ! command_exists make; then
        missing+=(make)
    fi

    if [ ${#missing[@]} -eq 0 ]; then
        print_info "Build dependencies satisfied"
        return 0
    fi

    print_warning "Missing: ${missing[*]}"
    local installer
    installer=$(get_installer_cmd)
    if [ -n "$installer" ]; then
        if $installer "${missing[@]}"; then
            return 0
        fi
    fi

    print_warning "Could not install build dependencies automatically."
    return 1
}

# ------------------------- Installation Directory -------------------------
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

    print_error "No writable installation directory found."
    return 1
}

# ------------------------- PATH and RC Refresh -------------------------
add_to_path() {
    local install_dir="$1"
    local shell=$(detect_shell)
    local shellrc

    # Determine the appropriate RC file
    case "$shell" in
        bash)
            shellrc="$HOME/.bashrc"
            # Sometimes bash uses .bash_profile or .profile, but .bashrc is standard
            ;;
        zsh)
            shellrc="$HOME/.zshrc"
            ;;
        *)
            shellrc="$HOME/.profile"
            ;;
    esac

    # If already in PATH, nothing to do
    if [[ ":$PATH:" == *":$install_dir:"* ]]; then
        print_info "Already in PATH: $install_dir"
        return 0
    fi

    local path_line="export PATH=\"$install_dir:\$PATH\""

    if [ -f "$shellrc" ]; then
        if ! grep -qF "$install_dir" "$shellrc" 2>/dev/null; then
            echo "" >> "$shellrc"
            echo "# Added by progflow" >> "$shellrc"
            echo "$path_line" >> "$shellrc"
            print_info "Added $install_dir to $shellrc"
        else
            print_info "$shellrc already contains $install_dir"
        fi
    else
        echo "# Added by progflow" > "$shellrc"
        echo "$path_line" >> "$shellrc"
        print_info "Created $shellrc with PATH entry"
    fi

    # Update current session
    export PATH="$install_dir:$PATH"
    print_success "$install_dir added to PATH for this session"

    # Refresh the RC file (source it) for the current shell, suppress errors
    print_info "Refreshing shell environment from $shellrc..."
    # Temporarily disable -e to avoid exiting on source errors
    set +e
    # The '|| true' ensures we never fail because of set -o pipefail
    source "$shellrc" 2>/dev/null || true
    set -e
    print_info "Shell environment refreshed (if source succeeded)."
}

# ------------------------- Build from Source -------------------------
build_from_source() {
    local build_dir="$1"
    local install_dir="$2"

    print_info "Building progflow from source..."
    [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" 2>/dev/null || true

    cd "$build_dir"
    if ! cargo build --release 2>&1; then
        print_error "Build failed"
        return 1
    fi

    if [ ! -f "target/release/$PROGRAM_NAME" ]; then
        print_error "Binary not found after build"
        return 1
    fi

    cp "target/release/$PROGRAM_NAME" "$install_dir/"
    chmod +x "$install_dir/$PROGRAM_NAME"

    print_success "Binary built and installed to $install_dir/$PROGRAM_NAME"
    return 0
}

# ------------------------- Repository Cloning -------------------------
clone_repo() {
    local clone_dir="$1"

    print_info "Cloning repository..."
    ensure_git || return 1   # Auto-install git if missing

    if ! git clone --depth 1 "$REPO_URL" "$clone_dir"; then
        print_error "Failed to clone repository"
        return 1
    fi
    print_success "Repository cloned"
    return 0
}

# ------------------------- Existing Binary Search -------------------------
find_existing_binary() {
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            echo "$dir/$PROGRAM_NAME"
            return 0
        fi
    done
    local found
    found=$(command_exists "$PROGRAM_NAME" && which "$PROGRAM_NAME") || true
    if [ -n "$found" ]; then
        echo "$found"
        return 0
    fi
    return 1
}

# ------------------------- Version/Update -------------------------
get_remote_hash() {
    git ls-remote "$REPO_URL" HEAD | awk '{print $1}'
}

get_local_version_info() {
    local binary
    binary=$(find_existing_binary) || return 1
    # We can't easily get the hash from the binary, so we'll check if the 
    # current directory is the repo and matches the binary.
    # For now, we will rely on the binary's version if possible, 
    # but since version is static, we'll force update if requested via 'install' 
    # and only be 'smart' in the 'update' command.
    echo "unknown"
}

# ------------------------- Install/Uninstall -------------------------
install() {
    local is_update_cmd="${1:-false}"
    
    print_info "Starting $PROGRAM_NAME installation..."
    echo -e "${YELLOW}  [!] This might take a few minutes, maybe longer than you'd expect.${NC}"
    echo -e "${YELLOW}      Building from source takes some time... stick with us.${NC}"
    
    if [ "$is_update_cmd" = "true" ]; then
        print_info "Checking for updates..."
        ensure_git || exit 1
        local remote_hash
        remote_hash=$(get_remote_hash)
        
        # In a real scenario, we might store the installed hash in a file.
        # Since we don't have that yet, the smart check is limited to 
        # local git context if available.
        if [ -d ".git" ]; then
            local local_hash
            local_hash=$(git rev-parse HEAD)
            if [ "$local_hash" = "$remote_hash" ] && [ "$FORCE_INSTALL" = false ]; then
                print_success "$PROGRAM_NAME is already at the latest version ($local_hash)."
                exit 0
            fi
        fi
    fi

    print_info "Detected OS: $(get_os)-$(get_arch)"
    if is_termux; then print_info "Environment: Termux"; fi
    if is_wsl;   then print_info "Environment: WSL"; fi
    if is_macos; then print_info "Environment: macOS"; fi

    # Existing installation?
    local existing_binary
    existing_binary=$(find_existing_binary) || true
    if [ -n "$existing_binary" ] && [ "$FORCE_INSTALL" = false ]; then
        print_warning "$PROGRAM_NAME is already installed at $existing_binary"
        read -p "Reinstall? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Installation cancelled"
            exit 0
        fi
    fi

    # Rust
    if ! check_rust; then
        print_warning "Rust not found. Installing..."
        install_rust || { print_error "Rust installation failed"; exit 1; }
    fi

    [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" 2>/dev/null || true

    # Build deps (non-critical)
    check_build_deps || true

    # Installation directory
    local install_dir
    install_dir=$(find_install_dir) || { print_error "No install directory"; exit 1; }
    print_info "Installing to: $install_dir"

    # Build source
    local build_dir
    if [ -f "./Cargo.toml" ] && [ -d "./src" ]; then
        print_info "Using local source code"
        build_dir="$(pwd)"
    else
        build_dir=$(mktemp -d)
        trap "rm -rf $build_dir" EXIT
        clone_repo "$build_dir" || { print_error "Clone failed"; exit 1; }
    fi

    build_from_source "$build_dir" "$install_dir" || { print_error "Build failed"; exit 1; }

    # PATH and RC refresh
    add_to_path "$install_dir"

    # Config directory
    mkdir -p "$CONFIG_DIR"

    # Verification
    print_info "Verifying installation..."
    if "$install_dir/$PROGRAM_NAME" --version &>/dev/null || "$install_dir/$PROGRAM_NAME" --help &>/dev/null; then
        print_success "$PROGRAM_NAME installed successfully!"
        print_info "Run '$PROGRAM_NAME --help' to get started"
    else
        print_warning "Verification failed; please check manually."
    fi

    # Optional Cleanup
    echo -e "\n${YELLOW}------------------------------------------------------------${NC}"
    echo -e "${YELLOW}           OPTIONAL: STORAGE OPTIMIZATION${NC}"
    echo -e "${YELLOW}------------------------------------------------------------${NC}"
    echo -e "Building $PROGRAM_NAME requires Rust and other dependencies"
    echo -e "which can occupy significant disk space (up to 1GB+)."
    echo -e "\nIf you want to save space, you can uninstall them now."
    echo -e "${RED}CAUTION: Removing Rust will break 'progflow update'.${NC}"
    echo -e "You will need to reinstall Rust manually to update later."
    echo -e "${YELLOW}------------------------------------------------------------${NC}"

    read -p "Uninstall build dependencies (Rust, etc.)? [y/N]: " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_info "Attempting to remove build dependencies..."
        if is_termux; then
            pkg uninstall -y rust build-essential clang || true
        elif is_macos; then
            if command_exists brew; then
                brew uninstall rust || true
            fi
        else
            local installer
            installer=$(get_installer_cmd)
            if [ -n "$installer" ]; then
                # Try to replace 'install' with 'remove' or 'uninstall'
                local remove_cmd
                remove_cmd=$(echo "$installer" | sed 's/install/remove/g' | sed 's/apt-get/apt-get purge/g' | sed 's/-S --noconfirm/-Rs --noconfirm/g')
                $remove_cmd rust || true
            fi
        fi

        # Also check for rustup
        if command_exists rustup; then
            rustup self uninstall -y || true
        fi

        print_success "Cleanup complete. Storage optimized."
    else
        print_info "Keeping build dependencies for future updates."
    fi
}

uninstall() {
    print_info "Uninstalling $PROGRAM_NAME..."
    local removed=false
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            rm -f "$dir/$PROGRAM_NAME"
            print_info "Removed $dir/$PROGRAM_NAME"
            removed=true
        fi
    done
    if [ -d "$CONFIG_DIR" ]; then
        print_warning "Config directory still exists: $CONFIG_DIR"
        read -p "Remove it? [y/N]: " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_info "Removed config directory"
        fi
        removed=true
    fi
    if $removed; then
        print_success "$PROGRAM_NAME uninstalled"
    else
        print_warning "$PROGRAM_NAME not found"
    fi
}

show_version() {
    local binary
    binary=$(find_existing_binary) || true
    if [ -n "$binary" ]; then
        $binary --version 2>/dev/null || $binary --help | head -1
    else
        echo "$PROGRAM_NAME is not installed"
    fi
}

# ------------------------- Main -------------------------
main() {
    case "${1:-install}" in
        install)
            install
            ;;
        update)
            FORCE_INSTALL=true
            install true
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
