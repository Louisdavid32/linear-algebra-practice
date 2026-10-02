# Linear Algebra Practice

Hands-on linear algebra practice using **Python, Jupyter Notebook and Rust**.

## Objective

The goal of this repository is to put fundamental linear algebra concepts into practice through programming.

The same mathematical concepts and exercises are reproduced in both **Python** and **Rust**.

The objective is not only to obtain numerical answers, but also to understand the mathematics behind each operation before implementing it in code.

This repository starts with simple systems of linear equations and progressively introduces more important linear algebra concepts.

## Topics

- Systems of linear equations
- Matrices and vectors
- Matrix equation `Ax = b`
- Linear combinations
- Linear independence
- Matrix multiplication
- Elimination
- Row operations
- Solution verification
- Singular and non-singular systems
- Unique solutions
- No-solution systems
- Infinitely many solutions

## Course Inspiration

The exercises and concepts in this repository are inspired by the linear algebra course taught by **Gilbert Strang at MIT**, particularly **MIT OpenCourseWare 18.06 — Linear Algebra**.

The purpose is to take the mathematical ideas introduced in the course and reproduce them progressively through practical exercises.

The exercises are not intended to simply call a library function and return an answer.

The learning process is:

```text
Mathematical concept
        ↓
Understand the equations
        ↓
Represent the problem with matrices and vectors
        ↓
Solve it manually or algorithmically
        ↓
Implement it in Python
        ↓
Verify the result
        ↓
Reproduce the same concept in Rust
```

## Learning Approach

For every exercise, the workflow is:

1. Understand the mathematical problem.
2. Write the system of equations.
3. Identify the unknowns.
4. Identify the coefficient matrix `A`.
5. Identify the unknown vector `x`.
6. Identify the result vector `b`.
7. Express the system as `Ax = b`.
8. Solve the system.
9. Implement the solution in Python.
10. Verify the solution.
11. Reproduce the same concept in Rust.
12. Interpret the mathematical result.

## Python and Rust

Every important concept is reproduced in both languages.

```text
                    Linear Algebra Concept
                             │
                ┌────────────┴────────────┐
                │                         │
             Python                     Rust
                │                         │
         Jupyter Notebook              Cargo
                │                         │
         Experimentation          Reimplementation
                │                         │
                └────────────┬────────────┘
                             │
                       Same Mathematics
```

### Why Python?

Python is used first because Jupyter Notebook makes it easy to combine:

- mathematical explanations;
- equations;
- matrices;
- Python code;
- numerical results;
- experiments;
- observations.

Python allows us to focus primarily on understanding the mathematics.

### Why Rust?

Rust is used to reproduce the same mathematical concepts in another programming language.

The goal is not to learn two completely different mathematical approaches.

The mathematics remains the same.

```text
Same problem
   │
   ├── Python implementation
   │
   └── Rust implementation
```

This helps separate the **mathematical concept** from the **programming language used to implement it**.

## First Learning Series — Systems of Linear Equations

The first learning series contains **10 progressive exercises**.

The progression starts with a very simple system of equations and gradually introduces more important concepts.

---

## Exercise 01 — Simple 2×2 Linear System

Solve a simple system containing two equations and two unknowns.

Example:

```text
2x + y = 5
x - y = 1
```

### Main concepts

- system of linear equations;
- equations;
- unknown variables;
- coefficients;
- solution of a system.

### Goal

Understand what it means to find values of `x` and `y` that satisfy both equations simultaneously.

---

## Exercise 02 — From Equations to `Ax = b`

Take a linear system and rewrite it using matrices and vectors.

Starting from:

```text
2x + y = 5
x - y = 1
```

Identify:

```text
A = coefficient matrix
x = unknown vector
b = result vector
```

and represent the system as:

```text
Ax = b
```

### Main concepts

- coefficient matrix;
- vectors;
- matrix representation;
- dimensions;
- connection between equations and matrices.

---

## Exercise 03 — Elimination Method

Solve a system using elimination.

The objective is to understand how one equation can be combined with another to eliminate an unknown.

### Main concepts

- elimination;
- equivalent systems;
- pivots;
- solving step by step.

---

## Exercise 04 — Row Operations

Represent elimination directly using matrices.

Practice the three fundamental row operations:

1. Swap two rows.
2. Multiply a row by a non-zero scalar.
3. Add a multiple of one row to another row.

### Main concepts

- augmented matrix;
- row operations;
- elimination on matrices;
- preservation of solutions.

---

## Exercise 05 — 3×3 Linear System

Move from two unknowns to three unknowns.

General structure:

