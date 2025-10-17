use atento_core;

fn main() {
    match std::env::args().nth(1) {
        Some(a) => match a.as_str() {
            "-h" | "--help" => {
                print_help();
            }
            "-V" | "--version" => {
                print_version();
            }
            _ => {
                atento_core::run(&a);
            }
        },
        None => {
            print_help();
        }
    };
}

fn print_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),)
}

fn print_help() {
    println!(
        "{} v{}\n\nUsage:\n\
            {} <filename_path>\n\n\
            Options:\n\
            -h, --help     Print this help\n\
            -V, --version  Print version information",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    );
}
