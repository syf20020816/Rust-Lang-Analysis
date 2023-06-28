# 🦀Rust Lang Analysis

- author : syf20020816@outlook.com
- date : 2023-06-29

## Ⓜ️icon

- ❗：注意点
- Ⓜ️：重点
- 🔖：标签

## Ⓜ️Preface

Rust 核心库是Rust 标准库的无依赖基础。它是语言和标准库库之间的可移植粘合剂，它定义了所有 Rust 代码的内在和原始构建块。它没有链接到上游库，没有系统库，也没有 libc。核心库是 *最小的*: 它甚至不知道堆分配，也不提供并发或 I/O。 这些东西都需要平台集成，而核心库是与平台无关的。

 ❗所以我们得出需要看std之前先看core

## Ⓜ️My Learn Order

1. Types
2. Modules
3. Macros