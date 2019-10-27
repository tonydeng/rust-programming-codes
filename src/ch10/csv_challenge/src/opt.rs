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
    pub output: Option<String>, // 使用Option表示可以忽略的参数
}