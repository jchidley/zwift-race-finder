# Zwift Race Coach

Help a rider pace and race better through evidence-based ride reviews, practical pre-race plans, and controlled strategy experiments.

This project owns ride reviews, pacing plans and controlled strategy experiments. [Fitness Bridge](https://github.com/jchidley/fitness-bridge) owns sensor acquisition, transformations and BLE/ANT+ output; the Life Fitness project owns custom hardware, Pico firmware and physical control. `zwift-race-finder` remains independent and owns event selection and route-data extraction/import. None of their implementation backlogs is a prerequisite for reviewing a recorded ride.

## Getting started

There is no installation or application command yet. From an existing checkout at `~/git/zwift-race-coach`, start a project session and provide an explicit path to one completed activity **outside the checkout**, permission to read it, and your question about the ride. Do not upload private activity data to Git. No runtime, account login, live bridge or upcoming event is required for this first step.

The first observable result is a read-only inventory of available fields and time/data-quality limitations. That inventory determines which review metrics are supported and which parser/runtime, if any, is needed. See the [first ride-review contract](docs/explanation/FIRST_RIDE_REVIEW.md).

## First useful workflow

Given one explicitly approved completed activity:

1. Identify which time, power, cadence, speed, altitude, and position fields are actually available.
2. Review the distribution of effort and recovery, distinguishing observations from uncertain explanations.
3. Produce a private ride review with practical takeaways and explicit evidence limits.
4. Optionally add a pacing plan when upcoming-event details and adequate rider context are supplied; this does not block the ride review.

No maintained coaching application, activity-intake/report command, simulator or device integration exists yet. One-off read-only analysis is distinct from an implemented intake pipeline; see [current progress and evidence limits](STATUS.md). The implementation language/runtime remains unselected; there is no placeholder executable or pretend test suite.

The [first ride-review contract](docs/explanation/FIRST_RIDE_REVIEW.md) defines the private input, report and acceptance criteria. No live bridge is needed for that workflow.

## Before and after a race

Choose an event in the independent race-finder project, then supply its event link or route/format snapshot here. With adequate rider evidence, make a private pre-race plan covering the objective, likely selection points, conditional efforts, recovery opportunities and fallback. A simulator, new maximal test or reusable analysis command is not a prerequisite.

After racing, provide the completed ride recording for read-only review. Compare what happened with the preserved plan, distinguishing measured effort from uncertain tactical explanations. Optional video or immediate notes help because a FIT file alone does not establish pack position or drafting. Keep the plan, rider profile and report outside Git. See the [race-planning and review standards](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2).

## Documentation

- [Documentation map](docs/README.md)
- [Pacing and race experiments](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md)
- [Local Zwift ecosystem and multi-instance references](docs/reference/ZWIFT_ECOSYSTEM.md)
- [Historical simulation-tool catalogue](docs/reference/SIMULATION_TOOLS.md)
- [Project boundaries and migration record](docs/reference/PROJECT_BOUNDARIES.md)
- [Current status](STATUS.md) and [tasks](TASKS.md)
- [Contributor instructions and change-specific validation](AGENTS.md)

## Boundaries

- Keep personal activities, credentials, proprietary client exports, and generated experiment recordings outside Git.
- Leave race-finder and its backlog alone. If this workflow needs route/event data, accept an explicit supplied snapshot; do not require finder changes or share writable databases.
- Recorded ghosts are fixed trajectories, not responsive opponents. Client-based trials require verified inputs, telemetry, realistic effort constraints, and repeatable starting conditions.
- Native Windows Zwift is the first live-client baseline. netbrain/zwift and same-machine multi-instance work are deferred; their input/telemetry compatibility is not assumed.
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
