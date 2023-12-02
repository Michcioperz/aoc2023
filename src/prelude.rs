use std::sync::OnceLock;

pub use color_eyre::Result;

pub static CUSTOM_INPUT: OnceLock<String> = OnceLock::new();

pub trait DayInput {
    const CONTENTS: &'static str;

    /// Helper function for Self::CONTENTS, because rust-analyzer no likey somehow.
    fn input(&self) -> &str {
        if let Some(custom_input) = CUSTOM_INPUT.get() {
            custom_input.as_str()
        } else {
            Self::CONTENTS
        }
    }
}

pub trait TaskA: DayInput {
    fn solve_a(&self) -> Result<String> {
        todo!()
    }
}

pub trait TaskB: TaskA {
    fn solve_b(&self) -> Result<String> {
        todo!()
    }
}
