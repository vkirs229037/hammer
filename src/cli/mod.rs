pub mod errors;
use crate::{
    compile::{compiler::Compiler, errors::CompileError},
    error::HammerError,
    parser::{
        ast::{Ast, AstBuilder},
        lexer::Lexer,
    },
    vm::vm::VM,
};
use errors::*;
use regex::Regex;
use std::fs;
use std::{env::Args, io::Read, rc::Rc};

enum Command {
    Compile,
    Run(RunType),
    Inspect,
    Help,
}

enum RunType {
    Source,
    Bytecode,
}

pub struct Cli {
    command: Command,
    in_file: Option<String>,
    out_file: Option<String>,
}

impl Cli {
    pub fn usage() {
        println!("hammer <КОМАНДА> [ПАРАМЕТРЫ] ВХОДНОЙ_ФАЙЛ [ВЫХОДНОЙ_ФАЙЛ]");
        println!("Команды");
        println!("  compile <in> [out]      скомпилировать файл <in> (в файл [out], если задан)");
        println!("  run <in>                скомпилировать и запустить файл");
        println!("    run -b <in>           запустить файл с байткодом");
        println!("  inspect <in>            исследовать файл с байткодом (команды в байткоде + список констант)");
        println!("  help                    показать эту справку")
    }

    pub fn new(args: &mut Args) -> Result<Self, CliError> {
        let _program = args
            .next()
            .expect("Невозможная ситуация: нет первого аргумента командной строки");
        let command = args.next().ok_or(CliError::NoCommand)?;
        let next_arg = args.next();
        let re = Regex::new(r"\..*$").expect("\\..*$ является верным регулярным выражением");
        let cli = if next_arg.as_ref().is_some_and(|arg| arg == "-b") {
            if (command != "run") {
                return Err(CliError::IncorrectParam(command, next_arg.unwrap()));
            }
            let in_file = args.next().ok_or(CliError::NoInputFile)?;
            Self {
                command: Command::Run(RunType::Bytecode),
                in_file: Some(in_file.clone()),
                out_file: None,
            }
        } else {
            let in_file = match next_arg {
                Some(f) => Some(f),
                None => {
                    if command == "help" {
                        None
                    } else {
                        return Err(CliError::NoInputFile);
                    }
                }
            };
            let com_type;
            let out_file;
            match command.as_str() {
                "run" => {
                    com_type = Command::Run(RunType::Source);
                    out_file = Some(args.next().unwrap_or(String::from(
                        re.replace(in_file.clone().unwrap().as_str(), ""),
                    )));
                }
                "compile" => {
                    com_type = Command::Compile;
                    out_file = Some(args.next().unwrap_or(String::from(
                        re.replace(in_file.clone().unwrap().as_str(), ""),
                    )));
                }
                "inspect" => {
                    com_type = Command::Inspect;
                    out_file = None;
                }
                "help" => {
                    com_type = Command::Help;
                    out_file = None;
                }
                _ => return Err(CliError::UnknownCommand(command)),
            }
            Self {
                command: com_type,
                in_file,
                out_file,
            }
        };
        Ok(cli)
    }

    pub fn run(&self) -> Result<(), HammerError> {
        match &self.command {
            Command::Compile => {
                let mut input_file = match fs::OpenOptions::new()
                    .read(true)
                    .open(self.in_file.clone().unwrap())
                {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(HammerError::Compile(CompileError::FileError(
                            self.in_file.clone().unwrap(),
                            e,
                        )))
                    }
                };
                self.compile(&mut input_file)
            }
            Command::Run(rt) => {
                let mut input_file = match fs::OpenOptions::new()
                    .read(true)
                    .open(self.in_file.clone().unwrap())
                {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(HammerError::Compile(CompileError::FileError(
                            self.in_file.clone().unwrap(),
                            e,
                        )))
                    }
                };
                match rt {
                    RunType::Bytecode => self.interp(true),
                    RunType::Source => {
                        self.compile(&mut input_file)?;
                        self.interp(false)
                    }
                }
            }
            Command::Inspect => {
                todo!("Исследование файла байткода");
            }
            Command::Help => {
                Self::usage();
                Ok(())
            }
        }
    }

    fn compile(&self, input_file: &mut fs::File) -> Result<(), HammerError> {
        let mut program: String = String::new();
        input_file.read_to_string(&mut program);
        let mut lexer = Lexer::new(self.in_file.clone().unwrap(), program);
        match lexer.lex() {
            Ok(()) => {}
            Err(e) => {
                return Err(HammerError::Lex(e));
            }
        }

        let mut ast_builder = AstBuilder::new(lexer.tokens().to_vec());
        match ast_builder.parse() {
            Ok(()) => {}
            Err(e) => {
                return Err(HammerError::Parse(e));
            }
        };

        let Ast { tree, variables } = ast_builder.ast();
        let mut compiler = match Compiler::new(
            self.out_file
                .clone()
                .expect("При компиляции значение out_file всегда задано"),
        ) {
            Ok(c) => c,
            Err(e) => {
                return Err(HammerError::Compile(e));
            }
        };
        match compiler.compile(tree, variables) {
            Ok(()) => {
                let file = self
                    .out_file
                    .clone()
                    .expect("При компиляции значение out_file всегда задано");
                println!("Компиляция прошла успешно: {file}");
            }
            Err(e) => return Err(HammerError::Compile(e)),
        }
        Ok(())
    }

    fn interp(&self, b: bool) -> Result<(), HammerError> {
        let path = if b {
            self
                .in_file
                .clone()
                .expect("При запуске с -b значение in_file всегда задано")
        } else {
            self
                .out_file
                .clone()
                .expect("При запуске значение out_file всегда задано")
        };
        let mut file = match fs::OpenOptions::new().read(true).open(path.clone()) {
            Ok(f) => f,
            Err(e) => return Err(HammerError::Compile(CompileError::FileError(path, e))),
        };
        let mut bytecode: Vec<u8> = vec![];
        file.read_to_end(&mut bytecode);
        let mut vm = match VM::new(bytecode) {
            Ok(v) => v,
            Err(e) => return Err(HammerError::Bytecode(e)),
        };
        match vm.run() {
            Ok(()) => Ok(()),
            Err(e) => Err(HammerError::Interp(e)),
        }
    }
}
