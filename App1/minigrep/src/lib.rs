use std::{fs, error::Error, env};

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    // println!("Content \n {}",contents);
    let results = if config.case {
        search(&config.query,&contents);
    }else{
        search_case_insensitive(&config.query, &contents);
    };
    for line in search(&config.query,&contents){
        println!("{}",line);
    }
    Ok(())
}
pub struct Config{
    pub query:String,
    pub filename:String,
    pub case:bool
}
impl Config{

    pub fn new(args:&[String])->Result<Config,&str>{
        if args.len()<3{
            return Err("Not enough arguments, try : cargo run query filename.txt");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config{query,filename,case});
    }
}
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str)-> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    return results;
}

pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str>{
    let mut results  = Vec::new();
    // contents.lines().for_each(|line| {
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // });
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    return results;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust
        safe, fast, productive.
        ";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
}