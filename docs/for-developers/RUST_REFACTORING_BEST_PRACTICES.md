# Rust Refactoring Best Practices

This guide provides comprehensive Rust-specific refactoring guidelines, patterns, and anti-patterns to help you safely and effectively restructure Rust code.

## Table of Contents
1. [Core Principles](#core-principles)
2. [Ownership and Borrowing During Refactoring](#ownership-and-borrowing-during-refactoring)
3. [Module System Refactoring](#module-system-refactoring)
4. [Trait and Generic Refactoring](#trait-and-generic-refactoring)
5. [Common Refactoring Mistakes](#common-refactoring-mistakes)
6. [Performance-Preserving Techniques](#performance-preserving-techniques)
7. [Testing During Refactoring](#testing-during-refactoring)
8. [Incremental Refactoring Strategies](#incremental-refactoring-strategies)
9. [Macro Refactoring](#macro-refactoring)
10. [Async/Await Patterns](#asyncawait-patterns)
11. [API Compatibility](#api-compatibility)
12. [Unsafe Code Refactoring](#unsafe-code-refactoring)

## Core Principles

### 1. Rust-Specific Refactoring Philosophy
- **Leverage the Type System**: Use Rust's type system to make illegal states unrepresentable
- **Compile-Time Guarantees**: Refactor to move runtime checks to compile-time where possible
- **Zero-Cost Abstractions**: Ensure refactoring doesn't introduce runtime overhead
- **Explicit Over Implicit**: Make ownership, lifetimes, and error handling explicit

### 2. YAGNI (You Aren't Going to Need It)
Rust's features often eliminate the need for complex design patterns. Don't over-engineer:
```rust
// Bad: Over-abstracted
trait WidgetFactory {
    fn create_widget(&self) -> Box<dyn Widget>;
}

// Good: Direct and simple
fn create_widget() -> Widget {
    Widget::new()
}
```

## Ownership and Borrowing During Refactoring

### 1. Extract Method with Ownership Considerations
When extracting methods, carefully consider ownership:

```rust
// Before refactoring
fn process_data(data: Vec<String>) {
    let filtered: Vec<_> = data.iter()
        .filter(|s| s.len() > 5)
        .cloned()
        .collect();
    // More processing...
}

// After: Consider borrowing vs ownership
fn filter_long_strings(data: &[String]) -> Vec<String> {
    data.iter()
        .filter(|s| s.len() > 5)
        .cloned()
        .collect()
}

fn process_data(data: Vec<String>) {
    let filtered = filter_long_strings(&data);
    // Original data still available
}
```

### 2. Refactoring to Reduce Cloning
Identify unnecessary clones and refactor to use references:

```rust
// Anti-pattern: Cloning to satisfy borrow checker
fn calculate_total(items: Vec<Item>) -> f64 {
    let cloned_items = items.clone(); // Unnecessary!
    cloned_items.iter().map(|i| i.price).sum()
}

// Better: Use references
fn calculate_total(items: &[Item]) -> f64 {
    items.iter().map(|i| i.price).sum()
}
```

### 3. Lifetime Simplification
Refactor to eliminate unnecessary lifetime annotations:

```rust
// Before: Explicit lifetimes
fn find_item<'a, 'b>(name: &'a str, items: &'b [Item]) -> Option<&'b Item> {
    items.iter().find(|item| item.name == name)
}

// After: Lifetime elision
fn find_item(name: &str, items: &[Item]) -> Option<&Item> {
    items.iter().find(|item| item.name == name)
}
```

## Module System Refactoring

### 1. Progressive Module Extraction
Start with inline modules, then extract to files:

```rust
// Step 1: Inline module
mod database {
    pub struct Connection { /* ... */ }
    pub fn connect() -> Connection { /* ... */ }
}

// Step 2: Extract to file (database.rs)
// main.rs:
mod database;

// database.rs:
pub struct Connection { /* ... */ }
pub fn connect() -> Connection { /* ... */ }
```

### 2. Visibility Refactoring
Use the principle of least visibility:

```rust
// Before: Everything public
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// After: Encapsulation with getters
pub struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    pub fn id(&self) -> u64 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn email(&self) -> &str { &self.email }
}
```

### 3. Re-export Pattern
Simplify public API through strategic re-exports:

```rust
// lib.rs
mod errors;
mod models;
mod handlers;

// Re-export commonly used items
pub use errors::{Error, Result};
pub use models::User;
pub use handlers::process_request;
```

## Trait and Generic Refactoring

### 1. Extract Trait from Concrete Implementation
```rust
// Before: Concrete implementation
struct FileLogger {
    path: PathBuf,
}

impl FileLogger {
    fn log(&mut self, message: &str) {
        // Write to file
    }
}

// After: Extract trait
trait Logger {
    fn log(&mut self, message: &str);
}

struct FileLogger {
    path: PathBuf,
}

impl Logger for FileLogger {
    fn log(&mut self, message: &str) {
        // Write to file
    }
}
```

### 2. Generics Over Concrete Types
```rust
// Before: Specific to Vec
fn process_numbers(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

// After: Generic over iterables
fn process_numbers<I>(numbers: I) -> i32 
where
    I: IntoIterator<Item = i32>,
{
    numbers.into_iter().sum()
}
```

### 3. Associated Types vs Generic Parameters
```rust
// Before: Generic parameter
trait Container<T> {
    fn get(&self) -> &T;
}

// After: Associated type (when there's only one logical type per implementation)
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}
```

## Common Refactoring Mistakes

### 1. Over-Using `Arc<Mutex<T>>`
```rust
// Anti-pattern: Shared mutable state everywhere
struct App {
    state: Arc<Mutex<AppState>>,
}

// Better: Message passing or state machines
enum Message {
    UpdateUser(User),
    DeleteUser(u64),
}

struct App {
    receiver: mpsc::Receiver<Message>,
}
```

### 2. Ignoring Error Context
```rust
// Bad: Losing error context
fn read_config() -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string("config.json")?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// Good: Preserving context
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = fs::read_to_string("config.json")
        .context("Failed to read config file")?;
    let config = serde_json::from_str(&content)
        .context("Failed to parse config JSON")?;
    Ok(config)
}
```

### 3. Premature Async
```rust
// Bad: Making everything async unnecessarily
async fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b  // No actual async operations!
}

// Good: Only async when needed
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}
```

## Performance-Preserving Techniques

### 1. Iterator Chain Preservation
```rust
// Before: Collecting intermediate results
let numbers: Vec<i32> = get_numbers();
let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
let sum: i32 = doubled.iter().sum();

// After: Lazy evaluation
let sum: i32 = get_numbers()
    .iter()
    .map(|n| n * 2)
    .sum();
```

### 2. Const Generics for Performance
```rust
// Before: Runtime size
struct Buffer {
    data: Vec<u8>,
}

// After: Compile-time size
struct Buffer<const N: usize> {
    data: [u8; N],
}
```

### 3. Inlining Hot Paths
```rust
// Add inline hints for performance-critical small functions
#[inline]
fn is_valid(&self) -> bool {
    self.value > 0 && self.value < 100
}
```

## Testing During Refactoring

### 1. Property-Based Testing for Refactoring
```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn refactored_equals_original(input: Vec<i32>) {
            let original_result = original_implementation(&input);
            let refactored_result = refactored_implementation(&input);
            assert_eq!(original_result, refactored_result);
        }
    }
}
```

### 2. Snapshot Testing
```rust
#[test]
fn test_output_format() {
    let result = format_output(&test_data());
    insta::assert_snapshot!(result);
}
```

### 3. Regression Test Suite
```rust
// Create a dedicated module for regression tests
#[cfg(test)]
mod regression_tests {
    use super::*;
    
    #[test]
    fn issue_123_edge_case() {
        // Test that previously broken behavior stays fixed
    }
}
```

## Incremental Refactoring Strategies

### 1. Parallel Implementation
Keep old and new implementations side-by-side:

```rust
mod legacy {
    pub fn process_data(data: &[u8]) -> Result<Output, Error> {
        // Original implementation
    }
}

mod v2 {
    pub fn process_data(data: &[u8]) -> Result<Output, Error> {
        // Refactored implementation
    }
}

// Gradual migration
fn process_with_fallback(data: &[u8]) -> Result<Output, Error> {
    v2::process_data(data).or_else(|_| legacy::process_data(data))
}
```

### 2. Feature Flag Migration
```rust
#[cfg(feature = "new-parser")]
mod parser {
    pub use crate::v2::parser::*;
}

#[cfg(not(feature = "new-parser"))]
mod parser {
    pub use crate::legacy::parser::*;
}
```

### 3. Deprecation Strategy
```rust
#[deprecated(since = "0.2.0", note = "Use `new_function` instead")]
pub fn old_function() {
    new_function()
}
```

## Macro Refactoring

### 1. Macro to Function
```rust
// Before: Macro for simple logic
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

// After: Generic function
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### 2. Procedural Macro Extraction
```rust
// Extract complex macros to procedural macros in separate crate
// my_macros/src/lib.rs
#[proc_macro]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Implementation
}
```

### 3. Macro Hygiene
```rust
// Bad: Unhygienic macro
macro_rules! with_temp {
    ($val:expr, $body:expr) => {
        let temp = $val;  // 'temp' might conflict!
        $body
    };
}

// Good: Hygienic with unique names
macro_rules! with_temp {
    ($val:expr, $body:expr) => {{
        let _temp_value = $val;
        $body
    }};
}
```

## Async/Await Patterns

### 1. Sync to Async Migration
```rust
// Step 1: Create async version alongside sync
impl Client {
    pub fn fetch_data(&self) -> Result<Data> {
        // Synchronous implementation
    }
    
    pub async fn fetch_data_async(&self) -> Result<Data> {
        // Async implementation
    }
}

// Step 2: Deprecate sync version
#[deprecated(note = "Use fetch_data_async instead")]
pub fn fetch_data(&self) -> Result<Data> {
    // ...
}
```

### 2. Async Trait Refactoring
```rust
// Using async-trait for now
use async_trait::async_trait;

#[async_trait]
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
}

// Future: Native async traits (when stabilized)
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
}
```

### 3. Spawning and Task Management
```rust
// Before: Unstructured spawning
tokio::spawn(async {
    process_item(item).await;
});

// After: Structured concurrency
let handle = tokio::spawn(async move {
    process_item(item).await
});

// Ensure completion
handle.await?;
```

## API Compatibility

### 1. Non-Breaking Additions
```rust
// Mark structs/enums as non-exhaustive
#[non_exhaustive]
pub struct Config {
    pub host: String,
    pub port: u16,
    // Can add fields later without breaking
}

#[non_exhaustive]
pub enum Error {
    Network(String),
    Parse(String),
    // Can add variants later
}
```

### 2. Builder Pattern for Extensibility
```rust
pub struct RequestBuilder {
    // Private fields
}

impl RequestBuilder {
    pub fn new() -> Self { /* ... */ }
    pub fn header(mut self, key: &str, value: &str) -> Self { /* ... */ }
    pub fn timeout(mut self, duration: Duration) -> Self { /* ... */ }
    pub fn build(self) -> Request { /* ... */ }
}
```

### 3. Sealed Traits
```rust
mod private {
    pub trait Sealed {}
}

pub trait MyTrait: private::Sealed {
    // Users can't implement this trait
}

impl private::Sealed for MyType {}
impl MyTrait for MyType {}
```

## Unsafe Code Refactoring

### 1. Minimize Unsafe Blocks
```rust
// Before: Large unsafe block
unsafe {
    let ptr = data.as_ptr();
    let len = data.len();
    // Many lines of code...
    process_raw_data(ptr, len);
}

// After: Minimal unsafe surface
fn process_data(data: &[u8]) {
    // Safe preprocessing
    let ptr = data.as_ptr();
    let len = data.len();
    
    // Only unsafe where necessary
    unsafe {
        process_raw_data(ptr, len);
    }
    
    // Safe postprocessing
}
```

### 2. Safe Wrappers
```rust
// Wrap unsafe operations in safe abstractions
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl SafeBuffer {
    pub fn new(capacity: usize) -> Self {
        let ptr = unsafe {
            std::alloc::alloc(Layout::array::<u8>(capacity).unwrap())
        };
        Self { ptr, len: 0, capacity }
    }
    
    // Safe methods that encapsulate unsafe operations
    pub fn push(&mut self, value: u8) {
        assert!(self.len < self.capacity);
        unsafe {
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.ptr,
                Layout::array::<u8>(self.capacity).unwrap()
            );
        }
    }
}
```

### 3. Document Safety Invariants
```rust
/// # Safety
/// 
/// - `ptr` must be valid for reads of `len` bytes
/// - `ptr` must be properly aligned
/// - The memory must not be mutated during the call
unsafe fn read_raw_data(ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(ptr, len).to_vec()
}
```

## Refactoring Checklist

Before refactoring:
- [ ] Ensure comprehensive test coverage
- [ ] Run benchmarks to establish baseline performance
- [ ] Document current behavior
- [ ] Create a refactoring plan

During refactoring:
- [ ] Make small, incremental changes
- [ ] Run tests after each change
- [ ] Keep commits atomic and well-described
- [ ] Preserve API compatibility when possible

After refactoring:
- [ ] Run full test suite
- [ ] Compare benchmark results
- [ ] Update documentation
- [ ] Review for any unsafe code or panics
- [ ] Check for unnecessary allocations or clones

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Effective Rust](https://effective-rust.com/)
- [Refactoring by Martin Fowler](https://refactoring.com/) (general principles applicable to Rust)