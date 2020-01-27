#[macro_use]
mod common;

use serde_derive::Serialize;
use std::collections::HashMap;

#[test]
fn serialize_tuple_variant() {
    #[derive(Serialize)]
    enum Foo {
        Bar(i32, String),
    }

    let bar = Foo::Bar(100, "'bar'".to_string());
    let s = serde_var_export::to_string(&bar).unwrap();
    assert_eq!(
        s,
        "array(
  'Bar' => \n  array(
    0 => 100,
    1 => '\\'bar\\'',
  ),
)"
    )
}

#[test]
fn serialize_map() {
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    map.insert("foo2", "bar2");

    let s = serde_var_export::to_string(&map).unwrap();
    assert_either_eq!(
        s,
        "array(
  'foo' => 'bar',
  'foo2' => 'bar2',
)",
        "array(
  'foo2' => 'bar2',
  'foo' => 'bar',
)"
    );
}

#[test]
fn serialize_struct_variant() {
    #[derive(Serialize)]
    enum Foo {
        Bar { name: String, value: i32 },
    }

    let bar = Foo::Bar {
        name: "nnn".to_owned(),
        value: 100,
    };
    let s = serde_var_export::to_string(&bar).unwrap();
    assert_eq!(
        s,
        "array(
  'Bar' => \n  array(
    'name' => 'nnn',
    'value' => 100,
  ),
)"
    );
}
