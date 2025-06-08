#!/usr/bin/env bash
# ABOUTME: Wrapper for Zwift OCR telemetry extraction tools
# Usage: zwift_ocr.sh [screenshot|video|test] [options]

set -euo pipefail
IFS=$'\n\t'

# Debug mode
DEBUG="${DEBUG:-0}"
[[ "$DEBUG" == "1" ]] && set -x

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Error handling
die() { echo "ERROR: $*" >&2; exit 1; }
warn() { echo "WARNING: $*" >&2; }

# Dependency check
check_dependencies() {
    local deps=("$@")
    for cmd in "${deps[@]}"; do
        command -v "$cmd" >/dev/null 2>&1 || die "Missing: $cmd"
    done
}

# Usage information
usage() {
    cat << EOF
Zwift OCR Telemetry Extraction Tool

Usage: $0 [command] [options]

Commands:
    screenshot <path>     Extract telemetry from a screenshot
    video <path>         Process a video file
    test                 Run OCR engine comparison tests
    calibrate            Calibrate pose detection with sample images
    help                 Show this help message

Options:
    --engine [paddle|easy]    OCR engine to use (default: paddle)
    --skip-frames <n>         Skip frames in video processing (default: 30)
    --no-preview             Disable preview window for video processing
    --analyze                Show data analysis after processing

Examples:
    $0 screenshot docs/screenshots/normal_1_01_16_02_21.jpg
    $0 video recording.mp4 --skip-frames 60
    $0 test

For one-off execution without project setup:
    uvx --from . zwift_ocr_improved.py
EOF
}

# Main function
main() {
    check_dependencies "uv"
    
    # Ensure we're in the OCR tools directory
    cd "$SCRIPT_DIR"
    
    # Check if project is set up
    if [[ ! -f "pyproject.toml" ]]; then
        die "pyproject.toml not found. Run 'uv init' first."
    fi
    
    # Ensure dependencies are installed
    if [[ ! -d ".venv" ]]; then
        echo "Setting up Python environment..."
        uv sync
    fi
    
    # Parse command
    local command="${1:-help}"
    shift || true
    
    case "$command" in
        screenshot)
            [[ $# -eq 0 ]] && die "Screenshot path required"
            echo "Extracting telemetry from screenshot: $1"
            uv run python zwift_ocr_improved.py "$@"
            ;;
            
        video)
            [[ $# -eq 0 ]] && die "Video path required"
            echo "Processing video: $1"
            uv run python zwift_video_processor.py "$@"
            ;;
            
        test)
            echo "Running OCR engine comparison..."
            uv run python zwift_ocr_prototype.py "$@"
            ;;
            
        calibrate)
            echo "Calibrating pose detection..."
            uv run python test_pose_detection.py "$@"
            ;;
            
        help|--help|-h)
            usage
            exit 0
            ;;
            
        *)
            die "Unknown command: $command. Use '$0 help' for usage."
            ;;
    esac
}

# Test mode
if [[ "${1:-}" == "--test" ]]; then
    # Run self-tests
    echo "Running tests..."
    
    # Test dependency check
    check_dependencies "uv" || echo "PASS: Dependency check works"
    
    # Test file existence
    [[ -f "pyproject.toml" ]] && echo "PASS: Project file found"
    
    # Test command parsing
    DEBUG=1 main help >/dev/null && echo "PASS: Help command works"
    
    exit 0
fi

# Execute main
main "$@"