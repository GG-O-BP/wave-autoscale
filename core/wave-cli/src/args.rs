/**
 * Command line arguments
 *
 * This module defines the command line arguments for the wave-autoscale binary.
 *
 */
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub plan: Option<String>,
    #[arg(short, long)]
    pub config: Option<String>,
    #[arg(short, long)]
    pub except_api_server: bool,
    #[arg(short, long)]
    pub run_web_app: bool,
}
