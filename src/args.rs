use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "fan-manager.yaml")]
    pub config: String,
}