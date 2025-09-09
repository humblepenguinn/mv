use crate::AppState;
use async_trait::async_trait;
use indexmap::IndexMap;
use mv_core::analyzer::AnalyzerState;
use tokio::sync::Mutex;

pub(crate) struct DesktopAnalyzerState<'a> {
    pub state: &'a Mutex<AppState>,
}

#[async_trait]
impl<'a> AnalyzerState for DesktopAnalyzerState<'a> {
    async fn get_starting_pointers(&mut self) -> IndexMap<String, usize> {
        let state = self.state.lock().await;
        let pointers_guard = state.starting_pointers.lock().await;

        pointers_guard.clone().unwrap_or_default()
    }

    async fn set_starting_pointers(&mut self, pointers: IndexMap<String, usize>) {
        let state = self.state.lock().await;
        *state.starting_pointers.lock().await = Some(pointers);
    }
}
