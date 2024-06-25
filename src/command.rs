pub const BUILD: &str = "build";
pub const THEME: &str = "theme";

pub enum Command {
    Build,
    Theme,
    Unknown(String),
}

impl Command {

    pub fn from_str(s: &str) -> Command {
        match s {
            "build" => Command::Build,
            "theme" => Command::Theme,
            _ => Command::Unknown(s.to_string()),
        }
    }
}


pub fn handle_build_command() {
    // 处理 'build' 命令
    println!("执行 build 命令");
}

pub fn handle_theme_command(args: &[String]) {
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