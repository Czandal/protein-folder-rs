use std::env;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Config {
    pub p1: f64,
    pub p2: f64,
    pub input_mode: bool,
}

const DEFAULT_CONFIG: Config = Config {
    p1: 0.5,
    p2: 0.99,
    input_mode: true,
};

pub fn load_config() -> Config {
    let mut args = env::args().skip(1);
    let mut config = DEFAULT_CONFIG.clone();
    while let Some(arg) = args.next() {
        match &arg[..] {
            // TODO: HELP, VERSION
            "-h" | "--help" => {}
            "-q" | "--quiet" => {
                println!("Quiet mode is not supported yet.");
            }
            "-v" | "--verbose" => {
                println!("Verbose mode is not supported yet.");
            }
            "--probability1" | "-p1" | "-P1" => {
                if let Some(arg_config) = args.next() {
                    config.p1 = arg_config
                        .parse::<f64>()
                        .expect("P1 must be floating point number");
                } else {
                    panic!("No value specified for parameter --config.");
                }
            }
            "--probability2" | "-p2" | "-P2" => {
                if let Some(arg_config) = args.next() {
                    config.p2 = arg_config
                        .parse::<f64>()
                        .expect("P2 must be floating point number");
                } else {
                    panic!("No value specified for parameter --config.");
                }
            }
            "-b" | "--bench" => {
                config.input_mode = false;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unkown argument {}", arg);
                } else {
                    println!("Unkown positional argument {}", arg);
                }
            }
        }
    }
    return config;
}
