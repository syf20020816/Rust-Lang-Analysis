# int_macros

请参看`tests_rust_lang/lang_tests/src/core_tests/num_shells_int_macros.rs`

- package : `src/num/int_macros.rs`

## Preface

❗请注意，这里的int_macros虽然和`src/num/shells/int_macros.rs`文件名称相同但是并不是一个东西，这里是设计了`int_impl!`宏

所有`impl u/i num types`都由该宏实现如（i8）：

```rust
impl i8 {
    int_impl! {
        Self = i8,
        ActualT = i8,
        UnsignedT = u8,
        BITS = 8,
        BITS_MINUS_ONE = 7,
        Min = -128,
        Max = 127,
        rot = 2,
        rot_op = "-0x7e",
        rot_result = "0xa",
        swap_op = "0x12",
        swapped = "0x12",
        reversed = "0x48",
        le_bytes = "[0x12]",
        be_bytes = "[0x12]",
        to_xe_bytes_doc = "",
        from_xe_bytes_doc = "",
        bound_condition = "",
    }
}
```

<hr />

## Ⓜ️Source

源码部分我将拆为2个部分进行说明

```rust
macro_rules! int_impl {
    (
       `please see Macro input`
    ) => {
       `please see source code`
    }
}

```

### Macro input

作为整个宏的输入

```rust
macro_rules! int_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty,
        BITS = $BITS:literal,
        BITS_MINUS_ONE = $BITS_MINUS_ONE:literal,
        Min = $Min:literal,
        Max = $Max:literal,
        rot = $rot:literal,
        rot_op = $rot_op:literal,
        rot_result = $rot_result:literal,
        swap_op = $swap_op:literal,
        swapped = $swapped:literal,
        reversed = $reversed:literal,
        le_bytes = $le_bytes:literal,
        be_bytes = $be_bytes:literal,
        to_xe_bytes_doc = $to_xe_bytes_doc:expr,
        from_xe_bytes_doc = $from_xe_bytes_doc:expr,
        bound_condition = $bound_condition:literal,
    ) =>{}
}
```

- `Self = $SelfT:ty` : Num的具体类型，如i8,u8
- `ActualT = $ActualT:ident`: Num真正的类型
- `UnsignedT = $UnsignedT:ty`：对应无符号类型
- `BITS = $BITS:literal`：该类型所占字节长度，如i8占用8个字节(literal：字面量)
- `BITS_MINUS_ONE = $BITS_MINUS_ONE:literal`：表示 `BITS - 1`
- `Min = $Min:literal`：最小值 match `$T:MIN`
- `Max = $Max:literal`：最大值 match `$T:MAX`
- `rot = $rot:literal`：表示旋转操作（若你不知道什么是旋转操作请看：technical_term.md）
- `rot_op = $rot_op:literal`：表示旋转操作的符号
- `rot_result = $rot_result:literal`：表示旋转操作的结果
- `swap_op = $swap_op:literal`: 表示交换操作的符号
- `swapped = $swapped:literal`: 表示交换结果
- `reversed = $reversed:literal`: 表示反转结果
- `le_bytes = $le_bytes:literal`: 表示小端字节序
- `be_bytes = $be_bytes:literal`: 表示大端字节序
- `to_xe_bytes_doc = $to_xe_bytes_doc:expr`: 用于转化为字节数组的文档注释
- ` from_xe_bytes_doc = $from_xe_bytes_doc:expr`: 用于从字节数组转化的文档注释
- `bound_condition = $bound_condition:literal`: 用于条件判断

## Code

### step1: my_impl_i8

这里只是暂时的实现

```rust
use crate::int_impl;

#[derive(Debug)]
pub struct MyImplI8 {
    pub max: u8,
    pub min: i8,
}

int_impl! {
        SelfT = MyImplI8,
        ActualT = i8,
        UnsignedT = u8,
        BITS = 8,
        BITS_MINUS_ONE = 7,
        Min = -128,
        Max = 127,
        rot = 2,
        rot_op = "-0x7e",
        rot_result = "0xa",
        swap_op = "0x12",
        swapped = "0x12",
        reversed = "0x48",
        le_bytes = "[0x12]",
        be_bytes = "[0x12]",
        to_xe_bytes_doc = "",
        from_xe_bytes_doc = "",
        bound_condition = "",
}
```

### step2: int_impl!

```

```

