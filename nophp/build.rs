fn get_file(url: &str, name: &str) {
    let out = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out).join(name);
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut file = std::fs::File::create(&path).unwrap();
    std::io::copy(&mut resp, &mut file).unwrap();
}

fn main() {
    let base = "https://raw.githubusercontent.com/ByteForIT/NoPHP";
    // let pin = "7db4209"; // main
    let pin = "e962c0d"; // alpha-2.0

    let lexer = "lexer.py";
    let parser = "pparser.py";

    get_file(&format!("{base}/{pin}/nophp/lang/{lexer}"), lexer);
    get_file(&format!("{base}/{pin}/nophp/lang/{parser}"), parser);
}
