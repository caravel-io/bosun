use anyhow::Result;
use bosun;

fn main() -> Result<()> {
    bosun::build::run()?;
    Ok(())
}
