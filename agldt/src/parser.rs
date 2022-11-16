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
    body: Body,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Header {
    // #[serde(rename = "releaseDate")]
    // release_date: String,
    // #[serde(rename = "annotationDate")]
    // annotation_date: String,
    // #[serde(rename = "annotationScheme")]
    // annotation_scheme: String,
    // #[serde(rename = "fileDesc")]
    // file_desc: FileDesc,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct FileDesc {
//     #[serde(rename = "editionStmt")]
//     edition_stmt: EditionStmt,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct EditionStmt {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Body {
    #[serde(rename = "$value")]
    sentences: Vec<Sentence>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Sentence {
    id: u32,
    document_id: String,
    subdoc: String,
    #[serde(rename = "$value")]
    words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Word {
    id: u32,
    form: String,
    lemma: String,
    postag: String,
    relation: String,
    head: u32,
}

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
                <body>
                    <sentence id="1" document_id="urn:cts:greekLit:tlg0007.tlg004.perseus-grc1" subdoc="1.1">
                      <word id="1" form="περὶ" lemma="περί" postag="r--------" relation="AuxP" head="9"/>
                      <word id="2" form="Λυκούργου" lemma="Λυκοῦργος" postag="n-s---mg-" relation="ADV" head="1"/>
                      <word id="3" form="τοῦ" lemma="ὁ" postag="l-s---mg-" relation="ATR" head="2"/>
                      <word id="4" form="νομοθέτου" lemma="νομοθέτης" postag="n-s---mg-" relation="ATR" head="2"/>
                      <word id="5" form="καθόλου" lemma="καθόλου" postag="d--------" relation="ADV" head="8"/>
                      <word id="6" form="μὲν" lemma="μέν" postag="d--------" relation="AuxY" head="37"/>
                      <word id="7" form="οὐδὲν" lemma="οὐδείς" postag="p-s---na-" relation="OBJ" head="9"/>
                      <word id="8" form="ἔστιν" lemma="εἰμί" postag="v3spia---" relation="PRED_CO" head="37"/>
                      <word id="9" form="εἰπεῖν" lemma="εἶπον" postag="v--ana---" relation="SBJ" head="8"/>
                      <word id="10" form="ἀναμφισβήτητον" lemma="ἀναμφισβήτητος" postag="a-s---na-" relation="ATR" head="7"/>
                      <word id="11" form="," lemma="," postag="u--------" relation="AuxX" head="20"/>
                      <word id="12" form="οὗ" lemma="ὅς" postag="p-s---mg-" relation="ATR" head="20"/>
                      <word id="13" form="γε" lemma="γε" postag="d--------" relation="AuxZ" head="20"/>
                      <word id="14" form="καὶ" lemma="καί" postag="d--------" relation="AuxY" head="20"/>
                      <word id="15" form="γένος" lemma="γένος" postag="n-s---nn-" relation="SBJ_CO" head="20"/>
                      <word id="16" form="καὶ" lemma="καί" postag="d--------" relation="AuxY" head="20"/>
                      <word id="17" form="ἀποδημία" lemma="ἀποδημία" postag="n-d---fn-" relation="SBJ_CO" head="20"/>
                      <word id="18" form="καὶ" lemma="καί" postag="d--------" relation="AuxY" head="20"/>
                      <word id="19" form="τελευτὴ" lemma="τελευτή" postag="n-s---fn-" relation="SBJ_CO" head="20"/>
                      <word id="20" form="καὶ" lemma="καί" postag="c--------" relation="COORD" head="33"/>
                      <word id="21" form="πρὸς" lemma="πρός" postag="r--------" relation="AuxP" head="31"/>
                      <word id="22" form="ἅπασιν" lemma="ἅπας" postag="a-p---md-" relation="ADV" head="21"/>
                      <word id="23" form="ἡ" lemma="ὁ" postag="l-s---fn-" relation="ATR" head="31"/>
                      <word id="24" form="περὶ" lemma="περί" postag="r--------" relation="AuxP" head="31"/>
                      <word id="25" form="τοὺς" lemma="ὁ" postag="l-p---ma-" relation="ATR" head="26"/>
                      <word id="26" form="νόμους" lemma="νόμος" postag="n-p---ma-" relation="ATR_CO" head="28"/>
                      <word id="27" form="αὐτοῦ" lemma="αὐτός" postag="p-s---mg-" relation="ATR" head="28"/>
                      <word id="28" form="καὶ" lemma="καί" postag="c--------" relation="COORD" head="24"/>
                      <word id="29" form="τὴν" lemma="ὁ" postag="l-s---fa-" relation="ATR" head="30"/>
                      <word id="30" form="πολιτείαν" lemma="πολιτεία" postag="n-s---fa-" relation="ATR_CO" head="28"/>
                      <word id="31" form="πραγματεία" lemma="πραγματεία" postag="n-s---fn-" relation="SBJ_CO" head="20"/>
                      <word id="32" form="διαφόρους" lemma="διάφορος" postag="a-p---fa-" relation="ATR" head="34"/>
                      <word id="33" form="ἔσχηκεν" lemma="ἔχω" postag="v3plia---" relation="ATR" head="2"/>
                      <word id="34" form="ἱστορίας" lemma="ἱστορία" postag="n-p---fa-" relation="OBJ" head="33"/>
                      <word id="35" form="," lemma="," postag="u--------" relation="AuxX" head="33"/>
                      <word id="36" form="ἥκιστα" lemma="ἥκιστος" postag="a-p---na-" relation="ADV" head="45"/>
                      <word id="37" form="δὲ" lemma="δέ" postag="c--------" relation="COORD" head="0"/>
                      <word id="38" form="οἱ" lemma="ὁ" postag="l-p---mn-" relation="ATR" head="39"/>
                      <word id="39" form="χρόνοι" lemma="χρόνος" postag="n-p---mn-" relation="SBJ" head="45"/>
                      <word id="40" form="καθ̓" lemma="κατά" postag="r--------" relation="AuxP" head="42"/>
                      <word id="41" form="οὓς" lemma="ὅς" postag="p-p---ma-" relation="ADV" head="40"/>
                      <word id="42" form="γέγονεν" lemma="γίγνομαι" postag="v3sria---" relation="ATR" head="39"/>
                      <word id="43" form="ὁ" lemma="ὁ" postag="l-s---mn-" relation="ATR" head="44"/>
                      <word id="44" form="ἀνὴρ" lemma="ἀνήρ" postag="n-s---mn-" relation="SBJ" head="42"/>
                      <word id="45" form="ὁμολογοῦνται" lemma="ὁμολογέω" postag="v3ppie---" relation="PRED_CO" head="37"/>
                      <word id="46" form="." lemma="." postag="u--------" relation="AuxK" head="0"/>
                    </sentence>
                  </body>
            </treebank>
            "#;
        let tb: Treebank = from_str(src).unwrap();
        dbg!(tb);
    }
}
