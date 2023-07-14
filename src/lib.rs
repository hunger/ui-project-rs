/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// # use ui_project_rs::add;
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtract two numbers
///
/// ```
/// # use ui_project_rs::sub;
/// let result = sub(2, 3);
/// assert_eq!(result, -1);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiply two numbers
///
/// ```
/// # use ui_project_rs::mul;
/// let result = mul(2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// # use ui_project_rs::div;
/// let result = div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// # use ui_project_rs::div;
/// // panics on division by zero
/// div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}
