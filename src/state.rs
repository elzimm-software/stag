use std::cmp::PartialEq;
use crate::unique_bool::UniqueBool;

#[derive(Debug, PartialEq, Eq)]
pub enum StateFlags {
    None,
    Accepting,
    Initial,
    AcceptingAndInitial,
}

impl StateFlags {
    pub fn is_initial(&self) -> bool {
        self == &StateFlags::Initial || self == &StateFlags::AcceptingAndInitial
    }

    pub fn is_accepting(&self) -> bool {
        self == &StateFlags::Accepting || self == &StateFlags::AcceptingAndInitial
    }
}

#[derive(Debug, PartialEq)]
pub struct State {
    var: String,
    name: Option<String>,
    positional_args: Vec<String>,
    initial: UniqueBool,
    accepting: bool,
}

impl State {
    pub fn from<S: Into<String>>(var: S, name: Option<S>, positional_args: Vec<S>, flags: StateFlags) -> Result<Self, ()> {
        let mut init_unique = UniqueBool::new();
        unsafe {init_unique.try_set(flags.is_initial())?}
        Ok(Self {
            var: var.into(),
            name: if let Some(s) = name {
                Some(s.into())
            } else {
                None
            },
            positional_args: positional_args.into_iter().map(|x| x.into()).collect(),
            initial: init_unique,
            accepting: flags.is_accepting(),
        })
    }
}