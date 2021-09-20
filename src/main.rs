use minigrep::Config;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //離開程式
    });

    //因為成功時返回的是空元組..所以不用unwrap..直接用if let看有沒錯誤
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

