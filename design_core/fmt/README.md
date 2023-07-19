# fmt

 `Formatter` 是一个格式化输出的中间层，它负责将格式化的数据转发到底层的输出缓冲区（buffer）。

`Formatter` 结构体的核心功能是将格式化数据写入一个或多个输出缓冲区，这些缓冲区可以是标准输出、文件、字符串等等。为了实现这种灵活性，`Formatter` 并没有直接实现 `Write` trait 中的方法，而是利用 `Write` trait 提供的抽象接口，在内部将格式化数据传递给底层的缓冲区。

## type

### Result

```rust
pub type Result = result::Result<(), Error>
```





## Formatter 格式化配置

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

### alternate()

确定是否指定了`#`标志，即使用:`{:#}`

```rust
pub fn alternate(&self) -> bool {
        self.flags & (1 << rt::Flag::Alternate as u32) != 0
}
```

从这里看出使用了`Flag::Alternate`转化为u32,最后转出来的值为2（请看`rt::Flag`）

```
self.flag & (1 << 2) != 0
```

### write_str()

将一些数据写入此格式化程序中包含的基础缓冲区

```rust
pub fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
}
```

这里则是调用`Write trait`中的`write_str`方法请看下面的`impl Write for Formatter`中的write_str方法

### impl Write for Formatter

为Formatter实现Write

```rust
impl Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        self.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result {
        self.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        write(self.buf, args)
    }
}
```



## Write trait

将数据写入或格式化为接受缓冲区或流的Unicode的特性。 此特性只接受UTF-8编码的数据，并且不可刷新。如果您只想接受Unicode而不需要刷新，那么您应该实现这个特性；否则，您应该实现`std::io::Write。`

### write_str()

将字符串切片写入此写入程序，返回写入是否成功

```rust
fn write_str(&mut self, s: &str) -> Result;
```



## rt::Alignment 预定义文本位置

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

```rust
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
            // 判断是否使用优雅的输出格式
            if self.is_pretty() {
                if self.fields == 0 {
                    // 将`(\n`写入Formmatter的buf的缓冲区中
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

- is_pretty()：判断优雅输出
- write_str()：将一些数据写入此格式化程序中包含的基础缓冲区

### is_pretty()

判断是否使用优雅的输出格式

```rust
fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }
```

这里调用了`fmt::Formatter`的`alternate`方法进行判断

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

# rt

## Flag

这是一种标志，用于标记格式化输出

```rust
pub(super) enum Flag {
    // +
    SignPlus,
    // -
    SignMinus,
    // :#
    Alternate,
    SignAwareZeroPad,
    DebugLowerHex,
    DebugUpperHex,
}
```

