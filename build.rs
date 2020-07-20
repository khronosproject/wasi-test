fn generate_spec(src_dir: &std::path::Path, out_dir: &std::path::Path) {
    let entries = std::fs::read_dir(src_dir).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let source = String::from_utf8(std::fs::read(entry.path()).unwrap()).unwrap();
            let prelude = source.split("\n\n").next().unwrap().replace("//", "");

            let out_path = out_dir.join(
                entry
                .file_name()
                .into_string()
                .unwrap()
                .replace(".rs", ".json"),
                );

            let out_data = prelude.as_bytes();

            std::fs::write(out_path, out_data).unwrap();
        }
    }
}

fn main() {
    let src_dir = std::path::PathBuf::from("src/bin");
    let out_dir = std::path::PathBuf::from(
        std::env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set"),
    );

    let entries = std::fs::read_dir(&src_dir).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path().into_os_string().into_string().unwrap();
            println!("cargo:rerun-if-changed={}", path);
        }
    }

    generate_spec(&src_dir, &out_dir);
}
