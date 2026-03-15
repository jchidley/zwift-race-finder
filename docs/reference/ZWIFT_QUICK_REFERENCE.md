# Zwift Racing Quick Reference

Lookup tables for in-race decisions. For the reasoning behind these numbers, see [Zwift Racing Tactics](ZWIFT_RACING_TACTICS.md) and [Zwift Physics](ZWIFT_PHYSICS.md).

## Group Size Decision

| Group Size | Draft | Success vs Blob | Power Needed | Best For |
|------------|-------|-----------------|--------------|----------|
| Solo | 0% | <5% | 130–150% of blob | Climbs >7%, final 1–2 km |
| 2 riders | 25% | 10% | 120–130% | Emergency bridge |
| **3–5 riders** | **~35%** | **20–30%** | **110–120%** | **Coordinated attacks** |
| 6–10 riders | ~35% | 15% | 105–115% | Selection moves |
| 20+ riders | ~35% + blob | N/A | 75–85% (good position) | Main pack |

## Terrain Strategy

| Your Strength | Flat | Rolling | Mountain | Mixed |
|---------------|------|---------|----------|-------|
| Climber | Stay in blob | 3–5 rider break | Solo on >7% | Attack terrain changes |
| All-rounder | Stay in blob | Opportunistic break | Small group | Position + timing |
| Sprinter | Blob until final | Blob, sprint from rises | Survive to sprint | Blob, save matches |
| Time trialist | 3–5 rider break | 3–5 rider break | Small group | Breakaway |

## Gap Assessment

| Gap to Blob | Solo Survival | 3–5 Riders | 6+ Riders |
|-------------|---------------|------------|-----------|
| <10 sec | Bridge hard | Bridge easily | Comfortable |
| 10–30 sec | 1–2 min | 5–10 min | Probably safe |
| >30 sec | Climb only | Often survives | Usually safe |

## Blob Position Checklist

| When | Target Position | Action |
|------|-----------------|--------|
| Steady sections | Rows 5–15 | Conserve, follow wheels |
| 30 sec before climb | Top 10 | Micro-sprints forward |
| 1 km to finish | Top 5 | Committed move up |
| 500 m to finish | Top 3 | Launch or follow |

## PowerUp Timing

| PowerUp | Save For | Use When |
|---------|----------|----------|
| Feather (-10% weight) | Steepest gradient | Decisive climb or to avoid being dropped |
| Aero Helmet | Sprint finish | Final 300–500 m |
| Draft Boost (+50%) | Bridging | When catching a group |

## Power Zone Guide

| Situation | FTP % | Sustainable |
|-----------|-------|-------------|
| Race start surge | 150–200% | 1 minute |
| Solo attack | 120–130% | 5–10 min |
| Small break (3–5) | 105–115% | 15–25 min |
| Blob (good position) | 70–85% | Full race |
| Blob (poor position) | 90–95% | Full race |

## Decision Flowchart

```
Am I in the main blob (20+ riders)?
├─ YES → Am I in the front half?
│  ├─ YES → Maintain, watch for moves
│  └─ NO → Move up NOW (micro-sprints)
└─ NO → How big is the gap?
   ├─ <10 sec → Bridge solo
   ├─ 10–30 sec → Find 2–3 allies
   └─ >30 sec → Accept it, maximise current position

Is a break forming?
├─ 2 riders → Usually ignore
├─ 3–5 riders, similar w/kg → Consider joining
└─ 6+ riders → This is a split, must respond

Should I attack?
├─ Climb >7% → Solo viable
├─ 3–5 willing riders → Coordinate
├─ Final 2 km → Solo possible
└─ Otherwise → Stay in blob
```
