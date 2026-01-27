# Code Style - Rust

## Nommage

| Type | Convention | Exemple |
|------|------------|---------|
| Crates | snake_case | `it_tools_core` |
| Modules | snake_case | `encoding` |
| Types | PascalCase | `Base64Options` |
| Functions | snake_case | `encode_base64` |
| Constants | SCREAMING_SNAKE | `MAX_INPUT_SIZE` |

## Structure Fichier

```rust
//! Module documentation

// Imports
use std::collections::HashMap;
use anyhow::Result;
use crate::error::ToolError;

// Constants
const MAX_SIZE: usize = 10 * 1024 * 1024;

// Types
#[derive(Debug, Error)]
pub enum EncodingError { ... }

// Structs
#[derive(Debug, Clone)]
pub struct Options { ... }

// Implementations
impl Default for Options { ... }

// Functions
pub fn encode(input: &str) -> Result<String> { ... }

// Tests
#[cfg(test)]
mod tests { ... }
```

## Error Handling

```rust
// ✅ BON
let value = option.ok_or(MyError::NotFound)?;
let data = parse(input).context("Failed to parse")?;

// ❌ MAUVAIS
let value = option.unwrap();
```

## Documentation

```rust
/// Encodes input to Base64.
///
/// # Arguments
/// * `input` - The string to encode
///
/// # Examples
/// ```
/// let result = encode("hello")?;
/// ```
pub fn encode(input: &str) -> Result<String> { ... }
```
