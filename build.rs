fn main() {
    for name in &[
        "runs",
        "users",
        "games",
        "categories",
        "levels",
    ] {
        let path = format!("data/normalized/{}.bin.xz", name);
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .expect("unable to create placeholder data file");
    }
}