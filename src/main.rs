use anyhow::Result;
use clap::Parser;
use backup::backup;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = backup::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = backup::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-backup operational");
    Ok(())
}
