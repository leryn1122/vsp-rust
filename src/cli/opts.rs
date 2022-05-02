/// Wrapper of cli options and parameters
///
/// ```bash
/// vspc --opt param_1 param_2
/// ```
///
/// return the result as
///
/// ```bash
/// Opts {
///   name: "opt",
///   params: ["param_1", "param_2"]
/// }
/// ```
///
#[derive(Debug)]
pub struct Opt {
    name: String,
    params: Vec<String>,
}

impl Opt {

    /// Generate an empty opt by its name.
    pub fn by_name(name: &str) -> Opt {
        Opt {
            name: name.to_string(),
            params: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_params(&self) -> &Vec<String> {
        &self.params
    }
}