mod web_analyzer_state;

use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;

use mv_core::analyzer::Analyzer;
use mv_core::error::Error::{AnalyzerError, ParserError};
use mv_core::parser::Parser;

use crate::web_analyzer_state::WebAnalyzerState;

#[wasm_bindgen]
pub async fn analyze_source_code(input: String) -> String {
    let sanitized_source_code = input;

    let mut parser = Parser::new(&sanitized_source_code);
    let mut state = WebAnalyzerState::default();

    match parser.parse() {
        Ok(statements) => match Analyzer::default().analyze_statements(statements, &mut state).await {
            Ok(res) => serde_json::to_string(&json!({
                "stack": res.0,
                "heap": res.1,
            }))
            .unwrap(),

            Err(e) => match e {
                AnalyzerError(_, line_number, column_number) => {
                    return serde_json::to_string(&json!({
                        "error": {
                            "message": e.to_string(),
                            "line_number": line_number,
                            "column_number": column_number
                        }
                    }))
                    .unwrap();
                }

                _ => {
                    return serde_json::to_string(&json!({
                        "error": {
                            "message": e.to_string()
                        }
                    }))
                    .unwrap();
                }
            },
        },

        Err(e) => match e {
            ParserError(_, line_number, column_number) => {
                return serde_json::to_string(&json!({
                    "error": {
                        "message": e.to_string(),
                        "line_number": line_number,
                        "column_number": column_number
                    }
                }))
                .unwrap();
            }

            _ => {
                return serde_json::to_string(&json!({
                    "error": {
                        "message": e.to_string()
                    }
                }))
                .unwrap();
            }
        },
    }
}
