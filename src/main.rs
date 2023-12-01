use clap::Parser;
use color_eyre::Result;

use aoc2023::Task;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.custom_input {
        let io = std::io::stdin().lock();
        aoc2023::CUSTOM_INPUT
            .set(std::io::read_to_string(io)?)
            .unwrap();
    }
    println!("{}", cli.task.run()?);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    task: Task,
    #[arg(long)]
    custom_input: bool,
}
