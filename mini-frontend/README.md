# Mini Compiler Frontend

A lightweight compiler frontend written in Rust that implements **lexical analysis** and **parsing** for a minimal programming language.

## Project Overview

This project builds the foundational components of a compiler:

1. **Lexer** – Tokenizes source code into meaningful tokens
2. **Parser** – Builds an Abstract Syntax Tree (AST) from tokens
3. **AST** – Represents program structure in a hierarchical format

## Architecture

### File Structure

```
mini-frontend/
├── src/
│   ├── main.rs       # Entry point; accepts source code as CLI argument
│   ├── lexer.rs      # Tokenization stage (uses logos crate)
│   ├── parser.rs     # Parsing stage (uses pest crate)
│   ├── ast.rs        # AST node definitions
│   └── errors.rs     # Error handling
├── grammar.pest      # PEG grammar rules for parser
├── examples/
│   └── sample.mini   # Example source code
└── Cargo.toml        # Project dependencies
```

## Language Features

The mini-language supports:

- **Variable declarations**: `let x = 10;`
- **Binary operations**: Addition (`+`) and multiplication (`*`)
- **Parenthesized expressions**: `(1 + 2) * 3`
- **Identifiers and numbers**: Alphanumeric variable names and numeric literals

### Example Program

```rust
let x = 1 + 2;
let y = x + 3;
x + y;
```

## Components

### 1. Lexer (`src/lexer.rs`)

Converts raw source text into a stream of tokens using the **logos** crate.

**Supported Tokens:**
- Keywords: `let`
- Operators: `=`, `+`, `*`, `;`
- Delimiters: `(`, `)`
- Identifiers: Variable names
- Numbers: Unsigned 32-bit integers
- Whitespace: Automatically skipped

**Function:** `pub fn lex(input: &str) -> Result<Vec<Token>, String>`

### 2. Parser (`src/parser.rs`)

Parses token streams into an AST using the **pest** PEG parser.

**Grammar Rules** (from `grammar.pest`):
- `program` – Root rule; collection of statements
- `statement` – `let` binding or expression statement
- `let_stmt` – Variable declaration with assignment
- `expr_stmt` – Expression statement
- `expr` – Expression with operator precedence
- `addition` – Addition operations (lowest precedence)
- `multiplication` – Multiplication operations (higher precedence)
- `atom` – Numbers, identifiers, or parenthesized expressions

**Function:** `pub fn parse(tokens: Vec<Token>) -> Result<Program, String>`

### 3. AST (`src/ast.rs`)

Defines the structure of the Abstract Syntax Tree:

```rust
pub enum Expr {
    Number(i64),                    // Numeric literal
    Identifier(String),             // Variable reference
    BinaryOp {                      // Binary operation
        left: Box<Expr>,
        op: BinaryOpKind,
        right: Box<Expr>,
    }
}

pub enum BinaryOpKind {
    Add,                            // Addition operator
    Mul,                            // Multiplication operator
}

pub enum Statement {
    Let {                           // Variable declaration
        name: String,
        value: Expr,
    },
    Expr(Expr),                     // Expression statement
}

pub struct Program {
    pub statements: Vec<Statement>,
}
```

## Building and Running

### Build the Project

```bash
cargo build
```

### Run with Sample Code

```bash
cargo run -- "$(cat examples/sample.mini)"
```

### Run with Custom Code

```bash
cargo run -- "let result = 5 + 3; result;"
```

### View Debug Output

The lexer prints all recognized tokens:

```bash
RUST_BACKTRACE=1 cargo run -- "let x = 10;"
```

## Dependencies

- **logos** – Fast lexical analyzer generator
- **pest** – PEG parser framework
- **pest_derive** – Procedural macro for pest grammars

See `Cargo.toml` for exact versions.

## Current Status

✅ **Implemented:**
- Lexical analysis (tokenization)
- Parser infrastructure using PEG grammar
- AST node definitions
- Example source file

🚧 **In Progress / To Do:**
- AST construction from parse results (`build_ast` function)
- Semantic analysis (type checking)
- Code generation or interpretation

## Example Workflow

1. **Input** (source code string):
   ```
   let x = 1 + 2;
   ```

2. **Lexer output** (tokens):
   ```
   [Let, Identifier("x"), Equals, Number(1), Plus, Number(2), Semicolon]
   ```

3. **Parser output** (AST):
   ```
   Program {
       statements: [
           Let {
               name: "x",
               value: BinaryOp {
                   left: Number(1),
                   op: Add,
                   right: Number(2),
               }
           }
       ]
   }
   ```

## Future Enhancements

- Expand language features (loops, conditionals, functions)
- Implement type inference/checking
- Add optimization passes
- Generate LLVM IR or bytecode
- Build a virtual machine for execution

## License

MIT

---

**Author:** Created as part of Rust compiler frontend learning  
**Date:** February 2026
