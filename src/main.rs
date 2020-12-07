use anyhow::Result;
use env_logger;
use std::env;

mod days;

#[macro_use]
extern crate scan_rules;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    print_day(1);
    days::one::solve().await?;

    print_day(2);
    days::two::solve().await?;

    print_day(3);
    days::three::solve().await?;

    print_day(4);
    days::four::solve().await?;

    print_day(5);
    days::five::solve().await?;

    print_day(6);
    days::six::solve().await?;

    Ok(())
}

fn print_day(day: i32) {
    println!("--------- DAY {} ----------", day);
}
