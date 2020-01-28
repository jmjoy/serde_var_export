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

#[test]
fn test_complex_0() {
    #[derive(Serialize)]
    struct Item {
        name: String,
        age: i32,
        is: bool,
    }

    #[derive(Serialize)]
    struct Complex {
        map: HashMap<i32, Item>,
        list: Vec<Item>,
        num: Option<i32>,
        empty: Vec<i32>,
    }

    let complex = Complex {
        map: hash_map! {
            0 => Item {
                name: "foo".to_owned(),
                age: 100,
                is: true,
            },
            10 => Item {
                name: "'bar\\\n'".to_owned(),
                age: 200,
                is: false,
            }
        },
        list: vec![
            Item {
                name: "foo".to_owned(),
                age: 100,
                is: true,
            },
            Item {
                name: "'bar\\\n'".to_owned(),
                age: 200,
                is: false,
            },
        ],
        num: None,
        empty: vec![],
    };

    let s = serde_var_export::to_string(&complex).unwrap();
    let s0 = "array(
  'map' => \n  array(
    0 => \n    array(
      'name' => 'foo',
      'age' => 100,
      'is' => true,
    ),
    10 => \n    array(
      'name' => '\\'bar\\\\\n\\'',
      'age' => 200,
      'is' => false,
    ),
  ),
  'list' => \n  array(
    0 => \n    array(
      'name' => 'foo',
      'age' => 100,
      'is' => true,
    ),
    1 => \n    array(
      'name' => '\\'bar\\\\\n\\'',
      'age' => 200,
      'is' => false,
    ),
  ),
  'num' => NULL,
  'empty' => \n  array(
  ),
)";
    let s1 = "array(
  'map' => \n  array(
    10 => \n    array(
      'name' => '\\'bar\\\\\n\\'',
      'age' => 200,
      'is' => false,
    ),
    0 => \n    array(
      'name' => 'foo',
      'age' => 100,
      'is' => true,
    ),
  ),
  'list' => \n  array(
    0 => \n    array(
      'name' => 'foo',
      'age' => 100,
      'is' => true,
    ),
    1 => \n    array(
      'name' => '\\'bar\\\\\n\\'',
      'age' => 200,
      'is' => false,
    ),
  ),
  'num' => NULL,
  'empty' => \n  array(
  ),
)";

    assert_either_eq!(s, s0, s1);
}
