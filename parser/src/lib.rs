
extern crate core;

use std::fmt::Display;
use lalrpop_util::lalrpop_mod;

pub mod ast;
mod chumsky;

pub use ast::*;
use crate::chumsky::{parser};
use ::chumsky::Parser;
use crate::parser::{ExpressionParser};

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

pub fn parse(input: &str) -> Result<Expression, ParseError> {
    // Wrap the internal parser function - whether larlpop or chumsky
    println!("Parsing: >>>{}<<<", input);

    if true {
        // New chumsky version
        parser().parse(input)
            .into_result()
            .map_err(|e|  {
                ParseError {
                    msg: e.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join("\n")
                }
            })

    } else {
        // Existing Larlpop Parser:
        ExpressionParser::new()
            .parse(input)
            .map_err(|e| ParseError {
                msg: format!("{}", e),
            })
    }
}


lalrpop_mod!(#[allow(clippy::all)] pub parser, "/cel.rs");
