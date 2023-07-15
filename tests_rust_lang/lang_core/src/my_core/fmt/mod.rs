//! core/fmt 格式化实现
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/14
//! @version:0.0.1
//! @description:
//! ```

use std::result;
use crate::my_core::fmt::rt::Flag;


pub mod rt;

/// 格式化程序返回 Fromatter::align
/// 左对齐，右对齐，居中
pub enum Alignment {
    Left,
    Right,
    Center,
    /// rt中的但是我觉得可以直接使用在这里
    Unknown,
}

/// 将消息格式化为流后返回的错误类型。
///
/// 此类型不支持传输除发生错误以外的其他错误。
/// 任何额外的信息都必须安排通过其他方式传输。
/// 需要记住的一件重要的事情是:
/// fmt:：Error类型不应与std:：io:：Error或std:：Error:：Error混淆
pub struct Error;

pub type Result = std::result::Result<(), Error>;

/// 此结构表示格式字符串及其参数的安全预编译版本。
/// 这不能在运行时生成，因为它不能安全地完成，
/// 所以没有给定构造函数，并且字段是私有的，以防止修改。
pub struct Arguments<'a> {
    //设置要打印的字符串片段的格式
    pieces: &'a [&'static str],
    //所有的规范是默认的则为：占位符或None
    fmt: Option<&'a [rt::Placeholder]>,
    //插值的动态参数，与字符串交错
    //pieces （每个参数前面都有一个字符串切片）
    // args: &'a [rt::Argument],
}

/// 将数据写入或格式化为接受缓冲区或流的Unicode的特性。
///
/// 此特性只接受UTF-8编码的数据，并且不可刷新。
/// 如果您只想接受Unicode而不需要刷新，那么您应该实现这个特性；
///
/// 否则，您应该实现std:：io:：Write。
pub trait Write {
    /// 将字符串切片写入此写入程序，返回写入是否成功。
    /// 只有成功写入整个字符串切片，此方法才能成功，
    /// 并且在写入所有数据或发生错误之前，此方法不会返回。
    fn write_str(&mut self, s: &str) -> Result;
    fn write_char(&mut self, c: char) -> Result {
        // 将此字符以UTF-8编码到提供的字节缓冲区中，然后返回包含编码字符的缓冲区的子切片。
        // 如果缓冲区不够大，会引发恐慌。
        // 长度为4的缓冲区足够大，可以对任何字符进行编码。
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }
    // fn write_fmt(mut self: &mut Self, args: Arg)
}

/// 格式化配置。
///
/// 格式化程序表示与格式化相关的各种选项。
/// 用户不直接构造格式化程序；
/// 对一个的可变引用被传递给所有格式特征的fmt方法，如Debug和Display。
pub struct Formatter<'a> {
    flags: u32,
    fill: char,
    align: Alignment,
    width: Option<usize>,
    precision: Option<usize>,
    buf: &'a mut (dyn Write + 'a),
}

