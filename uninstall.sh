#!/bin/bash
#
# progflow Uninstallation Script
# Removes progflow and optionally its configuration
#

set -eo pipefail

PROGRAM_NAME="progflow"
INSTALL_DIRS=("/usr/local/bin" "$HOME/.local/bin")
CONFIG_DIR="$HOME/.config/flow"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check if running in Termux
is_termux() {
    [ -n "${PREFIX:-}" ] && [[ "$PREFIX" == */data/data/com.termux/* ]]
}

# Find installed binary
find_binary() {
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            echo "$dir/$PROGRAM_NAME"
            return 0
        fi
    done
    return 1
}

uninstall() {
    print_info "Uninstalling $PROGRAM_NAME..."
    
    local removed=false
    local binary
    
    # Remove binary from all install directories
    for dir in "${INSTALL_DIRS[@]}"; do
        if [ -f "$dir/$PROGRAM_NAME" ]; then
            rm -f "$dir/$PROGRAM_NAME"
            print_info "Removed $dir/$PROGRAM_NAME"
            removed=true
        fi
    done
    
    # Also check /usr/bin (for system installs)
    if [ -f "/usr/bin/$PROGRAM_NAME" ]; then
        rm -f "/usr/bin/$PROGRAM_NAME"
        print_info "Removed /usr/bin/$PROGRAM_NAME"
        removed=true
    fi
    
    # Check for config directory
    if [ -d "$CONFIG_DIR" ]; then
        print_warning "Config directory found: $CONFIG_DIR"
        
        local config_count
        config_count=$(find "$CONFIG_DIR" -name "*.json" 2>/dev/null | wc -l)
        
        if [ "$config_count" -gt 0 ]; then
            print_info "Found $config_count flow configuration(s)"
        fi
        
        echo
        read -p "Remove config directory? [y/N]: " -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_info "Removed config directory: $CONFIG_DIR"
            removed=true
        fi
    fi
    
    # Check for lock files
    local lock_count
    lock_count=$(find "$HOME" -name "*.lock" -path "*flow*" 2>/dev/null | wc -l)
    
    if [ "$lock_count" -gt 0 ]; then
        print_warning "Found $lock_count lock file(s)"
        echo
        read -p "Remove all lock files? [y/N]: " -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            find "$HOME" -name "*.lock" -path "*flow*" -delete 2>/dev/null || true
            print_info "Removed lock files"
            removed=true
        fi
    fi
    
    # Clean up PATH from shell rc files
    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [ -f "$rc" ]; then
            if grep -q "progflow" "$rc" 2>/dev/null; then
                print_info "Found progflow entry in $rc"
                read -p "Remove progflow lines from $rc? [y/N]: " -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    sed -i '/progflow/d' "$rc"
                    print_info "Cleaned $rc"
                fi
            fi
        fi
    done
    
    if [ "$removed" = true ]; then
        print_success "$PROGRAM_NAME uninstalled successfully!"
    else
        print_warning "$PROGRAM_NAME not found"
    fi
}

# Show status
status() {
    print_info "Checking $PROGRAM_NAME installation status..."
    
    local binary
    binary=$(find_binary) || true
    
    if [ -n "$binary" ]; then
        print_success "Installed: $binary"
        "$binary" --version 2>/dev/null || "$binary" --help | head -1
    else
        print_warning "$PROGRAM_NAME is not installed"
    fi
    
    if [ -d "$CONFIG_DIR" ]; then
        local config_count
        config_count=$(find "$CONFIG_DIR" -name "*.json" 2>/dev/null | wc -l)
        print_info "Config directory: $CONFIG_DIR ($config_count flow(s))"
    else
        print_info "No config directory found"
    fi
}

# Main
main() {
    case "${1:-uninstall}" in
        uninstall|remove)
            uninstall
            ;;
        status)
            status
            ;;
        help|--help|-h)
            echo "Usage: $0 [COMMAND]"
            echo ""
            echo "Commands:"
            echo "  uninstall   Remove progflow (default)"
            echo "  status      Show installation status"
            echo "  help        Show this help"
            ;;
        *)
            print_error "Unknown command: $1"
            exit 1
            ;;
    esac
}

main "$@"
