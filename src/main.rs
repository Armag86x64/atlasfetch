use atlasfetch::print_module;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    let results = [
        print_module::parse_distro(),
        print_module::print_battery(),
        print_module::parse_gpu(),
        print_module::parse_memory(),
        print_module::print_cpu(),
        print_module::parse_disk(),
    ];

    if cli.debug {
        for result in results {
            if let Err(e) = result {
                eprintln!("\n{}", e);
            }
        }
    }
}
