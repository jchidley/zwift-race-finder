#!/bin/bash
# ABOUTME: Validate that golden test data is representative of production data
# Usage: ./validate_test_data.sh

set -euo pipefail
IFS=$'\n\t'

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Zwift Race Finder - Test Data Validation${NC}"
echo -e "${BLUE}===========================================${NC}"
echo

# Check if production database exists
DB_PATH="$HOME/.local/share/zwift-race-finder/races.db"
if [[ ! -f "$DB_PATH" ]]; then
    echo -e "${RED}❌ Production database not found at: $DB_PATH${NC}"
    echo -e "${YELLOW}   This validation requires the production database.${NC}"
    echo -e "${YELLOW}   Run the application at least once to create it.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Found production database${NC}"
echo

# Change to project directory
cd "$PROJECT_ROOT"

# Run validation tests
echo -e "${BLUE}Running validation tests...${NC}"
echo

# Test 1: Route coverage and diversity
echo -e "${YELLOW}1. Validating route coverage and diversity${NC}"
cargo test --test golden_tests validate_test_routes -- --ignored --nocapture 2>&1 | grep -v "warning:" | grep -v "Compiling" || true

echo
echo -e "${YELLOW}2. Validating against race history${NC}"
cargo test --test golden_tests validate_against_race_history -- --ignored --nocapture 2>&1 | grep -v "warning:" | grep -v "Compiling" || true

echo
echo -e "${BLUE}===========================================${NC}"
echo -e "${BLUE}Validation Summary${NC}"
echo

# Provide recommendations
echo -e "${YELLOW}📋 Recommendations:${NC}"
echo "  • If test routes have <10% difference in mean duration: ✅ Good representation"
echo "  • If missing important routes: Consider adding them to test set"
echo "  • If prediction errors are high: May need to recalibrate estimations"
echo "  • Run this validation after major route database updates"
echo

# Check if we should update test data
read -p "Do you want to regenerate the golden baseline with current data? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "\n${YELLOW}Regenerating golden baseline...${NC}"
    cargo test generate_golden_baseline_improved -- --ignored
    echo -e "${GREEN}✓ Golden baseline regenerated${NC}"
    echo -e "${YELLOW}  Don't forget to commit the new baseline if tests pass!${NC}"
fi