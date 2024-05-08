use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    #[arg(long, default_value_t = false)]
    simple: bool,

    name: String,
}

fn main() {
    let args = Cli::parse();

    // TODO check validity of name
    //println!("{}", args.name);
    let cargo_toml_template = include_str!("../data/simple/Cargo.toml");
    let main_rs_template = include_str!("../data/simple/src/main.rs");
    let tests_rs_template = include_str!("../data/simple/src/tests.rs");
    if args.simple {
        std::fs::create_dir_all(&args.name).unwrap();
        let cargo_toml = cargo_toml_template.replace("NAME", &args.name);
        std::fs::write(format!("{}/Cargo.toml", &args.name), cargo_toml).unwrap();
        std::fs::write(format!("{}/.gitignore", &args.name), "/target\n").unwrap();
        std::fs::create_dir_all(format!("{}/src", &args.name)).unwrap();
        std::fs::write(format!("{}/src/main.rs", &args.name), main_rs_template).unwrap();
        std::fs::write(format!("{}/src/tests.rs", &args.name), tests_rs_template).unwrap();
    } else {
        eprintln!("Missing flag --simple");
    }

}