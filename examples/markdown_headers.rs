use myrustutils::print_markdown_headers;
use std::path::PathBuf;

fn main() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    print_markdown_headers(&base.join("examples/example.md")).unwrap();
}
