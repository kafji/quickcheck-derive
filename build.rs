use glob::glob;
use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    generate_trybuild_tests()
}

fn generate_trybuild_tests() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("trybuild_tests.rs");
    let mut out_file = File::create(out_path).unwrap();

    write!(
        out_file,
        r#"
            #[test]
            fn trybuild_tests() {{
                let t = trybuild::TestCases::new();
        "#
    )
    .unwrap();

    {
        let pat = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/buildable_pass_*.rs");
        let paths = glob(pat).unwrap();
        for path in paths {
            let path = path.unwrap();
            let path = path.display();
            write!(
                out_file,
                r#"
                    t.pass("{path}");
                "#
            )
            .unwrap();
        }
    }

    {
        let pat = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/buildable_fail_*.rs");
        let paths = glob(pat).unwrap();
        for path in paths {
            let path = path.unwrap();
            let path = path.display();
            write!(
                out_file,
                r#"
                    t.compile_fail("{path}");
                "#
            )
            .unwrap();
        }
    }

    write!(
        out_file,
        r#"
            }}
        "#
    )
    .unwrap();
}
