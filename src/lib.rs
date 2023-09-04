use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("need two arguments, first argument is the query, second argument is the file location");
        }

        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config { query, file_path } = config;

    println!("query: {}", query);
    println!("file_path: {}", file_path);

    let file_contents = fs::read_to_string(file_path)?;

    println!("file_contents:\n\n{file_contents}");

    Ok(())
}
