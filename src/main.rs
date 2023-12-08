use clap::Parser;
use color_eyre::Result;

use aoc2023::Task;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.custom_input {
        let io = std::io::stdin().lock();
        aoc2023::prelude::CUSTOM_INPUT
            .set(std::io::read_to_string(io)?)
            .unwrap();
    }
    if cli.test {
        aoc2023::prelude::TEST_INPUT.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    println!("{}", cli.task.run()?);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    task: Task,
    #[arg(short, long)]
    custom_input: bool,
    #[arg(short, long)]
    test: bool,
}
