use lexor_core::environment::{Environment, EnvironmentIO};
use lexor_core::evaluator::eval_program;
use lexor_core::lexer::Lexer;
use lexor_core::object::{LexorError, Object};
use lexor_core::parser::{Parser, validate_program};
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
    prompt_cancelled: bool,
}

impl EnvironmentIO for WasmIO {
    fn read_line(&mut self) -> String {
        if self.inputs.is_empty() {
            if !self.prompt_cancelled {
                let input = web_sys::window().and_then(|w| {
                    w.prompt_with_message("LEXOR Input Required:")
                        .ok()
                        .flatten()
                });

                if let Some(val) = input {
                    return val;
                }
                self.prompt_cancelled = true;
            }
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
#[allow(clippy::boxed_local)]
pub fn run_lexor(source_code: &str, inputs: Option<Box<[JsValue]>>) -> RunResult {
    let mapped_inputs = inputs
        .map(|values| {
            values
                .into_vec()
                .into_iter()
                .filter_map(|value| value.as_string())
                .collect()
        })
        .unwrap_or_default();

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

            // Semantic pass — catches undeclared SCAN targets before any code runs
            let semantic_errors = validate_program(&program);
            if !semantic_errors.is_empty() {
                return RunResult {
                    output: String::new(),
                    error: Some(format!("SEMANTIC ERRORS:\n{}", semantic_errors.join("\n"))),
                };
            }

            let mut env = Environment::new();
            let mut io = WasmIO {
                output_buffer: String::new(),
                inputs: mapped_inputs,
                prompt_cancelled: false,
            };
            let result = eval_program(&program, &mut env, &mut io);

            let error = if let Some(Object::Error(err)) = result {
                let category = match &err {
                    LexorError::TypeError { .. } => "TypeError",
                    LexorError::UndeclaredVariable { .. } => "UndeclaredVariable",
                    LexorError::DivisionByZero { .. } => "DivisionByZero",
                    LexorError::InvalidOperator { .. } => "InvalidOperator",
                    LexorError::InvalidAssignmentTarget { .. } => "InvalidAssignmentTarget",
                };
                Some(format!("RUNTIME ERROR: {}\n{}", category, err))
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
