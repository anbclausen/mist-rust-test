use mist_tools_rust::envelope::Envelope;
use mist_tools_rust::mime_types;
use mist_tools_rust::mist;
use mist_tools_rust::mist::reply_str_to_origin;
use mist_tools_rust::mist_tools::MistTools;
use std::env;

fn handle_action(_payload_bytes: Vec<u8>, _envelope: Envelope) -> Result<(), &'static str> {
    reply_str_to_origin("Hello from Rust", mime_types::TXT)
        .expect("Something went wrong in handle_action");
    Ok(())
}

fn main() {
    let args = env::args().collect();
    mist::service(args)
        .unwrap()
        .handle("action", handle_action)
        .init(|| Ok(println!("Hello, I started some Rust code")))
        .unwrap();
}
