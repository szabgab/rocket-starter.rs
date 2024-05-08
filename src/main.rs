use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    #[arg(long, default_value_t = false)]
    simple: bool,

    #[arg(long)]
    tera: bool,

    name: String,
}

fn main() {
    let args = Cli::parse();

    // TODO check validity of name
    //println!("{}", args.name);
    if args.simple {
        std::fs::create_dir_all(&args.name).unwrap();

        let cargo_toml_template = include_str!("../data/simple/Cargo.toml.skel");
        let main_rs_template = include_str!("../data/simple/src/main.rs");
        let tests_rs_template = include_str!("../data/simple/src/tests.rs");
     
        let cargo_toml = cargo_toml_template.replace("NAME", &args.name);

        std::fs::write(format!("{}/Cargo.toml", &args.name), cargo_toml).unwrap();
        std::fs::write(format!("{}/.gitignore", &args.name), "/target\n").unwrap();

        std::fs::create_dir_all(format!("{}/src", &args.name)).unwrap();
        std::fs::write(format!("{}/src/main.rs", &args.name), main_rs_template).unwrap();
        std::fs::write(format!("{}/src/tests.rs", &args.name), tests_rs_template).unwrap();
        return;
    }

    if args.tera {
        std::fs::create_dir_all(&args.name).unwrap();

        let cargo_toml_template = include_str!("../data/tera/Cargo.toml.skel");
        let main_rs_template = include_str!("../data/tera/src/main.rs");
        let tests_rs_template = include_str!("../data/tera/src/tests.rs");
        let templates_index_html = include_str!("../data/tera/templates/index.html.tera");
        let templates_404_html = include_str!("../data/tera/templates/404.html.tera");
        let templates_incl_header_html = include_str!("../data/tera/templates/incl/header.html.tera");
        let templates_incl_footer_html = include_str!("../data/tera/templates/incl/footer.html.tera");

        let cargo_toml = cargo_toml_template.replace("NAME", &args.name);

        std::fs::write(format!("{}/Cargo.toml", &args.name), cargo_toml).unwrap();
        std::fs::write(format!("{}/.gitignore", &args.name), "/target\n").unwrap();

        std::fs::create_dir_all(format!("{}/src", &args.name)).unwrap();
        std::fs::write(format!("{}/src/main.rs", &args.name), main_rs_template).unwrap();
        std::fs::write(format!("{}/src/tests.rs", &args.name), tests_rs_template).unwrap();

        std::fs::create_dir_all(format!("{}/templates/incl", &args.name)).unwrap();
        std::fs::write(format!("{}/templates/index.html.tera", &args.name), templates_index_html).unwrap();
        std::fs::write(format!("{}/templates/404.html.tera", &args.name), templates_404_html).unwrap();
        std::fs::write(format!("{}/templates/incl/header.html.tera", &args.name), templates_incl_header_html).unwrap();
        std::fs::write(format!("{}/templates/incl/footer.html.tera", &args.name), templates_incl_footer_html).unwrap();

        return;
    }

    eprintln!("Missing flag --simple or --tera");
 }
