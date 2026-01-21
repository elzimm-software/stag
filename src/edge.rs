
pub struct Edge {
    var_start: String,
    var_end: String,
    positional_args: Vec<String>,
    edge_text: Option<String>,
}

impl Edge {
    pub fn from(var_start: &str, var_end: &str, positional_args: &[String], edge_text: Option<&str>) -> Result<Self, ()> {
        Ok(Self {
            var_start: var_start.to_string(),
            var_end: var_end.to_string(),
            positional_args: positional_args.to_vec(),
            edge_text: if let Some(s) = edge_text {
                Some(s.to_string())
            } else {
                None
            },
        })
    }
}