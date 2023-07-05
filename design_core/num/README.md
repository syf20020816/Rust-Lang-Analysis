# Num Design

## 👎shells

在这个目录下实现了i/u两种系列的数字类型以及一个int_macros中的`int_module!`而这里做的事情很简单就是生成对应数字类型的 MAX和MIN，以如下形式:

```rust
pub const MIN:i8 = i8::MIN
pub const MAX:i8 = i8::MAX
```

### int_module!

目的是统一的，简便的生成对应每个i/u数字类型的最大最小值引用

这些常量模块提供了与基本类型相关的有用常量，设计这些冗余的常量模块有几个主要原因：

1. **方便导入**: 使用 `std::i8::MIN` 和 `std::i8::MAX` 这样的全名路径可以明确指定要使用的常量。这在代码中可以提供更清晰的语义，并且能够避免命名冲突。
2. **跨模块可见性**: 通过将常量放在常量模块中，可以在不同的模块中进行访问，而无需显式地引入常量。这对于库作者和用户来说都很方便，因为他们可以直接使用这些常量，而无需显式导入它们。
3. **示例和学习**: 这些常量模块提供了一个方便的位置，可以查看和学习与特定类型相关的常量和约束。这对于初学者来说尤其有帮助，因为它们可以在标准库文档中找到与整数类型相关的常量。

## 👍int_impl

### constants

```rust
MyImplI8::MAX
MyImplI8::MIN
MyImplI8::BITS
```

### from_str_radix()

将给定基数中的字符串切片转换为整数
意思就是:
- u8::from_str_radix("16",10) == 16 :表示将16这个10进制数字转为整数
- u8::from_str_radix("16",8) == 14 :表示将16这个8进制数字转为整数

在int_impl中使用：`tests_rust_lang/lang_core/src/my_core/num/mod.rs`中的`from_str_radix()`

处理步骤：

1. 判断进制数（range:[2,36]）
2. 检查Empty
3. 检查类型（true：i type number，false：u type number）
4. 目标字符串转字节切片（[u8]）
5. 匹配字节切片首位检查正负值
6. 从u32转适配类型
7. 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中(if can_not_overflow())
  - 检查溢出
8. 结束包装为Result

#### 使用

```rust
println!("{:?}", MyImplI8::from_str_radix("16", 8));
```

