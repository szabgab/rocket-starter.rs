use std::env;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    for name in ["simple", "tera1"] {
        let dest_path = Path::new(&out_dir).join(format!("{name}.tar.gz"));
        zip(dest_path, PathBuf::from(format!("data/{name}")))?;
    }

    Ok(())
}

fn zip(filename: PathBuf, folder: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Zipping {folder:?} to {filename:?}");

    let tar_gz = std::fs::File::create(filename)?;
    let encoder = GzEncoder::new(tar_gz, Compression::default());

    let mut archive = Builder::new(encoder);
    archive.append_dir_all(".", folder)?;

    Ok(())
}
