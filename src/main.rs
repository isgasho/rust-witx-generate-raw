use std::env;
use std::path::PathBuf;

fn main() {
    let witx_path: PathBuf = env::args_os().nth(1).unwrap().into();
    print!("{}", witx_generate_raw::generate(&[witx_path]));
}
