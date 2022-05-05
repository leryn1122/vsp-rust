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
#[deprecated]
#[derive(Debug)]
pub struct Opt {
    pub name: String,
    pub params: Vec<String>,
}

#[allow(deprecated)]
#[deprecated]
impl Opt {

    /// Generate an empty opt by its name.
    pub fn by_name(name: &str) -> Self {
        Opt {
            name: name.to_string(),
            params: Vec::new(),
        }
    }
}