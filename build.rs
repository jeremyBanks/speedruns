use std::{fs, io};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    fs::create_dir_all(&format!("{}/data/normalized", out_dir))
        .expect("unable to create normalized data directory");

    for name in &["runs", "users", "games", "categories", "levels"] {
        let path = format!("data/normalized/{}.bin.xz", name);
        let out_path = format!("{}/{}", out_dir, path);

        let mut out_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&out_path)
            .expect("unable to open/create normalized data file");

        if let Ok(mut source_file) = fs::OpenOptions::new().read(true).open(&path) {
            io::copy(&mut source_file, &mut out_file)
                .expect("failed to copy normalized data");
        } else {
            panic!("expected to find source files!");
        }
    }
}
