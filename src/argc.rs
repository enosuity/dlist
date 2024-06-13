use clap::{arg, Parser};

const TOP_NUM:usize = 5;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(short, long, default_value_t = TOP_NUM)]
  pub nums: usize,
}
