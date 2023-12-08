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
        .filter(|path| {
            path.file_stem()
                .and_then(|stem| stem.parse::<u32>().ok())
                .is_some()
        })
        .map(|path| {
            (
                path.file_stem().unwrap().to_string(),
                fs::read_to_string(&path).unwrap(),
                fs::read_to_string(path.with_file_name(format!("{}t", path.file_name().unwrap())))
                    .ok(),
            )
        })
        .collect();

    let out_path = Utf8PathBuf::from(env::var("OUT_DIR")?).join("generated.rs");
    {
        let f = fs::File::create(out_path)?;
        let mut f = BufWriter::new(f);
        writeln!(&mut f, "use prelude::*; use clap::ValueEnum;")?;

        writeln!(
            &mut f,
            "#[derive(ValueEnum, Clone, Copy, Debug)]
        pub enum Task {{"
        )?;
        for (day, _, _) in &inputs {
            writeln!(
                &mut f,
                "
                #[value(name = \"{day}a\")] Day{day}A,
                #[value(name = \"{day}b\")] Day{day}B,
            "
            )?;
        }
        writeln!(
            &mut f,
            "}}
        impl Task {{
            pub fn run(self) -> Result<String> {{
                match self {{"
        )?;
        for (day, _, _) in &inputs {
            writeln!(
                &mut f,
                "
                Self::Day{day}A => day{day}::Day{day}.solve_a(),
                Self::Day{day}B => day{day}::Day{day}.solve_b(),
            "
            )?;
        }
        writeln!(
            &mut f,
            "
                }}
            }}
        }}"
        )?;

        for (day, contents, test_contents) in &inputs {
            writeln!(
                &mut f,
                "impl DayInput for day{day}::Day{day} {{
                const CONTENTS: &'static str = {contents:?};"
            )?;
            if let Some(test_contents) = test_contents {
                writeln!(
                    &mut f,
                    "const TEST_CONTENTS: Option<&'static str> = Some({test_contents:?});"
                )?;
            }
            writeln!(&mut f, "}}")?;
        }

        drop(f.into_inner()?);
    }
    Ok(())
}
