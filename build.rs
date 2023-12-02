use camino::Utf8PathBuf;
use std::{
    env,
    io::{BufWriter, Write},
};

use fs_err as fs;

fn main() -> color_eyre::Result<()> {
    println!("cargo:rerun-if-changed=inputs");

    let inputs: Vec<_> = fs::read_dir("inputs")?
        .map(|entry_result| entry_result.unwrap())
        .map(|entry| Utf8PathBuf::from_path_buf(entry.path()).unwrap())
        .map(|path| {
            (
                path.file_stem().unwrap().to_string(),
                fs::read_to_string(&path).unwrap(),
            )
        })
        .collect();

    let out_path = Utf8PathBuf::from(env::var("OUT_DIR")?).join("inputs.rs");
    {
        let f = fs::File::create(out_path)?;
        let mut f = BufWriter::new(f);

        writeln!(&mut f, "#[derive(ValueEnum, Clone, Copy, Debug)]
        pub enum Task {{")?;
        for (day, _) in &inputs {
            writeln!(&mut f, "
                #[value(name = \"{day}a\")] Day{day}A,
                #[value(name = \"{day}b\")] Day{day}B,
            ")?;
        }
        writeln!(&mut f, "}}
        impl Task {{
            pub fn run(self) -> Result<String> {{
                match self {{")?;
        for (day, _) in &inputs {
            writeln!(&mut f, "
                Self::Day{day}A => Day{day}.solve_a(),
                Self::Day{day}B => Day{day}.solve_b(),
            ")?;
        }
        writeln!(&mut f, "
                }}
            }}
        }}")?;

        for (day, contents) in &inputs {
            writeln!(&mut f, "impl DayInput for Day{day} {{
                const CONTENTS: &'static str = {contents:?};
            }}")?;
        }

        drop(f.into_inner()?);
    }
    Ok(())
}
