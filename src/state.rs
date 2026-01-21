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

    pub fn build(&self, assume_math: bool) -> String {
        let initial: &str = if self.initial.is_true() {
            ", initial"
        } else {
            ""
        };
        let accepting: &str = if self.accepting {
            ", accepting"
        } else {
            ""
        };
        let mut positional_args: Vec<String> = Vec::new();
        self.positional_args.clone_into(&mut positional_args);
        let positional_args: String = positional_args.into_iter().fold(String::new(), |acc, e| {
            format!("{acc}, {e}")
        });
        let var = &self.var;
        let name: String = if let Some(s) = &self.name {
            if assume_math {
                format!("${s}$")
            } else {
                s.to_owned()
            }
        } else {
            "".to_string()
        };
        format!("\\node[state{initial}{accepting}{positional_args}] ({var}) {{{name}}};")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_state() {
        let state = State::from("s3", Some("$S_3$"), vec!["right of=s2"], StateFlags::Accepting).unwrap();
        assert_eq!(state, State {
            var: "s3".to_string(),
            name: Some("$S_3$".to_string()),
            positional_args: vec!["right of=s2".to_string()],
            initial: UniqueBool::new(),
            accepting: true
        })
    }

    #[test]
    fn test_build_state() {

    }
}