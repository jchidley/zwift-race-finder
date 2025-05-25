#!/bin/bash
# Setup git hooks to prevent committing secrets

set -e

echo "Setting up git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook to check for secrets

# Run the secrets check script if it exists
if [ -f "./check_secrets.sh" ]; then
    echo "Running security check before commit..."
    if ! ./check_secrets.sh; then
        echo ""
        echo "❌ Commit aborted due to security issues!"
        echo "Run ./sanitize_personal_data.sh to fix these issues."
        exit 1
    fi
else
    # Fallback: basic checks if script doesn't exist
    echo "Checking for common secrets..."
    
    # Check for specific profile ID
    if git diff --cached --name-only -z | xargs -0 grep -l "1106548" 2>/dev/null; then
        echo "❌ Error: Found hardcoded profile ID in staged files"
        exit 1
    fi
    
    # Check for long hex strings that might be keys
    if git diff --cached --name-only -z | xargs -0 grep -E "[a-fA-F0-9]{40,}" 2>/dev/null | grep -v "Cargo.lock"; then
        echo "⚠️  Warning: Found potential API keys (long hex strings)"
        echo "Please review before committing."
    fi
fi

# Run cargo fmt check
if command -v cargo >/dev/null 2>&1; then
    echo "Checking code formatting..."
    if ! cargo fmt -- --check; then
        echo "❌ Code is not formatted. Run 'cargo fmt' before committing."
        exit 1
    fi
fi

echo "✅ Pre-commit checks passed!"
EOF

# Make hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Git hooks installed!"
echo ""
echo "The pre-commit hook will now:"
echo "  - Check for secrets and personal data"
echo "  - Verify code formatting with cargo fmt"
echo ""
echo "To skip hooks temporarily (not recommended):"
echo "  git commit --no-verify"