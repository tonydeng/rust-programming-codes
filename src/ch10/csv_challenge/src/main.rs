use structopt::StructOpt;

use csv_challenge::{
    Opt,
    {load_csv,write_csv},
    replace_column,
};
// mod opt;
// use self::opt::Opt;
// mod err;
// mod core;
// use self::core::{
//     read::{load_csv,write_csv},
//     write::replace_column,
// };

use std::process;
use std::path::PathBuf;

fn main() {
    let opt = Opt::from_args();
    // 将opt.input字段中输入的CSV文件路径字符串转换成PathBuf类型
    let filename = PathBuf::from(opt.input);

    // 将得到的CSV文件路径filename传入load_csv函数中，使用match来处理load_csv返回的Result类型
    let csv_data = match load_csv(filename) {
        Ok(fname) => {fname},
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        },
    };

    // 使用replace_column方法替换原始CSV内容并生成新的修改过的内容modified_data
    let modified_data = match replace_column(csv_data, &opt.column_name, &opt.replacement) {
        Ok(data) => data,
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        },
    };

    //声明output变量绑定，代表输出csv文件路径，因为opt.output是可以忽略的参数，所以使用unwrap_or方法来定义默认输出路径
    let output_file = &opt.output.unwrap_or("output/output.csv".to_string());

    // 将督导的原始CSV文件内容csv_data和输出路径output_file传入write_csv方法中来输出CSV文件
    match write_csv(&modified_data, &output_file) {
        Ok(_) => {
            println!("write success!");
        },
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        },
    }
    // println!("{:?}",opt);
}


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