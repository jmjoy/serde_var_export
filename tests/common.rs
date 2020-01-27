#[macro_export]
macro_rules! assert_either_eq {
    ($left:expr, $right:expr, $right2:expr) => {{
        match (&$left, &$right, &$right2) {
            (left_val, right_val, right2_val) => {
                if !(*left_val == *right_val) && !(*left_val == *right2_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(
                        r#"assertion failed: `(left == right) || (left == right2)`
   left: `{:?}`,
  right: `{:?}`,
 right2: `{:?}`"#,
                        &*left_val, &*right_val, &*right2_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $right2:expr,) => {{
        $crate::assert_eq!($left, $right, $right2)
    }};
}
