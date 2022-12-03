use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Treebank {
    pub fn from_str(string: &str) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_str::<Treebank>(&preprocess(&string))
    }

    pub fn sentences(&self) -> Vec<Sentence> {
        self.body.sentences.clone()
    }
}

impl Display for Treebank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\nDocument information:\n{}\n",
            self.header, self.body
        )
    }
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

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Text: {}\nRelease date: {}\nAnnotation date: {}\nAnnotation scheme: {}",
            self.file_desc, self.release_date, self.annotation_date, self.annotation_scheme
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileDesc {
    #[serde(rename = "editionStmt")]
    edition_stmt: EditionStmt,
    #[serde(rename = "biblStruct")]
    bibl_struct: BiblStruct,
}

impl Display for FileDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bibl_struct)
    }
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
    name: Option<String>,
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
pub struct BiblStruct {
    monogr: Monogr,
}

impl Display for BiblStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.monogr)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Monogr {
    author: String,
    title: String,
    #[serde(rename = "respStmt")]
    resp_stmts: Vec<RespStmt>,
    // imprint: Imprint,
}

impl Display for Monogr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.author, self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Imprint {
    #[serde(rename = "pubPlace")]
    pub_place: Vec<String>,
    publisher: Vec<String>,
    date: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Body {
    #[serde(rename = "$value")]
    sentences: Vec<Sentence>,
}

impl Body {
    pub fn count_tokens(&self) -> usize {
        let mut c = 0;
        for sent in &self.sentences {
            c += sent.count_tokens();
        }
        c
    }
    pub fn count_words(&self) -> usize {
        let mut c = 0;
        for sent in &self.sentences {
            c += sent.count_words();
        }
        c
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Sentences:\t{}\n  Tokens:\t{}\n  Words:\t{}\n",
            self.sentences.len(),
            self.count_tokens(),
            self.count_words()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Sentence {
    id: u32,
    document_id: String,
    subdoc: String,
    #[serde(rename = "$value")]
    words: Vec<Word>,
}

impl Sentence {
    pub fn words(&self) -> Vec<Word> {
        self.words.clone()
    }

    pub fn count_tokens(&self) -> usize {
        self.words.len()
    }

    pub fn count_words(&self) -> usize {
        let mut c = 0;
        for token in &self.words {
            if token.is_word() {
                c += 1;
            }
        }
        c
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Word {
    id: u32,
    form: String,
    lemma: Option<String>,
    postag: Option<String>,
    artificial: Option<String>,
    relation: String,
    head: u32,
}

impl Word {
    pub fn form(&self) -> &str {
        self.form.as_ref()
    }
    pub fn lemma(&self) -> Option<&str> {
        if let Some(lemma) = self.lemma.as_ref() {
            Some(lemma)
        } else {
            None
        }
    }

    pub fn has_postag(&self) -> bool {
        self.postag.is_some()
    }

    pub fn is_word(&self) -> bool {
        if let Some(pos) = &self.postag {
            !pos.starts_with("u-")
        } else {
            false
        }
    }
}
