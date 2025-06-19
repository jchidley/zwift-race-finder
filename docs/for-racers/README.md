# 🚴 Zwift Racing Optimization Guide

Welcome racers! This section helps you go faster in Zwift races using the race finder tool and understanding Zwift's game mechanics.

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

### [Draft Strategies](DRAFT_STRATEGIES.md) *(Early Draft)*
Maximizing the 24-33% power savings:
- Positioning in the pack
- When draft matters most
- Field size effects

### [Route Tactics](ROUTE_TACTICS.md) *(Basic Ideas)*
Route-specific power distribution:
- Climbing vs flat strategies
- Where to make your moves
- Route knowledge advantages

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