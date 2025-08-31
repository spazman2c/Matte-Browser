# Matte Browser - Development Guidelines

This document provides guidelines for contributing to the Matte Browser project, including coding standards, testing practices, and development workflows.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Coding Standards](#coding-standards)
3. [Testing Strategy](#testing-strategy)
4. [Performance Guidelines](#performance-guidelines)
5. [Security Guidelines](#security-guidelines)
6. [Documentation Standards](#documentation-standards)
7. [Git Workflow](#git-workflow)
8. [Code Review Process](#code-review-process)
9. [Release Process](#release-process)

## Getting Started

### Prerequisites

- Rust 1.75.0 or later
- Cargo (comes with Rust)
- CMake 3.28.1 or later
- Platform-specific development tools
- Git

### Development Environment Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/matte-browser.git
   cd matte-browser
   ```

2. **Install Rust toolchain**
   ```bash
   rustup default 1.75.0
   rustup component add rustfmt clippy
   ```

3. **Install pre-commit hooks**
   ```bash
   pip install pre-commit
   pre-commit install
   ```

4. **Build the project**
   ```bash
   cargo build
   ```

5. **Run tests**
   ```bash
   cargo test
   ```

### IDE Setup

#### VS Code
- Install Rust extension
- Install rust-analyzer extension
- Configure settings for Rust development

#### IntelliJ IDEA / CLion
- Install Rust plugin
- Configure Rust toolchain
- Set up run configurations

## Coding Standards

### Rust Guidelines

#### Code Style
- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `rustfmt` for automatic formatting
- Maximum line length: 100 characters
- Use 4 spaces for indentation

#### Naming Conventions
```rust
// Modules and crates: snake_case
mod html_parser;
mod css_engine;

// Types and traits: PascalCase
struct HtmlParser;
trait ParserTrait;

// Functions and variables: snake_case
fn parse_html(input: &str) -> Result<Document>;

// Constants: SCREAMING_SNAKE_CASE
const MAX_BUFFER_SIZE: usize = 1024;

// Static variables: SCREAMING_SNAKE_CASE
static DEFAULT_CONFIG: Config = Config::new();
```

#### Error Handling
```rust
// Use Result<T, E> for fallible operations
pub fn parse_url(input: &str) -> Result<Url, ParseError> {
    // Implementation
}

// Use Option<T> for nullable values
pub fn find_element_by_id(&self, id: &str) -> Option<&Element> {
    // Implementation
}

// Custom error types
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URL format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
}
```

#### Documentation
```rust
/// Parses HTML content and returns a DOM tree.
///
/// # Arguments
///
/// * `input` - The HTML string to parse
///
/// # Returns
///
/// A `Result` containing the parsed `Document` or a `ParseError`.
///
/// # Examples
///
/// ```
/// use matte_browser::dom::HtmlParser;
///
/// let html = "<html><body><h1>Hello</h1></body></html>";
/// let document = HtmlParser::parse(html)?;
/// assert_eq!(document.title(), "Hello");
/// ```
pub fn parse(input: &str) -> Result<Document, ParseError> {
    // Implementation
}
```

### C++ Guidelines

#### Code Style
- Follow the [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html)
- Use `clang-format` for automatic formatting
- Maximum line length: 100 characters
- Use 2 spaces for indentation

#### Modern C++ Features
```cpp
// Use smart pointers instead of raw pointers
std::unique_ptr<Renderer> renderer = std::make_unique<Renderer>();

// Use auto for type deduction
auto result = calculate_performance_metrics();

// Use range-based for loops
for (const auto& element : elements) {
    process_element(element);
}

// Use std::optional for nullable values
std::optional<std::string> get_user_preference(const std::string& key);

// Use std::variant for sum types
std::variant<Success, Error> perform_operation();
```

#### Error Handling
```cpp
// Use exceptions for exceptional cases
class ParseException : public std::exception {
public:
    explicit ParseException(const std::string& message) : message_(message) {}
    const char* what() const noexcept override { return message_.c_str(); }
private:
    std::string message_;
};

// Use std::expected (C++23) or similar for expected errors
std::expected<Document, ParseError> parse_html(const std::string& input);
```

### General Guidelines

#### Code Organization
- Keep functions small and focused (max 50 lines)
- Use meaningful variable and function names
- Avoid deep nesting (max 3 levels)
- Extract complex logic into separate functions

#### Comments
- Write self-documenting code
- Comment on "why" not "what"
- Use TODO comments for future improvements
- Document complex algorithms

#### Performance
- Profile before optimizing
- Use appropriate data structures
- Avoid unnecessary allocations
- Consider memory layout and cache locality

## Testing Strategy

### Test Types

#### Unit Tests
- Test individual functions and methods
- Use descriptive test names
- Test edge cases and error conditions
- Keep tests fast and isolated

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_html() {
        let html = "<html><body><h1>Test</h1></body></html>";
        let result = HtmlParser::parse(html);
        assert!(result.is_ok());
        
        let document = result.unwrap();
        assert_eq!(document.title(), "Test");
    }

    #[test]
    fn test_parse_invalid_html() {
        let html = "<html><body><h1>Unclosed";
        let result = HtmlParser::parse(html);
        assert!(result.is_err());
    }
}
```

#### Integration Tests
- Test component interactions
- Test end-to-end workflows
- Use realistic test data
- Test error handling across components

```rust
#[cfg(test)]
mod integration_tests {
    use crate::browser::BrowserApp;
    use crate::dom::HtmlParser;
    use crate::network::HttpClient;

    #[tokio::test]
    async fn test_page_load_workflow() {
        let mut browser = BrowserApp::new().await.unwrap();
        let tab_id = browser.create_tab(1, Some("https://example.com".to_string())).await.unwrap();
        
        // Test the complete page load workflow
        // ...
    }
}
```

#### Performance Tests
- Benchmark critical code paths
- Test memory usage
- Monitor performance regressions
- Use realistic workloads

```rust
#[cfg(test)]
mod benchmarks {
    use criterion::{criterion_group, criterion_main, Criterion};
    use crate::dom::HtmlParser;

    fn benchmark_html_parsing(c: &mut Criterion) {
        let html = include_str!("../test_data/large_page.html");
        
        c.bench_function("parse_large_html", |b| {
            b.iter(|| HtmlParser::parse(html))
        });
    }

    criterion_group!(benches, benchmark_html_parsing);
    criterion_main!(benches);
}
```

### Test Data
- Use realistic test data
- Include edge cases and error conditions
- Keep test data in separate files
- Use fixtures for complex test scenarios

### Test Coverage
- Aim for 80%+ code coverage
- Focus on critical code paths
- Test error handling paths
- Use coverage tools to identify gaps

## Performance Guidelines

### Memory Management

#### Rust
- Use appropriate data structures
- Minimize allocations in hot paths
- Use `Cow<T>` for borrowed/owned data
- Profile memory usage

```rust
// Use Cow for efficient string handling
use std::borrow::Cow;

fn process_text(text: Cow<str>) -> String {
    if text.contains("special") {
        text.into_owned()
    } else {
        text.into_owned()
    }
}
```

#### C++
- Use RAII for resource management
- Prefer smart pointers over raw pointers
- Use move semantics where appropriate
- Avoid unnecessary copies

```cpp
// Use move semantics for efficiency
std::vector<std::string> process_strings(std::vector<std::string>&& strings) {
    std::vector<std::string> result;
    result.reserve(strings.size());
    
    for (auto& str : strings) {
        result.push_back(std::move(str));
    }
    
    return result;
}
```

### Algorithm Efficiency
- Choose appropriate algorithms
- Consider time and space complexity
- Profile performance-critical code
- Use parallel algorithms where beneficial

### Caching
- Cache frequently accessed data
- Use appropriate cache invalidation
- Consider cache locality
- Profile cache performance

## Security Guidelines

### Input Validation
- Validate all user inputs
- Use whitelist validation where possible
- Sanitize data before processing
- Use type-safe APIs

```rust
// Validate URLs before processing
pub fn validate_url(url: &str) -> Result<Url, ValidationError> {
    let parsed = Url::parse(url)?;
    
    // Whitelist allowed protocols
    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        _ => Err(ValidationError::UnsupportedProtocol(parsed.scheme().to_string())),
    }
}
```

### Memory Safety
- Use Rust's ownership system
- Avoid unsafe code unless necessary
- Document unsafe code thoroughly
- Use static analysis tools

### Sandboxing
- Respect process boundaries
- Validate IPC messages
- Use privilege brokering
- Implement proper access controls

### Cryptography
- Use established cryptographic libraries
- Don't implement custom crypto
- Use secure random number generators
- Validate certificates properly

## Documentation Standards

### Code Documentation
- Document all public APIs
- Use clear and concise language
- Include examples where helpful
- Keep documentation up-to-date

### Architecture Documentation
- Document design decisions
- Explain component interactions
- Include diagrams where helpful
- Document trade-offs and constraints

### User Documentation
- Write clear user guides
- Include troubleshooting sections
- Provide configuration examples
- Keep documentation current

## Git Workflow

### Branch Strategy
- `main`: Production-ready code
- `develop`: Integration branch
- `feature/*`: New features
- `bugfix/*`: Bug fixes
- `hotfix/*`: Critical fixes

### Commit Messages
- Use conventional commit format
- Write clear, descriptive messages
- Reference issues where applicable
- Keep commits atomic

```
feat(dom): add HTML5 parsing support

- Implement HTML5 tokenizer
- Add DOM tree construction
- Support self-closing tags
- Add comprehensive test coverage

Fixes #123
```

### Pull Request Process
1. Create feature branch from `develop`
2. Make changes with clear commits
3. Write comprehensive PR description
4. Request appropriate reviewers
5. Address review feedback
6. Merge after approval

## Code Review Process

### Review Checklist
- [ ] Code follows style guidelines
- [ ] Tests are included and pass
- [ ] Documentation is updated
- [ ] Performance impact is considered
- [ ] Security implications are reviewed
- [ ] Error handling is appropriate

### Review Guidelines
- Be constructive and respectful
- Focus on the code, not the person
- Ask questions to understand context
- Suggest improvements when possible
- Approve only when satisfied

### Review Roles
- **Primary Reviewer**: Deep technical review
- **Security Reviewer**: Security-focused review
- **Performance Reviewer**: Performance impact assessment
- **Documentation Reviewer**: Documentation quality

## Release Process

### Versioning
- Follow semantic versioning (MAJOR.MINOR.PATCH)
- Increment version appropriately
- Update changelog with each release
- Tag releases in Git

### Release Checklist
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Changelog is complete
- [ ] Version numbers are updated
- [ ] Release notes are written
- [ ] Build artifacts are created
- [ ] Release is tagged

### Release Types
- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

### Deployment
- Test releases thoroughly
- Deploy to staging first
- Monitor for issues
- Rollback plan ready

## Contributing

### Getting Help
- Check existing documentation
- Search existing issues
- Ask in discussions
- Join community channels

### Reporting Issues
- Use issue templates
- Provide detailed information
- Include reproduction steps
- Attach relevant logs

### Suggesting Features
- Check existing feature requests
- Provide use case details
- Consider implementation complexity
- Discuss with maintainers

This guide ensures consistent development practices and high code quality across the Matte Browser project.
