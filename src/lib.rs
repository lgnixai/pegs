pub mod ast;
pub mod parser;
mod context;

mod engine;
mod api;
mod types;
mod vm;
mod object;
mod package;




pub use engine::{Engine};

pub use rhai_codegen::*;
pub use types::{
    EvalAltResult,Scope
};

type ERR = EvalAltResult;
/// General evaluation error for Rhai scripts.
type RhaiError = Box<ERR>;
/// Generic [`Result`] type for Rhai functions.
type RhaiResultOf<T> = Result<T, RhaiError>;
