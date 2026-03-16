# Decisions

## Skips
<!-- Format: - Phase N (cluster): reason. User confirmed: yes/no -->
- Phase 5 (all): Specs not needed — only change is YAGNI deletion (no behavioural spec to preserve)
- Phase 8 (all): Rust — compiler enforces types. Auto-skip per skill rules.
- Phase 9 (all): No naming issues found during analysis. Skip.
- Phase 10 (all): No moves needed — dead code removal only. No restructuring.
- DRY extraction: Low priority. Route name extraction (10×) and distance conversion (10×) are single expressions deeply embedded in branching logic. Risk of behaviour change exceeds benefit.

## Bugs Found
<!-- Format: - description. Decision: fix-before|fix-after|defer. User confirmed: yes/no -->
- `event_display.rs:578` uses `/1000.0` instead of `/METERS_PER_KILOMETER`. Functionally identical (constant is 1000.0) but inconsistent. Decision: fix-during (trivial, same commit as YAGNI cleanup).

## DRY Extractions
<!-- Format: - extracted `helperName` replacing pattern `oldPattern`. Equivalence: [assertion about edge cases] -->

## Warnings Acknowledged
<!-- Format: - warning-key: description (date) -->
