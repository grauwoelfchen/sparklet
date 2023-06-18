#[allow(dead_code)]
fn main() {}

pub fn get_version() -> String {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    format!("{} v{}", pkg, ver)
}
