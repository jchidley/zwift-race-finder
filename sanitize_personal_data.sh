#!/bin/bash
# Script to remove personal information before making repository public

set -e

echo "🔒 Sanitizing personal data from repository..."

# Backup current state
echo "Creating backup..."
cp -r . ../zwift-race-finder-backup-$(date +%Y%m%d-%H%M%S) 2>/dev/null || true

# 0. Check for potential secrets/keys
echo "🔍 Scanning for potential secrets..."
FOUND_SECRETS=0

# Common patterns for secrets
SECRET_PATTERNS=(
    # API keys and tokens
    "api[_-]?key"
    "api[_-]?secret"
    "access[_-]?token"
    "auth[_-]?token"
    "private[_-]?key"
    "secret[_-]?key"
    "bearer"
    
    # Specific service patterns
    "sk_live_"
    "pk_live_"
    "github[_-]?token"
    "gh[_-]?token"
    "gitlab[_-]?token"
    "slack[_-]?token"
    
    # AWS
    "aws[_-]?access"
    "aws[_-]?secret"
    "AKIA[0-9A-Z]{16}"
    
    # Generic secrets
    "password"
    "passwd"
    "pwd"
    "credential"
    "jwt"
    "oauth"
    "client[_-]?secret"
)

# Check for hex/base64 strings that look like keys (40+ chars)
echo "Checking for long hex/base64 strings that might be keys..."
if grep -r -E "[a-fA-F0-9]{40,}|[a-zA-Z0-9+/]{40,}=" . \
    --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
    --include="*.md" --include="*.toml" --include="*.json" \
    --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
    grep -v -E "LICENSE|lock|Cargo\.lock|/target/"; then
    echo "⚠️  WARNING: Found potential keys (long hex/base64 strings)"
    FOUND_SECRETS=1
fi

# Check for common secret patterns
for pattern in "${SECRET_PATTERNS[@]}"; do
    if grep -r -i "$pattern" . \
        --include="*.rs" --include="*.sh" --include="*.py" --include="*.js" \
        --include="*.md" --include="*.toml" --include="*.json" --include="*.env*" \
        --exclude-dir=".git" --exclude-dir="target" 2>/dev/null | \
        grep -v -E "example|template|\.md:|LICENSE|lock|false|null|undefined|placeholder|your[_-]?|<.*>|\\.gitignore" | \
        grep -i -E "=|:|\s+['\"]?[a-zA-Z0-9]{20,}['\"]?"; then
        echo "⚠️  WARNING: Found potential secrets matching pattern: $pattern"
        FOUND_SECRETS=1
    fi
done

# Check for .env files
if find . -name ".env*" -not -name ".env.example" -not -path "./.git/*" 2>/dev/null | grep -q .; then
    echo "⚠️  WARNING: Found .env files that might contain secrets:"
    find . -name ".env*" -not -name ".env.example" -not -path "./.git/*" 2>/dev/null
    FOUND_SECRETS=1
fi

if [ $FOUND_SECRETS -eq 1 ]; then
    echo ""
    echo "❌ POTENTIAL SECRETS DETECTED!"
    echo "Please review the warnings above and remove any real secrets before continuing."
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborting sanitization. Please remove secrets first."
        exit 1
    fi
fi

# 1. Replace ZwiftPower profile ID and session ID
echo "Replacing ZwiftPower IDs..."
find . -type f -name "*.md" -o -name "*.sh" -o -name "*.py" -o -name "*.rs" | \
    xargs sed -i 's/z=YOUR_PROFILE_ID/z=YOUR_PROFILE_ID/g'
find . -type f -name "*.md" -o -name "*.sh" -o -name "*.py" -o -name "*.rs" | \
    xargs sed -i 's/profile\.php?z=YOUR_PROFILE_ID/profile.php?z=YOUR_PROFILE_ID/g'
