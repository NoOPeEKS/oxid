use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use oxid_lsp::client::LspClient;
use oxid_lsp::types::{CompletionItem, CompletionList, Diagnostic, Hover};

use crate::buffer::Buffer;
use crate::events::EventKind;
use crate::ui::ui;

mod commands;
mod events;
mod lsp;
pub mod modes;

pub struct App {
    pub mode: modes::Mode,
    pub tsize_x: usize,
    pub tsize_y: usize,
    pub quitting: bool,
    pub buffers: Vec<Buffer>,
    pub current_buf_index: usize,
    pub registers: HashMap<String, String>,
    pub command: Option<String>,
    pub lsp_client: LspClient,
    pub diagnostics: Option<Vec<Diagnostic>>,
    pub completion_list: Option<CompletionList>,
    pub selected_completion: Option<CompletionItem>,
    pub hover: Option<Hover>,
    pub error: Option<String>,
    pub debug_mode: bool,
}

impl App {
    pub fn new(buffers: Vec<Buffer>, tsize_x: usize, tsize_y: usize) -> Self {
        let mut client = oxid_lsp::client::start_lsp().expect("Could not start LSP");
        client
            .initialize()
            .expect("Could not initialize the LSP Client");
        let file_path = buffers[0]
            .file_path
            .clone()
            .expect("Could not extract file path on init");
        client
            .did_open(&file_path, buffers[0].file_text.to_string().as_ref())
            .expect("Could not send initial textDocument/didOpen request.");
        App {
            mode: modes::Mode::Normal,
            tsize_x,
            tsize_y,
            quitting: false,
            buffers,
            current_buf_index: 0,
            registers: HashMap::from([(String::from("default"), String::new())]),
            command: None,
            lsp_client: client,
            diagnostics: None,
            completion_list: None,
            selected_completion: None,
            hover: None,
            debug_mode: true,
            error: None,
        }
    }

    pub fn run(
        &mut self,
        event_receiver: Receiver<EventKind>,
        terminal: &mut ratatui::DefaultTerminal,
    ) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if let Ok(event) = event_receiver.recv() {
                self.handle_event(event, terminal)?;
            }

            if self.quitting {
                return Ok(());
            }
        }
    }
}
