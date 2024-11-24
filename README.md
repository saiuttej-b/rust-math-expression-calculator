# Rust Calculator

A simple command-line calculator built in Rust that evaluates mathematical expressions. It supports basic operations (`+`, `-`, `*`, `/`) with correct operator precedence and parentheses handling. The calculator uses the **Shunting-Yard Algorithm** for parsing and **Stack-Based Evaluation** for computation.

---

## Features

- Evaluate mathematical expressions such as `2 * 3 + 4` or `10 / (6 - 1)`.
- Handles **operator precedence** (`*` and `/` have higher precedence than `+` and `-`).
- Supports **parentheses** for grouping expressions, e.g., `3 + (2 * 5)`.
- Accepts floating-point numbers.
- Provides helpful error messages for invalid input.

---

## How It Works

1. **Input Parsing**: The program tokenizes the input into numbers, operators, and parentheses.
2. **Expression Conversion**: Converts the infix notation (e.g., `2 * 3 + 4`) into Reverse Polish Notation (RPN) using the **Shunting-Yard Algorithm**.
3. **Expression Evaluation**: Evaluates the RPN expression using a **stack-based approach**.

---

## Example Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-calculator.git
   cd rust-calculator
   ```
2. Build and run the project
   ```bash
   cargo run
   ```
3. Example commands:
   ```yml
   Enter an expression: 2 * 3 + 4
   Result: 10
   Enter an expression: 10 / (6 - 1)
   Result: 2
   Enter an expression: exit
   ```
