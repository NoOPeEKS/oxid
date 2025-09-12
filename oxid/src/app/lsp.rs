use super::App;

impl App {
    pub fn choose_completion(&mut self, comp_list_idx: usize) {
        if let Some(completion_list) = &self.completion_list {
            if let Some(comp_item) = completion_list.items.get(comp_list_idx) {
                self.selected_completion = Some(comp_item.clone());
            } else {
                self.selected_completion = None;
                self.error = Some("Not get items index".to_string());
            }
        } else {
            self.selected_completion = None;
            self.error = Some("No completion list yet".to_string());
        }
    }

    pub fn next_table_row(&mut self) {
        if let Some(completion_list) = &self.completion_list {
            let len = completion_list.items.len();
            if len == 0 {
                return;
            }

            let i = match self.table_state.selected() {
                Some(i) if i + 1 < len => i + 1,
                _ => len - 1,
            };

            self.table_state.select(Some(i));
            self.choose_completion(i);

            // keep offset within window
            let max_items = 6;
            if i >= self.completion_offset + max_items {
                self.completion_offset = i + 1 - max_items;
            }
        }
    }

    pub fn previous_table_row(&mut self) {
        if let Some(completion_list) = &self.completion_list {
            let len = completion_list.items.len();
            if len == 0 {
                return;
            }

            let i = match self.table_state.selected() {
                Some(i) if i > 0 => i - 1,
                _ => 0,
            };

            self.table_state.select(Some(i));
            self.choose_completion(i);

            // keep offset within window
            if i < self.completion_offset {
                self.completion_offset = i;
            }
        }
    }
}
