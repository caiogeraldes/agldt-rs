use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Preprocesses the source `.xml` code to allow for serialization of the treebank.
///
/// There are some oddities in the scheme used in AGLDT's `xml` header and body, that otherwise make
/// serializing it to a `struct` quite messy.
/// This is kind of a bodge, but should do the trick.
///
/// # Oddities
///
/// The main oddity on AGLDT use of `xml` occurs inside the  tag `<respStmt>`, where the
/// tag `<persName>` might contain either a single string value or a series of tags:
///
/// ```xml
/// <respStmt>
///   <persName>Bridget Almas</persName>
///   <resp>responsible for the annotation environment and cts:urn technology</resp>
///   <address>Tufts University</address>
/// </respStmt>
/// <respStmt>
///   <persName>
///     <short>Vanessa Gorman</short>
///     <name>Vanessa Gorman</name>
///     <address>vbgorman@gmail.com</address>
///     <uri>http://data.perseus.org/sosol/users/Vanessa%20Gorman</uri>
///   </persName>
///   <resp>annotator of the text</resp>
/// </respStmt>
/// ```
/// To solve this oddity, we apply two regex replacements so as to move the
/// `<name>` and `<address>` tags inside `<persName>`.
///
/// A handful of other oddities concearn the use of the tags `<primary>`, `<secondary>` and
/// `<annotator>` inside the tag `<sentence>`. Those are also removed by the regex in the current
/// version.
///
///
/// Finally, the `head` value is sometimes an empty string, which is still an issue for me to
/// serialize. As `0` is not used anywhere else, I replace empty strings for `"0"`.
//
///
pub fn preprocess(src: &str) -> String {
    let re_xmlcolon = Regex::new(r"xml:").unwrap();
    let src = re_xmlcolon.replace_all(src, "xml_");
    let re_persname = Regex::new(r"<persName>(.*)</persName>\n").unwrap();
    let src = re_persname.replace_all(&src, r"<persName><name>$1</name></persName>");
    let re_address = Regex::new(r"(<resp>.*</resp>)\n\s*(<address>.*</address>)").unwrap();
    let src = re_address.replace_all(&src, r"$2$1");
    let re_address_persname = Regex::new(r"(</persName>)\s*(<address>.*</address>)").unwrap();
    let src = re_address_persname.replace_all(&src, r"$2$1");
    let re_primary = Regex::new("<primary>.*</primary>").unwrap();
    let src = re_primary.replace_all(&src, r"");
    let re_secondary = Regex::new("<secondary>.*</secondary>").unwrap();
    let src = re_secondary.replace_all(&src, r"");
    let re_annotator = Regex::new("<annotator>.*</annotator>").unwrap();
    let src = re_annotator.replace_all(&src, r"");
    let re_head = Regex::new("head=\"\"").unwrap();
    let src = re_head.replace_all(&src, "head=\"0\"");

    src.clone().to_string()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Treebank {
    version: String,
    xml_lang: String,
    cts: String,
    header: Header,
    body: Body,
}

impl Treebank {
    pub fn from_xml_str(string: &str) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_str::<Treebank>(&preprocess(string))
    }

    pub fn sentences(&self) -> Vec<Sentence> {
        self.body.sentences.clone()
    }

    pub fn count_tokens(&self) -> usize {
        self.body.count_tokens()
    }

    pub fn count_words(&self) -> usize {
        self.body.count_words()
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditionStmt {
    #[serde(rename = "$value")]
    resp_stmts: Vec<RespStmt>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RespStmt {
    #[serde(rename = "persName")]
    pers_name: Option<PersInfo>,
    resp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersInfo {
    name: String,
    short: Option<String>,
    uri: Option<String>,
    address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BiblStruct {
    monogr: Monogr,
}

impl Display for BiblStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.monogr)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Monogr {
    author: String,
    title: String,
    // #[serde(rename = "respStmt")]
    // resp_stmts: Vec<RespStmt>,
    // imprint: Imprint,
}

impl Display for Monogr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.author, self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Imprint {
    #[serde(rename = "pubPlace")]
    pub_place: Vec<String>,
    publisher: Vec<String>,
    date: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
