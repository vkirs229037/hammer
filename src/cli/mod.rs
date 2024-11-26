mod errors;
use errors::*;
use std::env::Args;
use regex::Regex;

enum Command {
    Compile,
    Run(RunType),
    Inspect
}

enum RunType {
    Source,
    Bytecode,
}

pub struct Cli {
    command: Command,
    in_file: String,
    out_file: Option<String>,
}

impl Cli {
    fn new(args: &mut Args) -> Result<Self, CliError> {
        let cli;
        let _program = args.next().expect("Невозможная ситуация: нет первого аргумента командной строки");
        let command = args.next().ok_or_else(|| CliError::NoCommand)?;
        let next_arg = args.next().ok_or_else(|| CliError::NoInputFile)?;
        let re = Regex::new(r"\..*$").expect("\\..*$ является верным регулярным выражением");
        if (next_arg == "-b") {
            if (command != "run") {
                return Err(CliError::IncorrectParam(command, next_arg));
            }
            let in_file = args.next().ok_or_else(|| CliError::NoInputFile)?;
            cli = Self { 
                command: Command::Run(RunType::Bytecode), 
                in_file: in_file.clone(),
                out_file: Some(String::from(re.replace(in_file.as_str(), ""))),
            }
        }
        else {
            let in_file = next_arg;
            let com_type;
            let out_file;
            match command.as_str() {
                "run" => {
                    com_type = Command::Run(RunType::Source);
                    out_file = Some(String::from(re.replace(in_file.as_str(), "")));
                },
                "compile" => {
                    com_type = Command::Compile;
                    out_file = args.next();
                },
                "inspect" => {
                    com_type = Command::Inspect;
                    out_file = None;
                }
                _ => return Err(CliError::UnknownCommand(command))
            }
            cli = Self {
                command: com_type,
                in_file,
                out_file,
            }
        }
        Ok(cli)
    }
}