# Minigrep

## Description

Command Line Tool simulating basic functionalities of grep. Made as part of Rust Book reading and learning from it. With added github actions pipeline for testing and building the project.

## How to

### Case sensitive search

```bash
    cargo run -- <query parameter> <file_path parameter>
```

### Case insensitive search

```bash
    IGNORE_CASE=1 cargo run -- <query parameter> <file_path parameter>
```
