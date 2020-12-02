use anyhow::Result;
use env_logger;
use std::env;

mod days;

#[tokio::main]
async fn main() -> Result<()>{
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    print_day(1);
    days::day1::solve().await?;

    Ok(())
}

fn print_day(day: i32) {
    println!("--------- DAY {} ----------", day);
}
