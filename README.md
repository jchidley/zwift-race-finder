# Zwift Race Finder

Predict Zwift race durations from Racing Score and filter events by target time.

## Quick Start

```bash
./install.sh
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15
```

Config file example: `config.example.toml` → `~/.config/zwift-race-finder/config.toml`

## Documentation

- `docs/README.md` - Documentation index
- `docs/for-racers/` - Racing guides and tactics
- `docs/for-developers/` - Architecture, testing, contribution notes
- `docs/guides/` - Setup and operational how-to guides
- `docs/reference/` - Algorithms, domain concepts, schema
- `docs/research/` - Deep dives and investigations
- `docs/project-history/` - Accuracy and development timeline

## Notes

- Follow Zwift's Terms of Service when using or modifying this tool.
- Run `cargo test --test regression_tests -- --nocapture` before claiming accuracy changes.

## About This Code

Almost all of this code is AI/LLM-generated. It's best used as a source of inspiration for your own AI/LLM efforts rather than as a traditional library.

**This is personal alpha software.** If you want to use it:

- Pin to a specific commit; don't track `main`.
- Use AI/LLM to adapt it; without AI assistance, this project is hard to use.
- Treat it as inspiration rather than a dependency.

Suggestions are welcome as inspiration for future improvements.
