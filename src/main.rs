use std::env;
use std::fs; //處理跟文件相關事務
use std::process; //處理執行續

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1) //離開程式
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    ///重構成用Result   
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enugh arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config{query, filename})
    }
}