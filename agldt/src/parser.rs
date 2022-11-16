use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Event {
    Treebank(Treebank),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Treebank {
    version: String,
    #[serde(rename = "xml_lang")]
    lang: String,
    cts: String,
    header: Header,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Header {
    #[serde(rename = "releaseDate")]
    release_date: String,
    #[serde(rename = "annotationDate")]
    annotation_date: String,
    #[serde(rename = "annotationScheme")]
    annotation_scheme: String,
    #[serde(rename = "fileDesc")]
    file_desc: FileDesc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FileDesc {
    #[serde(rename = "editionStmt")]
    edition_stmt: EditionStmt,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EditionStmt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        use serde_xml_rs::from_str;
        let src = r#"
            <treebank version="2.1" xml_lang="grc" cts="urn:cts:greekLit:tlg0007.tlg004.perseus-grc1.tb">
                <header>
                <releaseDate>25 September 2015</releaseDate>
                <annotationDate>Sat Jan 31 16:11:24 +0000 2015</annotationDate>
                <annotationScheme>Guidelines for the syntactic annotation of the Ancient Greek dependency treebank 1.1</annotationScheme>
                <fileDesc>
                      <editionStmt>
                        <respStmt>
                          <persName>Giuseppe G. A. Celano</persName>
                          <resp>release editor: post-annotation normalization and harmonization</resp>
                          <address>Leipzig University</address>
                        </respStmt>
                        <respStmt>
                          <persName>Gregory R. Crane</persName>
                          <resp>supervisor</resp>
                          <address>Leipzig University and Tufts University</address>
                        </respStmt>
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
                        <respStmt>
                          <persName>
                            <short>Sean Stewart</short>
                            <name>Sean Stewart</name>
                            <address>sean.theodore.stewart@gmail.com</address>
                            <uri>http://data.perseus.org/sosol/users/Sean%20Stewart</uri>
                          </persName>
                          <resp>annotator of the text</resp>
                        </respStmt>
                        <respStmt>
                          <persName>
                            <short>arethusa</short>
                            <name>arethusa</name>
                            <address/>
                            <uri>http://github.com/latin-language-toolkit/arethusa</uri>
                          </persName>
                          <resp>annotator of the text</resp>
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
                      <biblStruct>
                        <monogr>
                          <author>Plutarch</author>
                          <title>Plutarch's Lives</title>
                          <respStmt>
                            <resp>with an English Translation by</resp>
                            <name>Bernadotte Perrin</name>
                          </respStmt>
                          <imprint>
                            <pubPlace>Cambridge, MA</pubPlace>
                            <publisher>Harvard University Press</publisher>
                            <pubPlace>London</pubPlace>
                            <publisher>William Heinemann Ltd.</publisher>
                            <date>1914</date>
                          </imprint>
                          <biblScope type="volume">1</biblScope>
                          <biblScope>Lycurgus</biblScope>
                        </monogr>
                      </biblStruct>
                    </fileDesc>
                </header>
            </treebank>
            "#;
        let tb: Treebank = from_str(src).unwrap();
        dbg!(tb);
    }
}
