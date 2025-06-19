# 📚 Reference Documentation

Core technical documentation for the Zwift Race Finder project.

## 📖 Documents

### [Algorithms](ALGORITHMS.md)
The prediction engine's core algorithms:
- Dual-speed model (pack vs solo)
- Drop probability calculations
- Route difficulty assessment
- Category-based calibration

### [Architecture](ARCHITECTURE.md)
System design and structure:
- Component relationships
- Data flow
- Technology choices
- Design decisions

### [Database Schema](DATABASE_SCHEMA.md)
SQLite database structure:
- Table definitions
- Relationships
- Data types
- Indexes

### [Physical Stats](PHYSICAL_STATS.md)
How physical attributes affect performance:
- Height and aerodynamics
- Weight and climbing
- Power-to-weight ratios
- Zwift physics implications

### [Zwift Domain](ZWIFT_DOMAIN.md)
Key concepts and terminology:
- Racing categories
- Event types
- Route characteristics
- Zwift-specific terms

## 🔗 Quick Links

### For Implementation Details
- **Prediction Logic**: See [Algorithms](ALGORITHMS.md)
- **System Design**: See [Architecture](ARCHITECTURE.md)
- **Data Storage**: See [Database Schema](DATABASE_SCHEMA.md)

### For Domain Understanding  
- **Zwift Concepts**: See [Zwift Domain](ZWIFT_DOMAIN.md)
- **Physics Impact**: See [Physical Stats](PHYSICAL_STATS.md)

## 📊 Key Technical Specs

### Performance
- Prediction time: <1 second
- Database queries: Optimized with indexes
- Memory usage: Minimal (SQLite-based)

### Accuracy
- Mean error: 23.6%
- Based on: 151 real races
- Validation: Regression testing

### Data Sources
- Zwift Public API
- Strava API (for real times)
- Community route database

## 🎯 Design Principles

1. **Empirical**: Based on real race data
2. **Simple**: Straightforward algorithms
3. **Fast**: Sub-second predictions
4. **Accurate**: Within 24% on average

These reference documents form the technical foundation of the project.