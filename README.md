# Zwift Race Coach

Help a rider pace and race better through evidence-based ride reviews, practical pre-race plans, and controlled strategy experiments.

This is an independent sibling of `zwift-race-finder`, not a new module inside it. The finder answers **which event fits**; the coach answers **how to prepare, pace, and learn from it**.

## First useful workflow

Given one explicitly approved completed activity and an upcoming event:

1. Identify which time, power, cadence, speed, altitude, and position fields are actually available.
2. Review the distribution of effort and recovery, distinguishing observations from uncertain explanations.
3. Produce a practical pacing plan with explicit assumptions and evidence limits.

No application, simulator, device integration, or personal-data import has been implemented yet. A language/runtime has not been selected; the initial setup intentionally contains no placeholder executable or pretend tests.

## Documentation

- [Documentation map](docs/README.md)
- [Pacing and race experiments](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md)
- [Local Zwift ecosystem and multi-instance references](docs/reference/ZWIFT_ECOSYSTEM.md)
- [Historical simulation-tool catalogue](docs/reference/SIMULATION_TOOLS.md)
- [Project boundaries and migration record](docs/reference/PROJECT_BOUNDARIES.md)
- [Current status](STATUS.md) and [tasks](TASKS.md)

## Boundaries

- Keep personal activities, credentials, proprietary client exports, and generated experiment recordings outside Git.
- Keep route extraction/import and the production race-finder model in the sibling project. Exchange versioned data rather than sharing writable databases.
- Recorded ghosts are fixed trajectories, not responsive opponents. Client-based trials require verified inputs, telemetry, realistic effort constraints, and repeatable starting conditions.
- netbrain/zwift is the priority candidate for multiple client instances, not an installed or validated dependency. Its documented ANT+ limitation must be resolved before assuming compatibility with vpower-based controllers.
- Synthetic inputs and controlled experiments are for offline/testing work, not automated competitive riding or substitutes for measured real-race accuracy.

Start project commands and a new Pi session from this directory, not from the race-finder checkout.
## About This Code

Almost all of this code is AI/LLM-generated. It's best used as a source of
inspiration for your own AI/LLM efforts rather than as a traditional library.

**This is personal alpha software.** All my GitHub projects should be considered
experimental. If you want to use them:

- **Pin to a specific commit** — don't track `main`, it changes without warning
- **Use AI/LLM to adapt** — without AI assistance, these projects are hard to use
- **Treat as inspiration** — build your own version rather than depending on mine

**Suggestions welcome** — If you have ideas for improvements or changes, I'd be
delighted to read them and use them as inspiration for my own efforts.

**Why not a library?** These days it's often quicker to use AI/LLM to build your
own than to integrate traditional libraries. My use of AI/LLM is inspired by
these people and posts:

- [Simon Willison's Weblog](https://simonwillison.net/) — Essential reading on
  LLMs, prompt engineering, and building with AI
- [CLI over MCP](https://lucumr.pocoo.org/2025/8/18/code-mcps/) — Armin Ronacher
  on why command-line tools are better integration points than custom protocols
- [Build It Yourself](https://lucumr.pocoo.org/2025/12/22/a-year-of-vibes/) —
  Armin Ronacher: "With our newfound power from agentic coding tools, you can
  build much of this yourself..."
- [Shipping at Inference Speed](https://steipete.me/posts/2025/shipping-at-inference-speed) —
  Peter Steinberger on the new workflow of building with AI assistance
- [Year in Review 2025](https://mariozechner.at/posts/2025-12-22-year-in-review-2025/) —
  Mario Zechner on AI-assisted development

**What I use:** Currently Anthropic's Claude Opus, evaluating OpenAI's GPT Codex
as an alternative.

## License

This project is dual-licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
