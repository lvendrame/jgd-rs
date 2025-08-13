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

    # Dry run first
    print_info "Running dry-run for $crate_name..."
    if ! cargo publish --dry-run; then
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
    if cargo publish; then
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
    if ! publish_crate "jgd-rs" "jgd-rs"; then
        print_error "Failed to publish jgd-rs library"
        exit 1
    fi

    # 2. Then publish the CLI tool
    if ! publish_crate "jgd-rs-cli" "jgd-rs-cli"; then
        print_error "Failed to publish jgd-rs-cli"
        exit 1
    fi

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
