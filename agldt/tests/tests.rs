use agldt::parser::*;

#[test]
fn test_persname() {
    use serde_xml_rs::from_str;
    let src = r#"
            <editionStmt>
        <respStmt>
          <persName>Bridget Almas</persName>
          <resp>responsible for the annotation environment and cts:urn technology</resp>
          <address>Tufts University</address>
        </respStmt>
        <respStmt>
          <persName>
            <short>Vanessa Gorman</short>
            <name>Vanessa Gorman</name>
            <address>vbgorman@gmail.com</address>
            <uri>http://data.perseus.org/sosol/users/Vanessa%20Gorman</uri>
          </persName>
          <resp>annotator of the text</resp>
        </respStmt>
        </editionStmt>
            "#;
    assert!(from_str::<EditionStmt>(&preprocess(&src)).is_ok());
}

#[test]
fn test_serde() {
    use serde_xml_rs::from_str;
    use std::fs::read_to_string;
    let src = read_to_string("./tests/tlg0007.tlg004.perseus-grc1.tb.xml").unwrap();
    assert!(from_str::<Treebank>(&preprocess(&src)).is_ok());
}

#[test]
#[ignore]
fn test_serde_all() {
    use dotenv;
    use serde_xml_rs::from_str;
    use std::fs::{read_dir, read_to_string};

    dotenv::dotenv().ok();
    let paths = read_dir(dotenv::var("AGLDT_PATH").unwrap()).unwrap();
    for file in paths {
        let file = file.unwrap().path();
        let src = read_to_string(&file).unwrap();
        if !(from_str::<Treebank>(&preprocess(&src)).is_ok()) {
            dbg!(&file);
            from_str::<Treebank>(&preprocess(&src)).unwrap();
            panic!();
        }
        // assert!(from_str::<Treebank>(&preprocess(&src)).is_ok());
    }
}
