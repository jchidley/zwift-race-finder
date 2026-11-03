#!/bin/bash

set -e

echo "🔍 Scanning for potential secrets and sensitive data..."
echo "============================================================"

FOUND_ISSUES=0

RED='\033[0;31m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "\n${YELLOW}Checking for hardcoded profile IDs...${NC}"
if grep -r "1106548" . \
    --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
    --include="*.md" --include="*.toml" --include="*.json" \
    --exclude-dir=".git" --exclude-dir="target" 2>/dev/null; then
    echo -e "${RED}❌ Found hardcoded profile ID 1106548${NC}"
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No hardcoded profile IDs found${NC}"
fi

echo -e "\n${YELLOW}Checking for session IDs...${NC}"
if grep -r -E "sid[\"'= ]+[a-fA-F0-9]{32,}" . \
    --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
    --include="*.md" --include="*.toml" --include="*.json" \
    --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
    grep -v "YOUR_SESSION_ID"; then
    echo -e "${RED}❌ Found potential session IDs${NC}"
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No session IDs found${NC}"
fi

echo -e "\n${YELLOW}Checking for API keys and tokens...${NC}"
PATTERNS=(
    "[aA][pP][iI][-_]?[kK][eE][yY].*[:=][\"']?[a-zA-Z0-9]{20,}[\"']?"
    "[aA][cC][cC][eE][sS][sS][-_]?[tT][oO][kK][eE][nN].*[:=][\"']?[a-zA-Z0-9]{20,}[\"']?"
    "[sS][eE][cC][rR][eE][tT].*[:=][\"']?[a-zA-Z0-9]{20,}[\"']?"
    "AKIA[0-9A-Z]{16}"
    "sk_live_[a-zA-Z0-9]{24,}"
    "pk_live_[a-zA-Z0-9]{24,}"
)

for pattern in "${PATTERNS[@]}"; do
    if grep -r -E "$pattern" . \
        --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
        --include="*.md" --include="*.toml" --include="*.json" --include="*.env*" \
        --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
        grep -v -E "example|template|YOUR_|placeholder|<.*>"; then
        echo -e "${RED}❌ Found potential secrets matching: $pattern${NC}"
        FOUND_ISSUES=1
    fi
done

if [ $FOUND_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ No API keys or tokens found${NC}"
fi

echo -e "\n${YELLOW}Checking for email addresses...${NC}"
if grep -r -E "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" . \
    --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
    --include="*.md" --include="*.toml" --include="*.json" \
    --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
    grep -v -E "example\.com|your\.email|noreply@|LICENSE"; then
    echo -e "${RED}❌ Found email addresses${NC}"
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No email addresses found${NC}"
fi

echo -e "\n${YELLOW}Checking for personal file paths...${NC}"
if grep -r -E "/Users/[a-zA-Z]+/|/home/[a-zA-Z]+/|C:\\\\Users\\\\[a-zA-Z]+\\\\|/mnt/c/Users/[a-zA-Z]+/" . \
    --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
    --include="*.md" --include="*.toml" --include="*.json" \
    --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
    grep -v -E "YOUR_USERNAME|username|example|<.*>"; then
    echo -e "${RED}❌ Found personal file paths${NC}"
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No personal file paths found${NC}"
fi

echo -e "\n${YELLOW}Checking for .env files...${NC}"
ENV_FILES=$(find . -name ".env*" -not -name ".env.example" -not -path "./.git/*" 2>/dev/null)
if [ -n "$ENV_FILES" ]; then
    echo -e "${RED}❌ Found .env files:${NC}"
    echo "$ENV_FILES"
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No .env files found${NC}"
fi

echo -e "\n${YELLOW}Checking for personal data files...${NC}"
if [ -f "zwiftpower_results.json" ] || [ -f "zwiftpower_page_structure.json" ]; then
    echo -e "${RED}❌ Found personal data files${NC}"
    ls -la zwiftpower*.json 2>/dev/null || true
    FOUND_ISSUES=1
else
    echo -e "${GREEN}✓ No personal data files found${NC}"
fi

echo -e "\n${YELLOW}Checking for database files...${NC}"
DB_FILES=$(find . -name "*.db" -o -name "*.sqlite" -o -name "*.db-journal" -o -name "*.db-wal" 2>/dev/null | grep -v ".git")
if [ -n "$DB_FILES" ]; then
    echo -e "${YELLOW}⚠️  Found database files (ensure no personal data):${NC}"
    echo "$DB_FILES"
fi

echo -e "\n============================================================"
if [ $FOUND_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✅ No secrets or sensitive data found!${NC}"
    echo "Repository appears safe to make public."
else
    echo -e "${RED}❌ FOUND SENSITIVE DATA!${NC}"
    echo "Please run ./sanitize_personal_data.sh to clean the repository."
    echo "Or manually review and fix the issues listed above."
    exit 1
fi