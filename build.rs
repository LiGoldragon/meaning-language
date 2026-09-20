use ethos_zero::{Actualizing, File, Generating, Potential};

fn main() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    println!("cargo:rerun-if-changed=ethos/meaning.ethos");
    println!("cargo:rerun-if-changed=src/generated/meaning.rs");
    let source = std::fs::read_to_string(root.join("ethos/meaning.ethos")).expect("source");
    let file = Potential::<File>::from(source).actualize().expect("read Meaning");
    let generated = file.generate().expect("generate Meaning");
    assert_eq!(generated, std::fs::read_to_string(root.join("src/generated/meaning.rs")).expect("generated"));
}
