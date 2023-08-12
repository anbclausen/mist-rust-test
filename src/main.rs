use mist_tools::{mime_types, mist_service, reply_str_to_origin, Envelope};

fn main() {
    mist_service!({actions: {"action": handle_action}})
}

fn handle_action(_payload_bytes: Vec<u8>, _envelope: Envelope) -> Result<(), String> {
    reply_str_to_origin("Hello from Rust", mime_types::TXT)
        .expect("Something went wrong in handle_action");
    Ok(())
}
