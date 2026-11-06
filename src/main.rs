use ::backup::{backup, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = backup::new(config);
    component.initialize()?;
    component.run()?;
    println!("SigmaOS-backup operational");
    Ok(())
}
