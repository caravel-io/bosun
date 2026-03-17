use anyhow::Result;
use fact_rs;

fn main() -> Result<()> {
    fact_rs::build::run()?;
    Ok(())
}
