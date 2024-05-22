use std::{error::Error, path::PathBuf};

use clap::Parser;
use flate2::bufread::GzDecoder;
use tar::Archive;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    #[arg(long, default_value_t = false)]
    simple: bool,

    #[arg(long)]
    tera1: bool,

    #[arg(long)]
    tera2: bool,

    #[arg(long)]
    tera_module: bool,

    name: String,
}

fn main() {
    let args = Cli::parse();

    let path = std::path::PathBuf::from(&args.name);
    if path.exists() {
        eprintln!("Folder {:?} already exists. Exiting.", &args.name);
        std::process::exit(1);
    }

    // TODO check validity of name
    //println!("{}", args.name);
    if args.simple {
        let skeleton = include_bytes!(concat!(env!("OUT_DIR"), "/simple.tar.gz"));
        crate_project(&args.name, skeleton);

        return;
    }

    if args.tera1 {
        let skeleton = include_bytes!(concat!(env!("OUT_DIR"), "/tera1.tar.gz"));
        crate_project(&args.name, skeleton);

        return;
    }

    if args.tera2 {
        let skeleton = include_bytes!(concat!(env!("OUT_DIR"), "/tera2.tar.gz"));
        crate_project(&args.name, skeleton);

        return;
    }

    if args.tera_module {
        let skeleton = include_bytes!(concat!(env!("OUT_DIR"), "/tera-module.tar.gz"));
        crate_project(&args.name, skeleton);

        return;
    }

    eprintln!("Missing flag --simple or --tera1 or --tera2 --tera-module");
}

fn crate_project(name: &str, skeleton: &[u8]) {
    std::fs::create_dir_all(name).unwrap();
    let folder = PathBuf::from(name);
    unzip(skeleton, &folder).unwrap();

    let cargo_toml_template = std::fs::read_to_string(folder.join("Cargo.toml.skel")).unwrap();
    std::fs::remove_file(folder.join("Cargo.toml.skel")).unwrap();

    let cargo_toml = cargo_toml_template.replace("NAME", name);
    std::fs::write(folder.join("Cargo.toml"), cargo_toml).unwrap();
    println!("Project created in {name:?}");
}

fn unzip(tar_gz: &[u8], folder: &PathBuf) -> Result<(), Box<dyn Error>> {
    //println!("Unzipping to {folder:?}");

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(folder)?;

    Ok(())
}
