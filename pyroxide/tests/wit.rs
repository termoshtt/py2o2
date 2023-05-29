use pyroxide::wit::*;

#[test]
fn example() {
    // Add a path where `example.py` exists
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("PYTHONPATH", project_root.join("tests"));

    let (wit, _path) = witgen("example").unwrap();
    insta::assert_snapshot!(wit, @r###"
        interface example {
        a1: func() 
        a2: func(x: s64) 
        a3: func(y: string, z: float64) 
        a4: func() -> s64
        a5: func(x: s64) -> string
        a6: func() -> (out0: s64, out1: string)
        a7: func(x: s64) -> (out0: s64, out1: string, out2: float64)
        }
        "###);
}
