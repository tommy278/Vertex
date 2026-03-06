use std::{fs::{read_to_string}, path::PathBuf};

use crate::backend::{errors::compiler::compiler_errors::CompileError, lexer::tokenizer::Tokenizer};

pub struct Linker{
    pub main_file:PathBuf, 
    pub final_name:String
}

impl Linker {
    fn new(main_file:PathBuf,final_name:String)->Self{
        Self{
            main_file,
            final_name
        }
    }
    pub fn link(&mut self)->Result<(),CompileError>{
        let main_file_content = read_to_string(&self.main_file).unwrap();
        let mut lexer = Tokenizer::new(main_file_content);
        let tokens = lexer.tokenize().unwrap();
        Ok(())
    }
    
}
