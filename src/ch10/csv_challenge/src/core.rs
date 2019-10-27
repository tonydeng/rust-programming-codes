// core.rs
pub mod read;
pub mod write;

use crate::err::Error;

/**
 * 考虑处理替换同样需要PathBuf，File和Error这三种类型的支持，为了避免代码重复，现在将引入这三种类型的代码移入到core.rs
 */
use std::{
    path::PathBuf,
    fs::File,
    io::{Read,Write}
};