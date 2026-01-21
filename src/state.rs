use crate::only_bool::UniqueBool;

pub struct State {
    var: String,
    name: Option<String>,
    initial: UniqueBool,
    accepting: bool,
}

impl State {
    pub fn from(var: &str, name: Option<&str>, initial: bool, accepting: bool) -> Result<Self, ()> {
        let mut init_unique = UniqueBool::new();
        unsafe {init_unique.try_set(initial)?}
        Ok(Self {
            var: var.to_string(),
            name: if let Some(s) = name {
                Some(s.to_string())
            } else {
                None
            },
            initial: init_unique,
            accepting,
        })
    }
}