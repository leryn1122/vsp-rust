use substring::Substring;

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

    pub fn from_args(argc: usize, argv: Vec<String>) -> Vec<Opt> {
        let mut opts: Vec<Opt> = Vec::new();
        for i in 1 .. argc {
            let segment = argv.get(i).unwrap();
            if segment.starts_with("--") {
                opts.push(
                    Opt::by_name(
                        segment.substring(2, segment.len()))
                );
            } else {
                // Always ok here. Fetch last Opt in the queue and fulfill params.
                let mut opt = opts.last_mut().unwrap();
                opt.params.push(segment.to_string());
            }
        }
        println!("{:?}", opts);
        return opts
    }

}