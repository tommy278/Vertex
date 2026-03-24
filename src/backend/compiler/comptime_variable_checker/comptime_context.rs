use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::compiler::functions_compiler_context::CompileTimeFunctionForCheck;
use crate::backend::errors::compiler::compiler_errors::CompileError;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    Bool, Int, StringValue, Void, Float
};
use crate::backend::errors::compiler::compiler_errors::CompileError::UndefinedType;
use std::collections::HashMap;
use crate::backend::ast::statements::structs::ComptimeStructForCheck;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
    pub functions: HashMap<String, CompileTimeFunctionForCheck>,
    pub scopes: Vec<HashMap<String, ComptimeVariable>>,
    pub structs:Vec<HashMap<String,ComptimeStructForCheck>>,
    pub types:Vec<String>,
    pub function_depth:usize,
    is_in_function_contex:bool,
    last_fn_context:usize,

}
impl CompileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            scopes: vec![HashMap::new()],
            types:Vec::new(),
            structs:Vec::new(),
            is_in_function_contex:false,
            last_fn_context:0,
            function_depth:0,

        }
    }
    pub fn add_type(&mut self,type_to_add:String)->Result<(),CompileError>{
        if !self.types.contains(&type_to_add) {
            self.types.push(type_to_add);
            return Ok(());
        }
        return Err(CompileError::TypeAlredyExists { name_of_type: type_to_add });
    }
    pub fn get_type(&self,type_to_identify: &str) -> Result<ComptimeValueType, CompileError> {
        match type_to_identify {
            "int" => Ok(Int),
            "string" => Ok(StringValue),
            "bool" => Ok(Bool),
            "void" => Ok(Void),
            "flt" => Ok(Float),
            _ => Err(UndefinedType {
                undefined_type: type_to_identify.to_string(),
            }),
        }
    }
    pub fn exit_scope(&mut self) {
        self.scopes
            .pop()
            .expect("Fatal error: stack underflow at compilation!");
    }
    pub fn enter_function_scope(&mut self){
        self.is_in_function_contex = true;
        self.last_fn_context = self.scopes.len();
        self.scopes.push(HashMap::new());
    }

    pub fn exit_function_scope(&mut self){
        self.is_in_function_contex = false;
        self.scopes.truncate(self.last_fn_context);
    }
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn add_variable(
        &mut self,
        name: String,
        variable: ComptimeVariable,
    ) -> Result<(), CompileError> {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            return Err(CompileError::VariableRecreation { name });
        } else {
            current_scope.insert(name, variable);
            Ok(())
        }
    }
    pub fn get_variable(&self, name: &str) -> Option<&ComptimeVariable> {
        let visible_scopes =
            if self.last_fn_context < self.scopes.len() {
                self.scopes.len() - self.last_fn_context
            } else {
                self.scopes.len()
            };

        for scope in self.scopes.iter().rev().take(visible_scopes) {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }

        None
    }
    pub fn add_function(
        &mut self,
        name: String,
        fnc: CompileTimeFunctionForCheck,
    ) -> Result<(), CompileError> {
        let curren_fnc_scope = &mut self.functions;
        if curren_fnc_scope.contains_key(&name).clone() {
            return Err(CompileError::FunctionAlredyExists { name });
        } else {
            curren_fnc_scope.insert(name, fnc);
            Ok(())
        }
    }
    pub fn get_fn(&mut self, name: &str) -> Result<CompileTimeFunctionForCheck, CompileError> {
        self.functions
            .get(name)
            .cloned()
            .ok_or(CompileError::UnknownFunction {
                name: name.to_string(),
            })
    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub tag: String,
    pub is_const: bool,
}
