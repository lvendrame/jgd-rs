#!/bin/bash

# cargo-publish.sh - Script to publish JGD-rs crates to crates.io
# This script publishes the crates in the correct dependency order

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to check if a crate is already published
check_crate_exists() {
    local crate_name=$1
    local version=$2

    print_info "Checking if $crate_name v$version is already published..."

    if cargo search "$crate_name" --limit 1 | grep -q "^$crate_name = \"$version\""; then
        return 0  # Crate exists
    else
        return 1  # Crate doesn't exist
    fi
}

# Function to extract version from Cargo.toml
get_version() {
    local cargo_toml_path=$1
    grep '^version = ' "$cargo_toml_path" | sed 's/version = "\(.*\)"/\1/'
}

# Function to update CLI dependency to use published version
update_cli_dependency() {
    local lib_version=$1
    print_info "Updating jgd-rs-cli dependency to use published version $lib_version..."

    cd jgd-rs-cli

    # Clean up any existing backup/temp files first
    rm -f Cargo.toml.backup Cargo.toml.tmp

    # Create backup of original Cargo.toml
    cp Cargo.toml Cargo.toml.backup

    # Update dependency to use published version instead of path
    # Use a more robust approach with multiple sed commands
    sed -i.tmp 's/jgd-rs = { path = "\.\.\/jgd-rs", version = "[^"]*" }/jgd-rs = "'$lib_version'"/' Cargo.toml
    rm -f Cargo.toml.tmp

    # Verify the change was made
    if grep -q 'jgd-rs = "'$lib_version'"' Cargo.toml; then
        print_success "Successfully updated dependency to version $lib_version"
    else
        print_error "Failed to update dependency in Cargo.toml"
        restore_cli_dependency
        exit 1
    fi

    cd - > /dev/null
}

# Function to restore CLI dependency to path-based
restore_cli_dependency() {
    print_info "Restoring jgd-rs-cli dependency to path-based..."
    cd jgd-rs-cli
    if [[ -f Cargo.toml.backup ]]; then
        mv Cargo.toml.backup Cargo.toml
        print_info "Restored original Cargo.toml"
    fi
    # Clean up any temporary files
    rm -f Cargo.toml.tmp Cargo.toml.backup
    cd - > /dev/null
}

# Function to publish a crate
publish_crate() {
    local crate_dir=$1
    local crate_name=$2

    print_info "Publishing $crate_name..."

    cd "$crate_dir"

    # Get version from Cargo.toml
    local version=$(get_version "Cargo.toml")
    print_info "Version: $version"

    # Check if already published
    if check_crate_exists "$crate_name" "$version"; then
        print_warning "$crate_name v$version is already published. Skipping..."
        cd - > /dev/null
        return 0
    fi

    # Check if we need --allow-dirty flag (for CLI with backup files)
    local allow_dirty=""
    if [[ -f "Cargo.toml.backup" ]]; then
        allow_dirty="--allow-dirty"
        print_warning "Using --allow-dirty flag due to backup files"
    fi

    # Dry run first
    print_info "Running dry-run for $crate_name..."
    if ! cargo publish --dry-run $allow_dirty; then
        print_error "Dry-run failed for $crate_name"
        cd - > /dev/null
        return 1
    fi

    # If in dry-run mode, don't actually publish
    if [[ "${DRY_RUN:-false}" == "true" ]]; then
        print_success "Dry-run completed successfully for $crate_name"
        cd - > /dev/null
        return 0
    fi

    # Actual publish
    print_info "Publishing $crate_name to crates.io..."
    if cargo publish $allow_dirty; then
        print_success "$crate_name v$version published successfully!"
    else
        print_error "Failed to publish $crate_name"
        cd - > /dev/null
        return 1
    fi

    cd - > /dev/null

    # Wait a bit for crates.io to process
    print_info "Waiting 30 seconds for crates.io to process $crate_name..."
    sleep 30
}

# Main script
main() {
    print_info "Starting JGD-rs crates publication process..."

    # Set up cleanup trap
    trap 'restore_cli_dependency' EXIT ERR

    # Clean up any existing backup files from previous runs
    rm -f jgd-rs-cli/Cargo.toml.backup jgd-rs-cli/Cargo.toml.tmp

    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || [[ ! -d "jgd-rs" ]] || [[ ! -d "jgd-rs-cli" ]]; then
        print_error "Please run this script from the jgd-rs repository root directory"
        exit 1
    fi

    # Check if user is logged in to crates.io
    print_info "Checking crates.io authentication..."
    if ! cargo login --help > /dev/null 2>&1; then
        print_error "cargo login command not available. Please ensure you have cargo installed."
        exit 1
    fi

    # Warn about authentication
    print_warning "Make sure you're logged in to crates.io with 'cargo login <token>'"
    read -p "Press Enter to continue or Ctrl+C to abort..."

    # Build and test everything first
    print_info "Building and testing all crates..."
    if ! cargo build --release; then
        print_error "Build failed. Please fix build errors before publishing."
        exit 1
    fi

    # Skip tests if requested
    if [[ "${SKIP_TESTS:-false}" != "true" ]]; then
        if ! cargo test; then
            print_error "Tests failed. Please fix test failures before publishing."
            exit 1
        fi
        print_success "All builds and tests passed!"
    else
        print_warning "Skipping tests as requested"
        print_success "Build passed!"
    fi

    # Publish in dependency order
    # 1. First publish the library (jgd-rs) since CLI depends on it
    local lib_version=$(get_version jgd-rs/Cargo.toml)
    if ! publish_crate "jgd-rs" "jgd-rs"; then
        print_error "Failed to publish jgd-rs library"
        exit 1
    fi

    # 2. Update CLI dependency to use published version and publish CLI
    if [[ "${DRY_RUN:-false}" != "true" ]]; then
        update_cli_dependency "$lib_version"

        # Build CLI with updated dependency to verify it works
        print_info "Building CLI with published dependency..."
        cd jgd-rs-cli
        if ! cargo build --release; then
            print_error "Failed to build CLI with published dependency"
            cd - > /dev/null
            restore_cli_dependency
            exit 1
        fi
        cd - > /dev/null
    fi

    if ! publish_crate "jgd-rs-cli" "jgd-rs-cli"; then
        if [[ "${DRY_RUN:-false}" != "true" ]]; then
            restore_cli_dependency
        fi
        print_error "Failed to publish jgd-rs-cli"
        exit 1
    fi

    # Restore original dependency configuration for development
    if [[ "${DRY_RUN:-false}" != "true" ]]; then
        restore_cli_dependency
    fi

    # Final cleanup
    cd jgd-rs-cli
    rm -f Cargo.toml.backup Cargo.toml.tmp
    cd - > /dev/null

    print_success "All crates published successfully!"
    print_info "Publication complete. You can now install with:"
    echo "  cargo install jgd-rs-cli"
    echo "  # Or add to Cargo.toml: jgd-rs = \"$(get_version jgd-rs/Cargo.toml)\""
}

# Script options
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            print_info "Dry-run mode enabled"
            DRY_RUN=true
            shift
            ;;
        --skip-tests)
            print_warning "Skipping tests"
            SKIP_TESTS=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --dry-run     Run cargo publish --dry-run only"
            echo "  --skip-tests  Skip running tests before publishing"
            echo "  --help, -h    Show this help message"
            echo ""
            echo "This script publishes JGD-rs crates to crates.io in the correct dependency order."
            echo "Make sure you're logged in with 'cargo login <token>' before running."
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Run main function
main
