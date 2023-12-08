use std::{
    str::FromStr,
    sync::{atomic::AtomicBool, OnceLock},
};

pub use color_eyre::Result;
pub use itertools::Itertools;

pub static CUSTOM_INPUT: OnceLock<String> = OnceLock::new();
pub static TEST_INPUT: AtomicBool = AtomicBool::new(false);

pub trait DayInput {
    const CONTENTS: &'static str;
    const TEST_CONTENTS: Option<&'static str> = None;

    /// Helper function for Self::CONTENTS, because rust-analyzer no likey somehow.
    fn input(&self) -> &'static str {
        if let Some(custom_input) = CUSTOM_INPUT.get() {
            custom_input.as_str()
        } else if TEST_INPUT.load(std::sync::atomic::Ordering::SeqCst) {
            Self::TEST_CONTENTS.expect("task does not have a test input")
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

pub trait ParseNext<T: AsRef<str>>: Iterator<Item = T> {
    fn parse_next<U: FromStr>(&mut self) -> Option<U> {
        self.next().and_then(|s| s.as_ref().parse().ok())
    }
}
impl<T: AsRef<str>, I: Iterator<Item = T>> ParseNext<T> for I {}

pub trait PairFun {
    type Item;
    type I: IntoIterator<Item = Self::Item>;
    type SelfU<U>;
    fn via_iter<U, J: Iterator<Item = U>, F: FnOnce(<Self::I as IntoIterator>::IntoIter) -> J>(
        self,
        f: F,
    ) -> Self::SelfU<U>;
}

impl<T> PairFun for (T, T) {
    type Item = T;
    type I = [T; 2];

    type SelfU<U> = (U, U);

    fn via_iter<U, J: Iterator<Item = U>, F: FnOnce(<Self::I as IntoIterator>::IntoIter) -> J>(
        self,
        f: F,
    ) -> Self::SelfU<U> {
        f([self.0, self.1].into_iter()).collect_tuple().unwrap()
    }
}
