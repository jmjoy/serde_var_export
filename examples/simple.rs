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
