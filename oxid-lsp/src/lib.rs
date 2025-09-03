use std::sync::atomic::AtomicUsize;

use crate::types::*;

pub mod client;
pub mod types;
pub mod capabilities;

static ID: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> usize {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
