use anyhow::Result;
use factrs;

fn main() -> Result<()> {
    factrs::build::run()?;
    Ok(())
}
