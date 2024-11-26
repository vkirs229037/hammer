pub mod errors;
use errors::*;
use std::{env::Args, io::Read};
use regex::Regex;
use std::fs;
use crate::{compile::{compiler::Compiler, errors::CompileError}, error::HammerError, parser::{ast::AstBuilder, lexer::Lexer}, vm::vm::VM};

enum Command {
    Compile,
    Run(RunType),
    Inspect,
    Help
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
                    out_file = Some(args.next().unwrap_or(String::from(re.replace(in_file.as_str(), ""))));
                },
                "inspect" => {
                    com_type = Command::Inspect;
                    out_file = None;
                },
                "help" => {
                    com_type = Command::Help;
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

    pub fn run(&self) -> Result<(), HammerError> {
        let mut input_file = match fs::OpenOptions::new().read(true).open(self.in_file.clone()) {
            Ok(f) => f,
            Err(e) => return Err(HammerError::Compile(CompileError::FileError(self.in_file.clone(), e))),
        };
        match &self.command {
            Command::Compile => {
                self.compile(&mut input_file)
            },
            Command::Run(rt) => {
                match rt {
                    RunType::Bytecode => {
                        self.interp()
                    },
                    RunType::Source => {
                        self.compile(&mut input_file)?;
                        self.interp()
                    }
                }
            },
            Command::Inspect => {
                todo!("Исследование файла байткода");
            },
            Command::Help => {
                Self::usage();
                Ok(())
            }
        }
    }

    fn compile(&self, input_file: &mut fs::File) -> Result<(), HammerError> {
        let mut program: String = String::new();
        input_file.read_to_string(&mut program);
        let mut lexer = Lexer::new("module".to_owned(), program);
        match lexer.lex() {
            Ok(()) => { }
            Err(e) => { 
                println!("{e}");
                return Err(HammerError::Lex(e));
            }
        }

        let mut ast_builder = AstBuilder::new(lexer.tokens().to_vec());
        match ast_builder.parse() {
            Ok(()) => { }
            Err(e) => {
                println!("Ошибка: {e}");
                return Err(HammerError::Parse(e));
            }
        };

        let tree = ast_builder.tree();
        let mut compiler = match Compiler::new(&tree, self.out_file.clone().expect("При компиляции значение out_file всегда задано")) {
            Ok(c) => c,
            Err(e) => {
                println!("Ошибка: {e}"); 
                return Err(HammerError::Compile(e));
            }
        };
        match compiler.compile() {
            Ok(()) => {
                let file = self.out_file.clone().expect("При компиляции значение out_file всегда задано");
                println!("Компиляция прошла успешно: {file}");
            },
            Err(e) => {
                println!("Ошибка: {e}");
                return Err(HammerError::Compile(e))
            }
        }
        Ok(())
    }

    fn interp(&self) -> Result<(), HammerError> {
        let path = self.out_file.clone().expect("При запуске значение out_file всегда задано");
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