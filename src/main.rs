use anyhow::Result;
use clap::Parser;
use fact_rs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .init();
    }

    fact_rs::run()?;
    Ok(())
}
