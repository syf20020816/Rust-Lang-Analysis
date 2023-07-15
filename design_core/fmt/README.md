# fmt

## type

### Result

```rust
pub type Result = result::Result<(), Error>
```



## prepare

### Formatter 格式化配置

```rust
/// 格式化程序表示与格式化相关的各种选项。
/// 用户不直接构造格式化程序；
/// 对一个的可变引用被传递给所有格式特征的fmt方法，如Debug和Display。
pub struct Formatter<'a> {
    flags: u32,
    fill: char,
    align: rt::Alignment,
    width: Option<usize>,
    precision: Option<usize>,
    buf: &'a mut (dyn Write + 'a),
}
```

### rt::Alignment 预定义文本位置

四种类型：左对齐，右对齐，居中，未定义

```rust
pub enum Alignment {
    Left,
    Right,
    Center,
    Unknown,
}
```

## Debug trait

该trait只有一个方法fmt

```
fn fmt(&self, f: &mut Formatter<'_>) -> Result;
```

其中参数f为:`Formatter`

由于源码很长很复杂，没有什么头绪，所以我们先从Formatter struct的第一个方法顺藤摸瓜

### debug_tuple

创建一个DebugTuple生成器，该生成器旨在帮助创建元组结构的`fmt::Debug`实现

很简单这个方法直接传入结构体的名称，输出的结构体名称就是传入的，相当于去自定义了一个结构体名字

```
pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b, 'a> {
        builders::debug_tuple_new(self, name)
    }
```

这里我们聚焦于`DebugTuple`这个返回以及`builders::debug_tuple_new`这个方法

请看下方`builder.rs`

# builders.rs

## DebugTuple

是用于帮助Debug的实现的结构体,当希望输出格式化的元组作为`Debug::fmt`实现的一部分时，这很有用。 可以通过`Formatter::debug_tuple`方法构造

```rust
pub struct DebugTuple<'a, 'b: 'a> {
    fmt: &'a mut fmt::Formatter<'b>,
    result: fmt::Result,
    fields: usize,
    empty_name: bool,
}
```

- fmt：格式化
- result：结果
- fields：字段
- empty_name：格式化名称是否为空

### field()

向生成的元组结构输出中添加一个新字段

```rust
pub fn field(&mut self, value: &dyn fmt::Debug) -> &mut Self {
        self.result = self.result.and_then(|_| {
            if self.is_pretty() {
                if self.fields == 0 {
                    self.fmt.write_str("(\n")?;
                }
                let mut slot = None;
                let mut state = Default::default();
                let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
                value.fmt(&mut writer)?;
                writer.write_str(",\n")
            } else {
                let prefix = if self.fields == 0 { "(" } else { ", " };
                self.fmt.write_str(prefix)?;
                value.fmt(self.fmt)
            }
        });

        self.fields += 1;
        self
    }
```



## debug_tuple_new()

```rust
pub(super) fn debug_tuple_new<'a, 'b>(
    fmt: &'a mut fmt::Formatter<'b>,
    name: &str,
) -> DebugTuple<'a, 'b> {
    let result = fmt.write_str(name);
    DebugTuple { fmt, result, fields: 0, empty_name: name.is_empty() }
}
```

这个方法用于构建一个DebugTuple


