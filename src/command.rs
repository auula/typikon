use core::fmt;

#[derive(Debug)]
pub enum Command {
    Build,
    Theme,
    Init,
    Unknown(String),
}

impl Command {
    pub fn from_str(s: &str) -> Command {
        match s {
            "build" => Command::Build,
            "theme" => Command::Theme,
            "init" => Command::Init,
            _ => Command::Unknown(s.to_string()),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build => write!(f, "Build"),
            Command::Theme => write!(f, "Theme"),
            Command::Init => write!(f, "Init"),
            Command::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

pub fn handle_init() {}

pub fn handle_build() {
    // 处理 'build' 命令
    println!("执行 build 命令");
}

pub fn handle_theme(args: &[String]) {
    match args.get(2) {
        Some(option) => {
            if let Some(value) = option.strip_prefix("--get=") {
                println!("Fetching theme parameter: {}", value);
            } else {
                eprintln!("Unknown option: {}", option);
            }
        }
        None => {
            eprintln!("Lack of options: --get=<value>");
        }
    }
}
