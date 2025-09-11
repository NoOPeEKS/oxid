use oxid_lsp::types::CompletionItem;

use super::App;

impl App {
    pub fn choose_completion(&mut self, completion: CompletionItem) {
        self.selected_completion = Some(completion);
    }
}
