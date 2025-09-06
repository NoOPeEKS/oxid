use anyhow::Result;
use ropey::Rope;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::channel;

use oxid::app::App;
use oxid::buffer::Buffer;
use oxid::events::{EventKind, handle_events};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let file_path = oxid::cli::get_file_name_arg()?;
    let file_text = Rope::from_reader(BufReader::new(File::open(&file_path)?))?;

    let tsize_x = terminal.size()?.width as usize;
    let tsize_y = terminal.size()?.height as usize;

    let mut buffers: Vec<Buffer> = Vec::new();
    buffers.push(Buffer::new(Some(file_path), file_text, tsize_x, tsize_y));

    let mut app = App::new(buffers);
    let (event_sender, event_receiver) = channel::<EventKind>();
    std::thread::spawn(move || handle_events(event_sender));
    let result = app.run(event_receiver, &mut terminal);

    ratatui::restore();
    result
}
