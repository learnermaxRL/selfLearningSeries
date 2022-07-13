use dotenv::dotenv;
use std::env;



fn main() {
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}:{}", key, value);
    }
    println!("Value of size is {}",
    env::var("size").unwrap());

    
    for argument in env::args() {
        println!("{}", argument);
        }

}