find . -type f -name "*.md" -o -name "*.sh" -o -name "*.py" -o -name "*.rs" | \
    xargs sed -i 's/sid=YOUR_SESSION_ID/sid=YOUR_SESSION_ID/g'

# 2. Replace Windows username in paths
echo "Replacing Windows paths..."
find . -type f -name "*.md" -o -name "*.sh" -o -name "*.py" | \
    xargs sed -i 's|/mnt/c/Users/YOUR_USERNAME/|/mnt/c/Users/YOUR_USERNAME/|g'

# 3. Replace email addresses
echo "Replacing email addresses..."
find . -type f -name "*.md" | \
    xargs sed -i 's/rechung@gmail\.com/your.email@example.com/g'

# 4. Update main.rs to use generic fallback data
echo "Updating main.rs..."
sed -i 's/username: "Jack"\.to_string()/username: "User"\.to_string()/g' src/main.rs

# 5. Remove personal data files if they exist
echo "Removing personal data files..."
rm -f zwiftpower_results.json zwiftpower_page_structure.json

# 6. Create a config template for users
cat > config.example.json << 'EOF'
{
  "zwiftpower_profile_id": "YOUR_PROFILE_ID",
  "zwiftpower_session_id": "YOUR_SESSION_ID",
  "default_zwift_score": 195,
  "default_category": "D",
  "windows_username": "YOUR_USERNAME"
}
EOF

# 7. Add .env.example for environment variables
cat > .env.example << 'EOF'
# ZwiftPower Configuration
ZWIFTPOWER_PROFILE_ID=YOUR_PROFILE_ID
ZWIFTPOWER_SESSION_ID=YOUR_SESSION_ID

# Default settings
DEFAULT_ZWIFT_SCORE=195
DEFAULT_CATEGORY=D
EOF

# 8. Update README to mention config
cat >> README.md << 'EOF'

## Configuration

For personal settings, copy `config.example.json` to `config.json` and update with your values:
- `zwiftpower_profile_id`: Your ZwiftPower profile ID (found in your profile URL)
- `zwiftpower_session_id`: Session ID from ZwiftPower cookies (optional)
- `default_zwift_score`: Your current Zwift Racing Score
- `windows_username`: Your Windows username (for WSL paths)
EOF

# 9. Add security notice to README
echo "Adding security notice to README..."
cat >> README.md << 'EOF'

## Security Notice

- **Never commit** your ZwiftPower session IDs or profile IDs to version control
- Keep your `config.json` and `.env` files private (they're in .gitignore)
- Browser extraction scripts should only be used on your own ZwiftPower profile
- This tool only uses public Zwift APIs and does not store credentials

## Privacy

This tool stores all data locally on your machine. No data is sent to external servers.
EOF

echo ""
echo "✅ Sanitization complete!"
echo ""
echo "⚠️  IMPORTANT: Please review the following files manually:"
echo "   - Cargo.toml (author name)"
echo "   - LICENSE-MIT (copyright holder)"
echo "   - SECURITY_AUDIT.md (review findings)"
echo "   - Any other files that might contain personal data"
echo ""
echo "You may want to:"
echo "1. Update author in Cargo.toml to your GitHub username or organization"
echo "2. Update copyright in LICENSE-MIT"
echo "3. Review all .md files for any remaining personal references"
echo "4. Verify personal data files are removed"
echo ""
echo "To verify clean state:"
echo "  git grep -i '1106548\|jackc\|rechung'"
echo "  git status --ignored"
echo ""
echo "After review, you can make the repository public!"
echo ""
echo "📝 For personal use after sanitization:"
echo ""
echo "Option 1 - Automatic (if you ran setup_secure_config.sh):"
echo "  Your config is safely stored at ~/.config/zwift-race-finder/"
echo "  The tool will automatically use it!"
echo ""
echo "Option 2 - Manual restore:"
echo "  ./restore_personal_config.sh"
echo ""
echo "Option 3 - Fresh setup:"
echo "  1. Copy config.example.json to config.json"
echo "  2. Fill in your personal values"