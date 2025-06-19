# 🛠️ Developer Documentation

Welcome developers! This section contains technical documentation for understanding, extending, and contributing to the Zwift Race Finder.

## 📁 Directory Structure

### `/architecture/`
Core technical design documents:
- System architecture
- Design decisions
- Component relationships

### `/testing/`
Comprehensive testing documentation:
- [Modern Testing Strategy](testing/MODERN_TESTING_STRATEGY.md) - Current approach
- Testing philosophy and guidelines
- Mutation testing techniques

### `/refactoring/`
Code improvement guides:
- [Best Practices](refactoring/BEST_PRACTICES.md) - Rust refactoring patterns
- Modern Rust enhancements
- Refactoring strategies

### `/api-research/`
Zwift API discoveries:
- API knowledge base (consolidated from multiple logs)
- Integration patterns
- Future API work

### `/active-plans/`
Current development work:
- [UOM Migration](active-plans/UOM_MIGRATION_PLAN.md) - Active migration to units of measure
- Other ongoing initiatives

### `/data-extraction/`
Data gathering techniques:
- Route data extraction methods
- Web scraping approaches

### `/integrations/`
External system integrations:
- Zwift offline mode
- Strava API
- Other services

## 🎯 Current Development Focus

### Active Projects
1. **UOM Migration** - Migrating to proper units of measure handling
2. **Racing Guides** - Expanding user-focused documentation
3. **API Research** - Discovering new Zwift endpoints

### Key Metrics
- **Prediction Accuracy**: 23.6% (target was 30%)
- **Test Coverage**: Comprehensive with mutation testing
- **Performance**: Sub-second predictions

## 🏗️ Architecture Overview

The system follows a clean architecture:
```
API Layer (Zwift, Strava)
    ↓
Data Layer (SQLite)
    ↓
Domain Logic (Rust)
    ↓
Prediction Engine
```

See [Architecture](../reference/ARCHITECTURE.md) for details.

## 🧪 Testing Philosophy

We follow a pragmatic testing approach:
- **Mutation Testing**: Ensures tests actually catch bugs
- **Regression Tests**: 151 real races validate accuracy
- **Property Tests**: For algorithmic correctness
- **No 100% Coverage Goal**: Quality over quantity

See [Modern Testing Strategy](testing/MODERN_TESTING_STRATEGY.md).

## 🔧 Development Setup

1. **Prerequisites**:
   - Rust (latest stable)
   - SQLite3
   - Git

2. **Build**:
   ```bash
   cargo build --release
   ```

3. **Test**:
   ```bash
   cargo test
   cargo mutants  # Mutation testing
   ```

## 📊 Key Algorithms

The prediction engine uses:
- **Dual-speed model**: Pack vs solo speeds
- **Drop probability**: Based on weight/elevation
- **Category calibration**: From regression testing

See [Algorithms](../reference/ALGORITHMS.md) for implementation details.

## 🤝 Contributing

1. **Find an Issue**: Check GitHub issues or [Active Plans](active-plans/)
2. **Discuss First**: Complex changes need discussion
3. **Follow Standards**: See [Rust Best Practices](refactoring/BEST_PRACTICES.md)
4. **Test Thoroughly**: Including mutation testing
5. **Document Changes**: Update relevant docs

## 📚 Essential Reading

For new contributors:
1. [Architecture](../reference/ARCHITECTURE.md) - System design
2. [Zwift Domain](../reference/ZWIFT_DOMAIN.md) - Key concepts  
3. [Project Wisdom](../PROJECT_WISDOM.md) - Lessons learned
4. [Testing Guide](../COMPREHENSIVE_TESTING_GUIDE.md) - Testing approach

## 🔍 Finding Information

- **How predictions work**: [Algorithms](../reference/ALGORITHMS.md)
- **Database schema**: [Database](../reference/DATABASE_SCHEMA.md)
- **API details**: [API Knowledge Base](api-research/API_KNOWLEDGE_BASE.md)
- **Historical decisions**: [Project History](../project-history/)

## 💡 Development Philosophy

- **Data-driven**: Decisions based on regression tests
- **Pragmatic**: Simple solutions preferred
- **User-focused**: Features that help racers
- **Transparent**: Extensive documentation

Questions? Check the architecture docs or explore the codebase!