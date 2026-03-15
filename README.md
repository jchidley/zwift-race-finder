# Zwift Race Finder

Predict Zwift race durations from your Racing Score and find events that fit your schedule.

## Quick Start

```bash
cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15
```

Optionally save defaults to `~/.config/zwift-race-finder/config.toml` (see `config.example.toml`).

## Documentation

| Need | Go to |
|------|-------|
| Learn to use it | [Tutorial: Find your first race](docs/tutorial/getting-started.md) |
| Do a specific task | [How-to guides](docs/howto/) — deployment, data import, config, secrets |
| Look something up | [Reference](docs/reference/) — algorithms, architecture, database, CLI |
| Understand why | [Explanation](docs/explanation/) — Zwift physics, racing tactics, testing philosophy |

Other resources:
- [Project history](docs/project-history/) — accuracy timeline, discoveries
- [PROJECT_WISDOM.md](PROJECT_WISDOM.md) — learning log

## Notes

- Follow Zwift's Terms of Service when using or modifying this tool.
- Run `cargo test regression_test -- --nocapture` before claiming accuracy changes.

## About This Code

Almost all of this code is AI/LLM-generated. It's best used as a source of
inspiration for your own AI/LLM efforts rather than as a traditional library.

**This is personal alpha software.** If you want to use it:

- **Pin to a specific commit** — don't track `main`, it changes without warning
- **Use AI/LLM to adapt** — without AI assistance, this project is hard to use
- **Treat as inspiration** — build your own version rather than depending on mine

Suggestions welcome as inspiration for future improvements.

## License

MIT OR Apache-2.0