```text
a11*x + a12*y + a13*z = b1
a21*x + a22*y + a23*z = b2
a31*x + a32*y + a33*z = b3
```

Matrix representation:

```text
Ax = b
```

with:

```text
A : 3×3 matrix
x : 3×1 vector
b : 3×1 vector
```

### Main concepts

- larger systems;
- matrix dimensions;
- three-dimensional unknown vector;
- elimination with multiple variables.

---

## Exercise 06 — Matrix-Vector Multiplication

Understand what actually happens when we calculate:

```text
Ax
```

Each row of matrix `A` interacts with vector `x` to produce one component of vector `b`.

### Main concepts

- matrix-vector multiplication;
- rows;
- columns;
- dot products;
- dimensions.

### Goal

Understand why a system of equations can be represented by:

```text
Ax = b
```

instead of simply memorizing the notation.

---

## Exercise 07 — Verify a Solution

After finding a candidate solution `x`, calculate:

```text
Ax
```

and compare the result with:

```text
b
```

If:

```text
Ax = b
```

then the solution satisfies the original system.

### Main concepts

- solution verification;
- substitution;
- matrix multiplication;
- numerical validation.

---

## Exercise 08 — Linear Combinations

Interpret the system using the **columns of matrix `A`**.

If:

```text
A = [a1 a2 ... an]
```

then solving:

```text
Ax = b
```

can also be interpreted as finding coefficients such that:

```text
x1*a1 + x2*a2 + ... + xn*an = b
```

### Main concepts

- column view of a matrix;
- linear combinations;
- coefficients;
- span;
- connection between matrix multiplication and vector combinations.

---

## Exercise 09 — Linear Dependence and Singular Systems

Study a system where some equations or vectors depend on others.

Example idea:

```text
x + y = 2
2x + 2y = 4
```

The second equation does not provide completely new information.

### Main concepts

- linear dependence;
- dependent equations;
- redundant information;
- singular matrices;
- absence of a unique solution.

---

## Exercise 10 — Unique, No Solution and Infinitely Many Solutions

Compare the three important possibilities for a linear system.

### Case 1 — Unique solution

The equations intersect at exactly one solution.

```text
one solution
```

### Case 2 — No solution

The equations contradict each other.

```text
no solution
```

### Case 3 — Infinitely many solutions

Some equations represent the same constraint or depend on others.

```text
infinitely many solutions
```

### Main concepts

- consistency;
- inconsistency;
- unique solutions;
- infinitely many solutions;
- singular and non-singular systems;
- linear dependence.

This final exercise connects several concepts introduced throughout the first learning series.

## Technologies

### Python

- Python
- NumPy
- Jupyter Notebook

### Rust

- Rust
- Cargo

Additional mathematical libraries may be introduced progressively when they are useful.

## Project Structure

```text
linear-algebra-practice/
│
├── README.md
│
├── notebooks/
│   ├── 01_simple_linear_system.ipynb
│   ├── 02_matrix_form.ipynb
│   ├── 03_elimination.ipynb
│   ├── 04_row_operations.ipynb
│   ├── 05_3x3_linear_system.ipynb
│   ├── 06_matrix_vector_multiplication.ipynb
│   ├── 07_verify_solution.ipynb
│   ├── 08_linear_combinations.ipynb
│   ├── 09_linear_dependence.ipynb
│   └── 10_solution_cases.ipynb
│
└── rust/
    ├── Cargo.toml
    └── src/
```

The Rust structure will evolve progressively as each concept is reproduced.

## Repository Philosophy

This repository is primarily a **learning laboratory**.

The goal is not to hide mathematics behind library functions.

For example, a function such as:

```python
numpy.linalg.solve()
```

can solve a linear system very quickly.

However, before relying on such functions, the repository aims to understand:

- what the system represents;
- how the matrix is constructed;
- why `Ax = b`;
- how elimination works;
- what the solution means;
- why a solution may not exist;
- why several solutions may exist;
- how to verify the result.

The priority is therefore:

```text
Mathematics
    ↓
Understanding
    ↓
Manual reasoning
    ↓
Python implementation
    ↓
Verification
    ↓
Rust implementation
```

rather than:

```text
Library function
      ↓
Answer
```

## First Goal

Complete the **10 introductory exercises on systems of linear equations**.

For every important concept:

```text
1. Understand it mathematically
2. Implement it in Python
3. Explore it in Jupyter Notebook
4. Verify the result
5. Reproduce the same concept in Rust
```

The long-term objective is to progressively build a practical understanding of linear algebra that can later be applied to areas such as:

- machine learning;
- numerical computing;
- data science;
- optimization;
- computer graphics;
- scientific computing.
