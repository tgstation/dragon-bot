use std::thread;

fn main() {
    loop {
        thread::park();
    }
}
