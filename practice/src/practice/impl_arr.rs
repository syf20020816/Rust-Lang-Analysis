//! impl []
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/14
//! @version:0.0.1
//! @description:
//! ```

enum AsciiChar{
    Null,
    Delete
}

pub trait AsciiCharSlice<Rhs = AsciiChar> {
    type Origin;
    fn as_str(&self) -> Vec<&str>;
}

impl AsciiCharSlice for [AsciiChar] {
    type Origin = AsciiChar;

    fn as_str(&self) -> Vec<&str> {
        let mut res:Vec<&str> = Vec::new();
        for item in self {
            match item {
                AsciiChar::Null => res.push("Null"),
                AsciiChar::Delete => res.push("Delete"),
                _ => ()
            }
        }
        res
    }
}