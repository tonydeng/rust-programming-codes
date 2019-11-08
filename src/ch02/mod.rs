//! 第二章：语言精要
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第2章：{}", "语言精要");
/// }
/// title();
/// ```
pub fn title(){
    println!("第2章：{}", "语言精要");
}

pub mod binding;
pub mod function;
pub mod control_flow;