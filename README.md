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

### ❗Rust Nightly

在运行本项目时你需要使用Nightly（前瞻版）不然将报错：`const trait impls are experimental`

### ❗How to change Rust Nightly

使用`rustup show`查看当前使用版本

再使用`rustup default nightly`切换即可

若你没有nightly请使用这个命令安装`rustup toolchain install nightly`

<img src="https://github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/rust_nightly.png">

## Ⓜ️My Learn Order

1. Types
2. Modules
3. Macros

## Ⓜ️Note

所有解析源码和库的目录是基本相同的！这会省去很多查找的时间

例如：

`core/src/num/shells/int_macros.rs`对应 `tests_rust_lang/lang_core/src/my_core/num/shells/int_macros.rs`

## Ⓜ️How To Read

每一个小模块对应标准库和核心库都有对应的重实现

0️⃣首先当你想要了解一个模块的时候，查找他位于标准库或核心库中的位置，例如`int_module!`位于`core/src/num/shells/int_macros.rs`依照对应法则找到文档位置`core/numm/shells/int_macros.md`

1️⃣找到模块的测试代码，这会帮助你快速了解该模块能够做什么，对应法则：`src/num/shells/int_macros.rs`-->`tests_rust_lang/lang_tests/src/core_tests/num/shells/int_macros.rs`

2️⃣通过文档中给出的源码路线图，按照顺序查看每一步每一块的实现，请注意阅读注解，在注解中会用简单的语句描述函数、变量、常量、结构体的作用

3️⃣学习下一个小实现

4️⃣基本看完实现后请查找design_core或design_std查找设计说明，如：`design_core/num/README.md`，这里会告诉你为什么需要这样实现和这样实现的好处，欢迎大家进行补充、修改和指正

5️⃣学习下一个模块

## 😎Great Practice

在提供的项目中还有一个工程：practice

在这个工程中会给出一些有用的小例子帮助你学习完源码后进行扩展