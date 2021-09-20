use std::env;
use std::fs;
use std::process;
use std::error::Error; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //離開程式
    });

    //因為成功時返回的是空元組..所以不用unwrap..直接用if let看有沒錯誤
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
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