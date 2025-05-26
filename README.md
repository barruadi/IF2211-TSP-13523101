# Traveling Salesman Problem (TSP)

This is a TSP solver using Dynamic Programming in Rust

## Algoritma 

This algorithm solves the TSP by breaking it down into subproblems and storing previously computed results (memoization) to avoid redundant computations.

## Requirement

- Rust
- Cargo

## How To Use

### Run the Code

```sh
cargo run
```

### Input Example

#### Coordinate Input

Format:
```sh
# Coordinate Mode
# Node Count
# Position
```
Example:
```sh
1
4
0 0
0 1
1 0
1 1
```

#### Matrix Input

Format:
```sh
# Matrix Mode
# Node count
# Distance from one Node to another
```
Example:
```sh
2
4
0 10 15 20
10 0 35 25
15 35 0 30
20 25 30 0
```

## Author

| **NIM**  | **Nama**             | **Github**                              |
| -------- | -------------------- | --------------------------------------- |
| 13523101 | Barru Adi Utomo      | [barruadi](https://github.com/barruadi) |