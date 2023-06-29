//! Error Types for conversion Int types
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```

use std::any::{ TypeId};

use std::fmt::{Debug, Display};

/// Error is a trait!
/// <hr>
/// design:
/// - Error need to be print
/// - Error need contain some msg (but sometimes it is None)
pub trait MyError: Debug + Display {
    /// 低级源信息
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }
    /// 获取TypeId
    fn type_id(&self, _: private::Internal) -> TypeId
        where
            Self: 'static,
    {
        TypeId::of::<Self>()
    }

    fn description(&self)->&str{
        "My Error Des; use Display"
    }
    fn cause(&self)->Option<&dyn MyError>{
        self.source()
    }
    // fn provide<'a>(&'a self,demand:&mut Demand<'a>){}
}

///这是一个防止`type_id`被`Error覆盖的破解方法`
///
///事实上，因为这可能会导致不合理的下转换。
mod private {
    #[derive(Debug)]
    pub struct Internal;
}
