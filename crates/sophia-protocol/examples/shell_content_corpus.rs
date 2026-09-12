#[path = "support/shell_content_fixtures.rs"]
mod fixtures;

fn main() {
    let malformed = std::env::args().nth(1).as_deref() == Some("--malformed");
    if malformed {
        for (name, bytes) in fixtures::malformed() {
            emit(&name, &bytes);
        }
    } else {
        for (i, bytes) in fixtures::frames().iter().enumerate() {
            emit(&format!("content-{}", 160 + i), bytes);
        }
    }
}

fn emit(name: &str, bytes: &[u8]) {
    print!("{name} ");
    for byte in bytes {
        print!("{byte:02x}");
    }
    println!();
}