/// 源码中拆开了new和其他的方法，放一起也一样
impl<'a> Formatter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Formatter<'a> {
        Formatter {
            flags: 0,
            fill: ' ',
            align: Alignment::Unknown,
            width: None,
            precision: None,
            buf,
        }
    }
    /// 约束生命周期b>c
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
        where
            'b: 'c,
            F: FnOnce(&'b mut (dyn Write + 'b)) -> &'c mut (dyn Write + 'c),
    {
        Formatter {
            flags: self.flags,
            fill: self.fill,
            align: self.align,
            width: self.width,
            precision: self.precision,
            buf: wrap(self.buf),
        }
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn fill(&self) -> char {
        self.fill
    }
    pub fn align(&self) -> Option<Alignment> {
        match self.align {
            Alignment::Left => Some(Alignment::Left),
            Alignment::Right => Some(Alignment::Right),
            Alignment::Center => Some(Alignment::Center),
            Alignment::Unknown => None
        }
    }
    pub fn width(&self) -> Option<usize> {
        self.width
    }
    pub fn precision(&self) -> Option<usize> {
        self.precision
    }
    ///确定是否指定了+标志
    pub fn sign_plus(&self) -> bool {
        // SignPlus, : 0
        // SignMinus, : 1
        // Alternate, : 2
        // SignAwareZeroPad, : 3
        // DebugLowerHex, :  4
        // DebugUpperHex, : 5
        // + : flags != 0
        self.flags & (1 << Flag::SignPlus as u32) != 0
    }
    pub fn sign_minus(&self) -> bool {
        self.flags & (1 << Flag::SignMinus as u32) != 0
    }
    /// 确定是否指定了#标志
    /// 存在表示带上struct名称和`{}`或`()`取决于结构体
    /// 有 ： assert_eq!(format!("{:#}", Foo(23)), "Foo(23)");
    /// 无 ： assert_eq!(format!("{}", Foo(23)), "23");
    pub fn alternate(&self) -> bool {
        // - : flags & 2 != 0 | flags & 10(b) != 0
        self.flags & (1 << Flag::Alternate as u32) != 0
    }
    /// 确定是否指定了0标志
    pub fn sign_aware_zero_pad(&self) -> bool {
        self.flags & (1 << Flag::SignAwareZeroPad as u32) != 0
    }
    fn debug_lower_hex(&self) -> bool {
        self.flags & (1 << rt::Flag::DebugLowerHex as u32) != 0
    }

    fn debug_upper_hex(&self) -> bool {
        self.flags & (1 << rt::Flag::DebugUpperHex as u32) != 0
    }
    pub fn pad_integral(&mut self, is_nonnegative: bool, prefix: &str, buf: &str) -> Result {
        //字节长度
        let mut width = buf.len();
        //符号
        let mut sign = None;
        // true : -
        // false : +
        // 长度+1:增加符号位
        if is_nonnegative {
            sign = Some('-');
            width += 1;
        } else if self.sign_plus() {
            sign = Some('+');
            width += 1;
        }

        let prefix = if self.alternate() {
            width += prefix.chars().count();
            Some(prefix)
        } else {
            None
        };
        //写入符号（如果存在），然后写入前缀（如果请求）
        fn write_prefix(f: &mut Formatter<'_>, sign: Option<char>, prefix: Option<&str>) -> Result {
            if let Some(c) = sign {
                f.buf.write_char(c)?;
            }
            if let Some(prefix) = prefix { f.buf.write_str(prefix) } else { Ok(()) }
        }
        match self.width {
            //如果没有最低长度要求，那么我们可以只写字节
            None => {
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)
            }
            //检查我们是否超过了最小宽度，如果是，那么我们也可以只写字节
            Some(min) if width >= min => {
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)
            }
            //填充字符串为0
            Some(min) if self.sign_aware_zero_pad() => {
                //将fill设置为0
                //这里的old_fill就是原来的值
                //新的self.fill = '0'
                let old_fill = std::mem::replace(&mut self.fill, '0');
                //同上
                let old_align = crate::mem::replace(&mut self.align, Alignment::Right);
                write_prefix(self, sign, prefix)?;
                let post_padding = self.padding(min - width, Alignment::Right)?;
                self.buf.write_str(buf)?;
                post_padding.write(self)?;
                self.fill = old_fill;
                self.align = old_align;
                Ok(())
            }
            Some(min) => {
                let post_padding = self.padding(min - width, Alignment::Right)?;
                write_prefix(self, sign, prefix)?;
                self.buf.write_str(buf)?;
                post_padding.write(self)
            }
        }
    }
    pub(crate) fn padding(
        &mut self,
        padding: usize,
        default: Alignment,
    ) -> result::Result<PostPadding, Error> {
        let align = match self.align {
            Alignment::Unknown => default,
            Alignment::Left => Alignment::Left,
            Alignment::Right => Alignment::Right,
            Alignment::Center => Alignment::Center,
        };

        let (pre_pad, post_pad) = match align {
            Alignment::Left => (0, padding),
            Alignment::Right => (padding, 0),
            Alignment::Center => (padding / 2, (padding + 1) / 2),
        };

        for _ in 0..pre_pad {
            self.buf.write_char(self.fill)?;
        }

        Ok(PostPadding::new(self.fill, post_pad))
    }
}


/// 格式化输出的派生宏
/// 需要使用{:?}指定
pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> ();
}

