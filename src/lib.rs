/*!
# serde_var_export

[![Actions](https://github.com/jmjoy/serde_var_export/workflows/CI/badge.svg)](https://github.com/jmjoy/serde_var_export/actions?query=workflow%3ACI)
[![Crate](https://img.shields.io/crates/v/serde_var_export.svg)](https://crates.io/crates/serde_var_export)
[![API](https://docs.rs/serde_var_export/badge.svg)](https://docs.rs/serde_var_export)

PHP function [`var_export()`](https://www.php.net/manual/en/function.var-export) support for [Serde](https://crates.io/crates/serde).

*/

mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
