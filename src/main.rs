mod config;
use config::Environment;

fn main() {

    //pairs trading algorithm goes here
    
    let paper_config = config::Config::load(Environment::Paper);
    let production_config = config::Config::load(Environment::Production);
    println!("Hello World!");
    println!("{}", paper_config.alpaca_base_url);
    println!("{}", production_config.alpaca_base_url);
}
