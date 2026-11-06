# GitHub setup preview — approved and applied

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

Initially prepared read-only with the owner's explicitly requested github-api skill on 2026-10-30. The owner subsequently approved this plan. The skill regenerated an identical plan and applied it to `jchidley/zwift-race-coach`; the resulting repository and three files were independently verified and the same files applied locally.

## Proposed external writes

- Create `jchidley/zwift-race-coach`, **private**, description: “Ride reviews, pacing plans, and controlled Zwift race-strategy experiments”.
- Disable GitHub Actions.
- Use branch `main`; create repository without auto-initialization, then create exactly `README.md`, `LICENSE-MIT`, and `LICENSE-APACHE` through the skill.
- Issues enabled; projects and wiki disabled; delete merged branches; squash and rebase merges enabled; merge commits disabled.
- README preserves the prepared local introduction and appends the owner's template sections. MIT copyright: 2026 Jack Chidley. Apache text: version 2.0.

## Source and exact preview evidence

- README template: https://raw.githubusercontent.com/jchidley/license_template/main/README.md
- MIT text: https://raw.githubusercontent.com/spdx/license-list-data/main/text/MIT.txt
- Apache text: https://raw.githubusercontent.com/spdx/license-list-data/main/text/Apache-2.0.txt
- Full preview, including exact proposed file contents: `/tmp/zwift-race-coach-github-preview-C0hPCg.txt` (local temporary evidence; regenerate if unavailable).
- `README.md`: SHA-256 `7a1b73064e6485b694734a68b2d8f3a53a99027b510d8898c4c61a05b8eec396`
- `LICENSE-MIT`: SHA-256 `c421e8e20f5a25234a1a12e51203b431efa4576896301b234210aff339d9e50b`
- `LICENSE-APACHE`: SHA-256 `074e6e32c86a4c0ef8b3ed25b721ca23aca83df277cd88106ef7177c354615ff`

The upstream README uses “dual-licensed under the terms of both” MIT and Apache-2.0 and includes the owner's standard AI/personal-alpha and contribution sections. The requested project licensing intent is the Rust-style MIT OR Apache-2.0 choice; retain that explicit choice in any future package metadata. The approved README and both license texts are now applied locally and remotely without payload changes.

The skill policy required explicit approval of the target, visibility, and exact writes before applying. Approval was received; the fresh plan matched exactly. A read-only preflight confirmed the repository did not exist before creation. Subsequent verification confirmed private visibility, `main`, all reviewed settings, disabled Actions, and exact contents of the three files. Verified remote head: `e9982b0eb5de6927b337376afe9572cb05ec2e0d` at 2026-10-30T08:28:42Z. Apply log: `/tmp/zwift-race-coach-github-apply.log`; verification receipt: `/tmp/zwift-race-coach-github-receipt.json`. Do not retry creation: the repository now exists.

Git initialization, commits, remote linking, and pushing the remaining local documentation are **not performed by this skill operation**. Review their scope separately from the canonical coaching project session; never stage or push the dirty race-finder checkout as part of this bootstrap.
