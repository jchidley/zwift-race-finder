# Rust Refactoring Tools Quick Reference

## Essential Tool Installation

```bash
# Core refactoring tools
cargo install cargo-edit         # Add/remove/upgrade dependencies
cargo install cargo-expand       # Expand macros for understanding
cargo install cargo-machete      # Find unused dependencies
cargo install cargo-outdated     # Check for outdated dependencies
cargo install cargo-audit        # Security audit dependencies

# Code quality tools
rustup component add clippy      # Linting and suggestions
rustup component add rustfmt     # Code formatting

# Advanced tools (optional but recommended)
cargo install cargo-mutants      # Mutation testing
cargo install cargo-semver-checks # Check for breaking changes
cargo install cargo-criterion    # Benchmarking framework
cargo install cargo-nextest      # Better test runner
cargo install cargo-watch        # Auto-run on file changes

# For thorough unused dependency checking (requires nightly)
cargo install cargo-udeps
```

## IDE Integration

### VS Code
1. Install "rust-analyzer" extension
2. Install "Error Lens" for inline errors
3. Enable format on save:
   ```json
   {
     "[rust]": {
       "editor.formatOnSave": true,
       "editor.defaultFormatter": "rust-lang.rust-analyzer"
     }
   }
   ```

### IntelliJ IDEA
1. Install "Rust" plugin
2. Enable "Optimize imports on the fly"
3. Configure rustfmt integration

### Neovim
```lua
-- With lazy.nvim
{
  'simrat39/rust-tools.nvim',
  dependencies = { 'neovim/nvim-lspconfig' },
  config = function()
    require('rust-tools').setup({})
  end
}
```

## Common Refactoring Commands

### rust-analyzer (VS Code)
- `Ctrl+.` - Quick fixes and refactorings
- `F2` - Rename symbol
- `Ctrl+Shift+R` - Refactor menu
- `Alt+Enter` - Show available actions

### Cargo Commands
```bash
# Apply compiler suggestions
cargo fix

# Apply clippy suggestions
cargo fix --clippy

# Migrate to new edition
cargo fix --edition

# Format code
cargo fmt

# Check without building
cargo check

# Run clippy with all lints
cargo clippy -- -W clippy::all -W clippy::pedantic

# Expand macros
cargo expand main

# Watch for changes
cargo watch -x check -x test -x clippy
```

## Tool-Specific Usage

### cargo-edit
```bash
cargo add serde --features derive    # Add dependency
cargo rm old-crate                   # Remove dependency
cargo upgrade --compatible           # Upgrade preserving compatibility
cargo set-version 2.0.0             # Set crate version
```

### cargo-machete
```bash
cargo machete                        # Find unused dependencies
cargo machete --fix                  # Remove unused dependencies
```

### cargo-mutants
```bash
cargo mutants                        # Run mutation testing
cargo mutants --jobs 4              # Parallel execution
cargo mutants --file src/parser.rs  # Test specific file
```

### cargo-expand
```bash
cargo expand                         # Expand all macros
cargo expand main                    # Expand specific module
cargo expand --test test_name       # Expand test
```

### cargo-audit
```bash
cargo audit                          # Check for vulnerabilities
cargo audit fix                      # Fix vulnerabilities if possible
```

## Refactoring Workflows

### Module Extraction Workflow
```bash
# 1. Identify module to extract
cargo clippy -- -W clippy::all

# 2. Create module structure
mkdir src/new_module
touch src/new_module/mod.rs

# 3. Move code incrementally, checking each step
cargo check

# 4. Run tests after each move
cargo test

# 5. Format and lint
cargo fmt
cargo clippy -- -D warnings
```

### Dependency Update Workflow
```bash
# 1. Check what needs updating
cargo outdated

# 2. Update compatible versions
cargo upgrade --compatible

# 3. Run tests
cargo test

# 4. Update breaking changes one by one
cargo upgrade --incompatible serde

# 5. Fix compilation errors and test
```

### Performance Refactoring Workflow
```bash
# 1. Establish baseline
cargo criterion

# 2. Make refactoring changes

# 3. Compare performance
cargo criterion --compare

# 4. Generate flamegraph (requires perf)
cargo flamegraph
```

## Quick Diagnostics

```bash
# Project health check
echo "=== Checking project health ==="
cargo fmt -- --check && echo "✓ Formatting OK"
cargo clippy -- -D warnings && echo "✓ No clippy warnings"
cargo test --quiet && echo "✓ All tests passing"
cargo audit && echo "✓ No security vulnerabilities"
cargo machete && echo "✓ No unused dependencies"
cargo outdated && echo "✓ Dependencies checked"
```

## Useful Aliases

Add to your shell configuration:
```bash
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias ccl='cargo clippy -- -W clippy::all'
alias cw='cargo watch -x check -x test -x clippy'
alias ce='cargo expand'
alias cud='cargo +nightly udeps'
```

## Troubleshooting

### rust-analyzer not working
```bash
# Restart rust-analyzer
pkill rust-analyzer

# Clear cache
rm -rf target/.rustc_info.json
cargo clean

# Rebuild proc macros
cargo check
```

### Clippy not finding all issues
```bash
# Clean and re-run
cargo clean
cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
```

### cargo-mutants too slow
```bash
# Use fewer threads
cargo mutants --jobs 2

# Test specific modules
cargo mutants --file src/specific_module.rs

# Skip slow tests
cargo mutants -- --skip slow_test
```