use lexor_core::environment::{Environment, EnvironmentIO};
use lexor_core::evaluator::eval_program;
use lexor_core::lexer::Lexer;
use lexor_core::object::Object;
use lexor_core::parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RunResult {
    output: String,
    error: Option<String>,
}

#[wasm_bindgen]
impl RunResult {
    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        self.output.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

struct WasmIO {
    pub output_buffer: String,
    inputs: Vec<String>,
}

impl EnvironmentIO for WasmIO {
    fn read_line(&mut self) -> String {
        if self.inputs.is_empty() {
            String::new()
        } else {
            self.inputs.remove(0)
        }
    }

    fn print(&mut self, text: &str) {
        self.output_buffer.push_str(text);
    }
}

#[wasm_bindgen]
pub fn run_lexor(source_code: &str, inputs: Box<[JsValue]>) -> RunResult {
    let mapped_inputs: Vec<String> = inputs
        .iter()
        .filter_map(|val| val.as_string())
        .collect();

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Some(program) => {
            if !parser.errors.is_empty() {
                let err_str = parser.errors.join("\n");
                return RunResult {
                    output: String::new(),
                    error: Some(format!("FATAL SYNTAX ERRORS:\n{}", err_str)),
                };
            }

            let mut env = Environment::new();
            let mut io = WasmIO {
                output_buffer: String::new(),
                inputs: mapped_inputs,
            };
            let result = eval_program(&program, &mut env, &mut io);

            let error = if let Some(Object::Error(msg)) = result {
                Some(format!("FATAL RUNTIME ERROR:\n{}", msg))
            } else {
                None
            };

            RunResult {
                output: io.output_buffer,
                error,
            }
        }
        None => {
            let err_str = parser.errors.join("\n");
            RunResult {
                output: String::new(),
                error: Some(format!("FATAL PARSING FAILURE:\n{}", err_str)),
            }
        }
    }
}
