use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    out_path = out_path.join("out");
    out_path.set_file_name("modder.rs");

    fs::write(
        out_path,
        format!(
            "
    #[path = \"{}\"]
    mod modded;",
            fs::canonicalize("modded/modded.rs")
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                .replace('\\', "\\\\")
        ),
    )
    .unwrap();
}
