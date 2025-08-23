use std::sync::atomic::AtomicUsize;

pub mod client;
pub mod types;

static ID: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> usize {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
