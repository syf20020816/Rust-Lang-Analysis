//! How to define an Error
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

///绿帽子的错误
pub struct CuckoldError{
    pub kind:CuckoldKind
}

pub enum CuckoldKind{
    GreatMan,
    Gay,
    Impulse
}


impl Debug for CuckoldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.description(),f)
    }
}

impl Display for CuckoldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self,f)
    }
}

impl Error for CuckoldError{
    fn description(&self) -> &str {
        match self.kind {
            CuckoldKind::GreatMan => "you are a great man , sorry",
            CuckoldKind::Gay => "sorry I am a gay",
            CuckoldKind::Impulse => "that thing was just an accident"
        }
    }

}