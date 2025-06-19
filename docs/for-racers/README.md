# 🚴 Zwift Racing Optimization Guide

Welcome racers! This section helps you go faster in Zwift races using the race finder tool and understanding Zwift's game mechanics.

### 📋 [Racing Research Summary](RACING_RESEARCH_SUMMARY.md) *(Start Here!)*
Quick reference to all key findings - draft percentages, attack timing, group sizes, and essential tactics in one place.

## Core Principle: Power is Your Only Variable

Once a race starts, you can only control one thing: **your power output**. Everything else is fixed:
- ❌ Height (fixed at race start)
- ❌ Weight (fixed at race start)  
- ❌ Equipment (fixed at race start)
- ✅ **Power (watts)** - The ONLY thing you control!

This means success comes from optimizing how you use your available watts.

## 📚 Racing Guides (Early Drafts)

These guides are in early development and will expand based on user feedback and racing insights:

### [Power Optimization](POWER_OPTIMIZATION.md) *(Initial Concepts)*
Managing your watts effectively:
- When to push vs when to save
- Understanding watts/kg vs raw watts
- Pacing strategies for your fitness level

### [Draft Strategies](DRAFT_STRATEGIES.md) *(Updated with Research)*
Maximizing the 25-35% power savings:
- Verified draft percentages by position
- Binary draft model explained
- Group size effects on speed

### [Route Tactics](ROUTE_TACTICS.md) *(Updated with Attack Timing)*
Route-specific power distribution:
- Critical attack timing (different from road!)
- Route-specific blob dynamics
- Climbing mid-climb, not at base
- Pre-crest acceleration for descents

### [Zwift as a Game](ZWIFT_AS_GAME.md) *(Philosophy Draft)*
Understanding and exploiting game mechanics:
- Why Zwift isn't "unfair" - it's consistent
- Working with the physics engine
- Mental approach to virtual racing

### [Category Racing](CATEGORY_RACING.md) *(Outline Only)*
Optimizing for your category:
- Category-specific tactics
- Working with your strengths
- Common category mistakes

## 🆕 Advanced Racing Guides

### [Blob Dynamics Mastery](BLOB_DYNAMICS_MASTERY.md) *(New - Comprehensive)*
Master Zwift's unique pack racing dynamics:
- Academic research vs Zwift reality
- Optimal positioning strategies
- Breakaway success factors
- Bridging tactics

### [Blob Size Science](BLOB_SIZE_SCIENCE.md) *(New - Research-Based)*
Understanding group size effects:
- Real-world aerodynamics research
- Zwift's implementation
- Optimal breakaway sizes
- The "blob effect" explained

### [Breakaway Size Strategy](BREAKAWAY_SIZE_STRATEGY.md) *(New - Practical)*
When and how to attack:
- Solo vs group breakaways
- Power requirements by group size
- Success rates and timing
- PowerUp coordination

### [Group Size Matrix](GROUP_SIZE_MATRIX.md) *(New - Quick Reference)*
Quick lookup for tactical decisions:
- Group size comparison table
- Terrain-specific recommendations
- Power requirements
- Decision flowchart

## 🔄 Real World vs Zwift Comparisons

### [Real vs Virtual Physics](REAL_VS_VIRTUAL_PHYSICS.md) *(New - Comprehensive)*
Understanding the physics differences:
- What transfers from road racing
- What requires complete rethinking
- Why Zwift is "crit racing at 3x speed"
- Mental adaptations required

### [Zwift vs Road Racing](ZWIFT_VS_ROAD_RACING.md) *(New - Tactical Focus)*
Critical tactical differences:
- Attack timing completely different
- Binary vs gradual draft
- PowerUps as tactical layer
- Common road racer mistakes in Zwift

## 🎯 Using the Race Finder Tool

The tool helps you find races that match your available time and fitness:

```bash
# Find races around 2 hours for your category
zwift-race-finder --duration 120 --tolerance 30

# Find shorter races for intense efforts  
zwift-race-finder --duration 30 --tolerance 15

# Show your current Racing Score category
zwift-race-finder --show-score
```

## 🎮 Key Insights: It's a Game, Not a Simulation

1. **Consistent Rules**: Every rider faces the same physics engine
2. **Learn the System**: Understanding > Complaining
3. **Optimize Within Constraints**: Work with the mechanics, not against them

## 📈 Current Tool Performance

- **Prediction Accuracy**: 23.6% mean error
- **Based on**: 151 real races
- **Factors in**: Pack dynamics, route elevation, category speeds

## 🚀 Quick Start

1. Find a race: `zwift-race-finder`
2. Note the predicted duration
3. Plan your power strategy
4. Focus on pack position and timing
5. Use your watts wisely!

## 📝 Note on Documentation

These racing guides are living documents in early stages. As we gather more insights from racers and analyze more data, they will expand with:
- Specific workout suggestions
- Advanced pack dynamics
- Category-specific strategies
- Power profile optimization

Have racing insights to share? Contributions welcome!