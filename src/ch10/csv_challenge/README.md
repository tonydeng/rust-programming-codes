#  CVS Challenge

- [CVS Challenge](#cvs-challenge)
  - [项目介绍](#%e9%a1%b9%e7%9b%ae%e4%bb%8b%e7%bb%8d)
  - [步骤](#%e6%ad%a5%e9%aa%a4)
    - [1. 创建项目](#1-%e5%88%9b%e5%bb%ba%e9%a1%b9%e7%9b%ae)
    - [2. 使用`structopt`解析命令行参数](#2-%e4%bd%bf%e7%94%a8structopt%e8%a7%a3%e6%9e%90%e5%91%bd%e4%bb%a4%e8%a1%8c%e5%8f%82%e6%95%b0)
    - [3. 添加`opt`模块](#3-%e6%b7%bb%e5%8a%a0opt%e6%a8%a1%e5%9d%97)
    - [4. 添加对`csv`的核心处理模块](#4-%e6%b7%bb%e5%8a%a0%e5%af%b9csv%e7%9a%84%e6%a0%b8%e5%bf%83%e5%a4%84%e7%90%86%e6%a8%a1%e5%9d%97)
    - [5. 添加单元测试](#5-%e6%b7%bb%e5%8a%a0%e5%8d%95%e5%85%83%e6%b5%8b%e8%af%95)
    - [6. 添加集成测试](#6-%e6%b7%bb%e5%8a%a0%e9%9b%86%e6%88%90%e6%b5%8b%e8%af%95)
      - [6.1. 新增`src/lib.rs`，将所有模块引入其中，并暴露对外可以调用的函数。](#61-%e6%96%b0%e5%a2%9esrclibrs%e5%b0%86%e6%89%80%e6%9c%89%e6%a8%a1%e5%9d%97%e5%bc%95%e5%85%a5%e5%85%b6%e4%b8%ad%e5%b9%b6%e6%9a%b4%e9%9c%b2%e5%af%b9%e5%a4%96%e5%8f%af%e4%bb%a5%e8%b0%83%e7%94%a8%e7%9a%84%e5%87%bd%e6%95%b0)
      - [6.2. 修改`main.rs`](#62-%e4%bf%ae%e6%94%b9mainrs)
      - [6.3. 创建`tests/integration_test.rs`文件](#63-%e5%88%9b%e5%bb%batestsintegrationtestrs%e6%96%87%e4%bb%b6)
    - [7. 添加性能基准测试](#7-%e6%b7%bb%e5%8a%a0%e6%80%a7%e8%83%bd%e5%9f%ba%e5%87%86%e6%b5%8b%e8%af%95)
      - [7.1. 创建`benches/file_op_bench.rs`](#71-%e5%88%9b%e5%bb%babenchesfileopbenchrs)
      - [7.2. 安装并使用`rust nightly`版本进行基准测试](#72-%e5%ae%89%e8%a3%85%e5%b9%b6%e4%bd%bf%e7%94%a8rust-nightly%e7%89%88%e6%9c%ac%e8%bf%9b%e8%a1%8c%e5%9f%ba%e5%87%86%e6%b5%8b%e8%af%95)

## 项目介绍

本项目主要是用来验证`Rust`模块化编程。

## 步骤

### 1. 创建项目

```bash
cargo new --bin csv_challenge
```

> 创建二进制项目。`cargo`默认使用了`--bin`参数。

默认目录结构

```bash
.
├── Cargo.toml
├── src
│  └── main.rs
```

### 2. 使用`structopt`解析命令行参数

`structopt`是基于`clap`的基础上构建而成，简化了操作。

添加依赖

```toml
[dependencies]
structopt = "0.2"
structopt-derive = "0.2"
```

> 因为`structopt`是基于过程宏(`Procedural Macro`)的，所以它依赖`sructopt-derive`包。

### 3. 添加`opt`模块

创建`src/opt.rs`

```rust
use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = "Usage")]
pub struct Opt {
    #[structopt(help = "Input file")]
    pub input : String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replcaement Column Name")]
    pub replacement:String,
    #[structopt(help ="Output file, stdout if not present")]
    pub output: Option<String>, 
}
```

在`main.rs`中来引用`Opt`。

```rust
mod opt;
use self::opt::Opt;

fn main(){
    let opt = Opt::from_args();
}
```

### 4. 添加对`csv`的核心处理模块

添加`src/core/read.rs`和`src/core/write.rs`两个文件

- read.rs

```rust
use super::{Error, PathBuf,File,Read,Write};

pub fn load_csv(csv_file: PathBuf) -> Result<String,Error> {
    let file = read(csv_file)?;
    Ok(file)
}

pub fn write_csv(csv_data: &str, filename: &str) -> Result<(),Error> {
    write(csv_data,filename)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String,Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err("input file missing")?
    }

    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(data: &str, filename : &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}
```

- write.rs

```rust
use super::*;

pub fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();

    let columns : Vec<&str> = headers.split(',').collect();

    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file")?,
    };

    let mut result = String::with_capacity(data.capacity());

    result.push_str(&columns.join(","));
    result.push('\n');

    for line in lines {
        let mut records : Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}
```

### 5. 添加单元测试

```rust
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::load_csv;

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data =load_csv(filename);
        assert!(csv_data.is_ok());
    }
}
```

### 6. 添加集成测试

`Rust`对于二进制是不能添加集成测试的，因为二进制包只能独立使用，并不能对外提供可调用的函数，需要进行改造。

#### 6.1. 新增`src/lib.rs`，将所有模块引入其中，并暴露对外可以调用的函数。

```rust
mod opt;
mod err;
mod core;

pub use self::opt::Opt;
pub use self::core::{
    read::{load_csv,write_csv},
    write::replace_column,
};
```

#### 6.2. 修改`main.rs`

```rust
use csv_challenge::{
    Opt,
    {load_csv,write_csv},
    replace_column,
};
```

> 这种`main.rs`配合`lib.rs`的形式，是二进制包的**最佳实践**

#### 6.3. 创建`tests/integration_test.rs`文件

```rust
#[cfg(test)]

mod test{
    use std::path::PathBuf;
    use std::fs;

    use csv_challenge::{
        {load_csv,write_csv},
        replace_column,
    };

    #[test]
    fn test_csv_challenge() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data,"City","Beijing").unwrap();
        let output_file = write_csv(&modified_data, "output/test.csv");

        assert!(output_file.is_ok());

        fs::remove_file("output/test.csv");
    }
}
```

### 7. 添加性能基准测试

#### 7.1. 创建`benches/file_op_bench.rs`

```rust
#![feature(test)]
extern crate test;
use test::Bencher;
use std::path::PathBuf;
use csv_challenge::{
    Opt,
    {load_csv, write_csv},
    replace_column,
};

#[bench]
fn bench_read_100times(b: &mut test::Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0,  |_,_|{test_load_csv();0})
    });
}

fn test_load_csv() {
    let filename = PathBuf::from("./input/challenge.csv");
    load_csv(filename);
}
```

> 注意：要使用基准测试，必须启用`#![feature(test)]`。但是只能在`rust nightly`版本中使用。

#### 7.2. 安装并使用`rust nightly`版本进行基准测试

- 安装`nightly`版本

```bash
rustup toolchian install nightly
rustup default nightly
```

- 运行基准测试

```bash
cargo bench
```

基准测试运行结果

```bash
     Running target/release/deps/file_op_bench-4a13631faeeecf0a

running 1 test
test bench_read_100times ... bench:   2,019,512 ns/iter (+/- 1,480,889)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out
```