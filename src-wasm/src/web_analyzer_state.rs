use async_trait::async_trait;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use web_sys::window;

use mv_core::analyzer::AnalyzerState;

const STARTING_POINTERS_KEY: &str = "starting_pointers";

#[derive(Default, Serialize, Deserialize)]
pub struct WebAnalyzerState {
    starting_pointers: IndexMap<String, usize>,
}

#[async_trait]
impl AnalyzerState for WebAnalyzerState {
    async fn get_starting_pointers(&mut self) -> IndexMap<String, usize> {
        if let Some(win) = window() {
            if let Some(storage) = win.local_storage().ok().flatten() {
                if let Ok(Some(value)) = storage.get_item(STARTING_POINTERS_KEY) {
                    if let Ok(pointers) = serde_json::from_str::<IndexMap<String, usize>>(&value) {
                        self.starting_pointers = pointers;
                    }
                }
            }
        }
        self.starting_pointers.clone()
    }

    async fn set_starting_pointers(&mut self, pointers: IndexMap<String, usize>) {
        self.starting_pointers = pointers.clone();

        if let Some(win) = window() {
            if let Some(storage) = win.local_storage().ok().flatten() {
                if let Ok(json) = serde_json::to_string(&pointers) {
                    let _ = storage.set_item(STARTING_POINTERS_KEY, &json);
                }
            }
        }
    }
}
