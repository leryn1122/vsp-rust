#[warn(unused_variables)]
pub fn parse_config(_argc: usize, argv: Vec<String>) -> Opts {

    // if argc < 2 {
    //     print_usage_and_exit();
    // }

    let mut opts: Opts = Opts::new();

    for arg in argv {
        if "--help" == &arg {
            // print_usage_and_exit();
            opts.help = true;
        }

        if "--version" == &arg {
            // print_version_and_exit();
            opts.version = true;
        }

        if "--verbose" == &arg {
            opts.verbose = true;
        }

        if !&arg.starts_with("--") {
            let src = "vsp/test.vsp";
            opts.source = src.to_string().clone();
        }
    }

    return opts
}

#[derive(Debug)]
pub struct Opts {
    pub source: String,
    pub help: bool,
    pub version: bool,
    pub verbose: bool,
}

impl Opts {

    fn new() -> Opts {
        Opts {
            source: "".to_string(),
            help: false,
            version: false,
            verbose: false,
        }
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}