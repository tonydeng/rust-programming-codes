//! 函数与闭包
//! 

/// # 函数定义
///
/// Basic usage:
///
/// ```
/// pub fn fizz_buzz(num: i32) -> String {
///     if num % 15 == 0 {
///         return "fizzbuzz".to_string();
///     } else if num % 3 == 0 {
///         return "fizz".to_string();
///     } else if num % 5 == 0 {
///         return "buzz".to_string();
///     } else {
///         return num.to_string();
///     }
/// }
/// assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
/// assert_eq!(fizz_buzz(3), "fizz".to_string());
/// assert_eq!(fizz_buzz(5), "buzz".to_string());
/// assert_eq!(fizz_buzz(13), "13".to_string());
/// ```
pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}


/// # 函数指针： 函数作为参数
///
/// Basic usage:
///
/// ```
/// pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
///     op(a, b)
/// }
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
///
/// let a = 2;
/// let b = 3;
/// assert_eq!(math(sum, a, b), 5);
/// assert_eq!(math(product, a, b), 6);
/// ```
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) ->i32{
    op(a, b)
}

fn sum(a: i32, b: i32) ->i32 {
    a + b
}

fn prooduct(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true(a: i32, b: i32) -> bool {
    true
}