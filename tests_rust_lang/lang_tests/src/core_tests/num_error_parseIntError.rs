//!# Error
//! 
//! 请参看`tests_rust_lang/lang_tests/src/core_tests/num_error_parseIntError.rs`
//! 
//! - package : `src/num/error.rs`
//! 
//! <hr />
//! 
//! ## Ⓜ️ParseIntError Source
//! 
//! <img src="https://!github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/num_error_ParseIntError.png">
//! 
//! ```rust
//! pub struct ParseIntError {
//!     pub(super) kind: IntErrorKind,
//! }
//! 
//! pub enum IntErrorKind {
//!     Empty,
//!     InvalidDigit,
//!     PosOverflow,
//!     NegOverflow,
//!     Zero,
//! }
//! 
//! impl ParseIntError {
//!     pub fn kind(&self) -> &IntErrorKind {
//!         &self.kind
//!     }
//! }
//! 
//! impl fmt::Display for ParseIntError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         self.description().fmt(f)
//!     }
//! }
//! 
//! impl Error for ParseIntError {
//!     fn description(&self) -> &str {
//!         match self.kind {
//!             IntErrorKind::Empty => "cannot parse integer from empty string",
//!             IntErrorKind::InvalidDigit => "invalid digit found in string",
//!             IntErrorKind::PosOverflow => "number too large to fit in target type",
//!             IntErrorKind::NegOverflow => "number too small to fit in target type",
//!             IntErrorKind::Zero => "number would be zero for non-zero type",
//!         }
//!     }
//! }
//! 
//! ```
//! 
//! ## Code
//! 
//! ```rust
//! pub mod core_tests;
//! 
//! use lang_core::error::MyError;
//! use lang_core::my_core::{ParseIntError, IntErrorKind};
//! 
//! fn main() {
//!     let a = ParseIntError {
//!         kind: IntErrorKind::Zero
//!     };
//!     println!("ParseIntError-description: {}", a.description());
//!     println!("IntErrorKind: {:?}",a.kind());
//! }
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```

use lang_core::error::MyError;
use lang_core::my_core::{ParseIntError, IntErrorKind};

fn main() {
    let a = ParseIntError {
        kind: IntErrorKind::Zero
    };
    println!("{}", a.description());
}