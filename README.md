# serde_var_export
                                                                                                                                                     
[![Actions](https://github.com/jmjoy/serde_var_export/workflows/CI/badge.svg)](https://github.com/jmjoy/serde_var_export/actions?query=workflow%3ACI)
[![Crate](https://img.shields.io/crates/v/serde_var_export.svg)](https://crates.io/crates/serde_var_export)
[![API](https://docs.rs/serde_var_export/badge.svg)](https://docs.rs/serde_var_export)
                                                                                                                                                     
PHP function [`var_export()`](https://www.php.net/manual/en/function.var-export) support for [Serde](https://crates.io/crates/serde).
                                                                                                                                                     
## Installation
                                                                                                                                                     
With [cargo add](https://github.com/killercup/cargo-edit) installed run:
                                                                                                                                                     
```sh
$ cargo add -s serde_var_export
```
                                                                                                                                                     
## Example
                                                                                                                                                     
```rust
use serde_derive::Serialize;
                                                                                                                                                     
#[derive(Serialize)]
struct Foo {
    names: Vec<String>,
    nums: Vec<i32>,
}
                                                                                                                                                     
fn main() {
    let foo = Foo {
        names: vec!["hello".to_owned(), "world".to_owned()],
        nums: vec![1, 2, 3],
    };
    let s = serde_var_export::to_string(&foo).unwrap();
    println!("{}", s);
}
```
                                                                                                                                                     
print result:
                                                                                                                                                     
```php
array(
  'names' =>
  array(
    0 => 'hello',
    1 => 'world',
  ),
  'nums' =>
  array(
    0 => 1,
    1 => 2,
    2 => 3,
  ),
)
```

## Limitation

Now only support serialization, deserialization will support in future.
                                                                                                                                                     
## License
                                                                                                                                                     
The Unlicense.
