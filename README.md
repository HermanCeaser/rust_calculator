# Rust Calculator

This is a simple calculator implemented in Rust, designed to perform basic arithmetic operations using enums and pattern matching.


## Features
- Supports addition, subtraction, multiplication, and division.
- Uses the `Operation` enum to represent arithmetic operations.
- Implements pattern matching for clean and efficient logic handling.
- Provides a command-line interface for user input and displays the result.

## Prerequisites
To run this project, ensure you have the following installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

## Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/HermanCeaser/rust_calculator.git
cd rust_calculator
```

### 2. Build the Project
Compile the project using Cargo:
```bash
cargo build
```

### 3. Run the Project
Run the compiled program:
```bash
cargo run
```

### Input Instructions
 - Enter the first number.
 - Specify the operation (+ for addition, - for subtraction, * for multiplication, / for division).
 - Enter the second number.
 - View the result.


## How It Works

1. **Define the Operation Enum**  
   - The `Operation` enum has variants: `Add`, `Subtract`, `Multiply`, and `Divide`.  
   - Each variant holds two `f64` values representing the numbers involved in the operation.

2. **Calculate Function**  
   - The `calculate` function takes an `Operation` enum instance as input.  
   - It uses pattern matching to perform the corresponding arithmetic operation and returns the result as an `f64`.

3. **User Interaction**  
   - The program prompts the user to enter two numbers and an operation.  
   - Based on the input, an `Operation` enum instance is created and passed to the `calculate` function.  
   - The result is computed and printed to the console.


