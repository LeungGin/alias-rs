mod cli;
mod cmn;
mod core;
mod support;

fn main() {
    if let Err(e) = cli::parse() {
        println!("got some trouble, err={:?}, msg={}", e.kind, e.msg);
    }
}
