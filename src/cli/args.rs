use std::{env, process::exit};

use colored::Colorize;

use super::Command;

const BANNER: &str = r"
  _             _ _            
 | |_ _  _ _ __(_) |_____ _ _  
 |  _| || | '_ \ | / / _ \ ' \ 
  \__|\_, | .__/_|_\_\___/_||_|
      |__/|_|     v{}

Typikon lets you use markdown to write your online books.

Usage: typikon <command> [<args>...]

The commands are:

    init      Initialize directory to working directory
    build     Renderer static html file and output to book
    theme     Install the specified theme files

Use typikon help <command> for more information about a command.               
";

const VERSION: &str = "0.1.1-beta";

pub fn out_banner_string() {
    println!("{}",format!("{}", BANNER.replace("{}", VERSION).as_str().purple()))
}

// 解析命令行输入的参数，返回 Command 枚举和除了命令本身以外的参数
pub fn parse_args() -> (Command, Vec<String>) {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit(0)
    }

    let cmd = args[1].clone();
    let params = args[2..].to_vec();

    (Command::from_str(cmd.as_str()), params)
}


