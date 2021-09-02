use std::env;
use std::fs; //處理跟文件相關事務

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args); //拆分出函數..

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

//重構成struct比較有可讀性
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    //使用clone..花一些性能確可以省去維護借用生命周期的麻煩..是值得的
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{query, filename}
}