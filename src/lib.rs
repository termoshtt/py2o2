pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_wit_parser() {
        let test_wit = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.wit");
        let unresolved = wit_parser::UnresolvedPackage::parse_file(&test_wit).unwrap();
        let mut wit = wit_parser::Resolve::new();
        wit.push(unresolved, &HashMap::new()).unwrap();
        for (id, contents) in &wit.interfaces {
            dbg!(id);
            dbg!(contents);
        }
        panic!();
    }
}
