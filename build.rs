extern crate peg;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::io::*;

#[allow(dead_code)]
mod ast {
    include!("src/ast.rs");
}

macro_rules! TEST_SOUCE { () => ("
// {}
#[test]
fn {} () {{
    let input = {:?};
    let target_source = {:?};

    let result = parse(&input)
        .expect(\"Parsing of the input for {} failed!\");
    let target: ast::Element = serde_yaml::from_str(&target_source)
        .expect(\"Parsing the documentation of {} failed!\");
    assert_eq!(&target, &result,
        \"comparing documentation (left) with parse result (right) failed!\");
}}
")}

macro_rules! TEST_HEADER { () => ("
// THIS DOCUMENT IS AUTO-GENERATED AND SHOULD NOT BE EDITED BY HAND!

use ast;
use serde_yaml;
use parse;

")}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    case: String,
    input: String,
    out: ast::Element,
}

fn escape_test_name(input: String) -> String {
    input.replace(" ", "_").to_lowercase()
}

impl Test {
    fn write_code(&self, file: &mut fs::File) -> Result<()> {
        writeln!(
            file,
            TEST_SOUCE!(),
            self.case,
            escape_test_name(self.case.clone()),
            self.input,
            serde_yaml::to_string(&self.out).expect("Error serializing test input!"),
            self.case,
            self.case
        )
    }
}

fn generate_tests() {
    // tell cargo to rerun if the documentation changes.
    println!("cargo:rerun-if-changed=doc/docs.yml");
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let out_path = out_dir
        .join(Path::new("tests_generated.rs"))
        .with_extension("rs");

    let mut in_file = fs::File::open(Path::new("doc/docs.yml")).ok().expect(
        "Could not open input file!",
    );
    let mut out_file = fs::File::create(Path::new(&out_path)).ok().expect(
        "Could not open output file!",
    );

    let mut content = String::new();
    in_file.read_to_string(&mut content).ok().expect(
        "Could not read file!",
    );

    let tests: Vec<Test> =
        serde_yaml::from_str(&content).expect("Could not parse the documentation!");

    write!(out_file, TEST_HEADER!()).unwrap();

    for test in &tests {
        test.write_code(&mut out_file).unwrap();
    }
}

fn main() {
    peg::cargo_build("src/grammar.rustpeg");
    generate_tests();
}
