use std::{env,process};
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("{}",err);
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    // let contents = fs::read_to_string(config.filename).expect("Couldnt read bro");
    // println!("Text = \n{}",contents);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }

}
