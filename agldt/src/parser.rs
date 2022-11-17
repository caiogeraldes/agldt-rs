use regex::Regex;
use serde::{Deserialize, Serialize};

pub fn preprocess(src: &str) -> String {
    let re_xmlcolon = Regex::new(r"xml:").unwrap();
    let src = re_xmlcolon.replace_all(src, "xml_");
    let re_persname = Regex::new(r"<persName>(.*)</persName>\n").unwrap();
    let src = re_persname.replace_all(&src, r"<persName><name>$1</name></persName>");
    let re_address = Regex::new(r"(<resp>.*</resp>)\n\s*(<address>.*</address>)").unwrap();
    let src = re_address.replace_all(&src, r"$2$1");
    let re_address_persname = Regex::new(r"(</persName>)\s*(<address>.*</address>)").unwrap();
    let src = re_address_persname.replace_all(&src, r"$2$1");
    src.to_owned().to_string()
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Treebank {
    version: String,
    xml_lang: String,
    cts: String,
    header: Header,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Header {
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
pub struct FileDesc {
    #[serde(rename = "editionStmt")]
    edition_stmt: EditionStmt,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EditionStmt {
    #[serde(rename = "$value")]
    resp_stmts: Vec<RespStmt>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RespStmt {
    #[serde(rename = "persName")]
    pers_name: Option<PersInfo>,
    resp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PersInfo {
    name: String,
    short: Option<String>,
    uri: Option<String>,
    address: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Body {
    #[serde(rename = "$value")]
    sentences: Vec<Sentence>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sentence {
    id: u32,
    document_id: String,
    subdoc: String,
    #[serde(rename = "$value")]
    words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Word {
    id: u32,
    form: String,
    lemma: Option<String>,
    postag: Option<String>,
    artificial: Option<String>,
    relation: String,
    head: u32,
}
