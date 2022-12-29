/// Tools for parsing treebanks from AGLDT
///
/// # Stages
///
/// ## Preprocessing
///
/// Preprocesses the source `.xml` code to allow for serialization of the treebank.
///
/// There are some oddities in the scheme used in AGLDT's `xml` header and body, that otherwise make
/// serializing it to a `struct` quite messy.
/// This is kind of a bodge, but should do the trick.
///
/// ### Oddities
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
///
/// ## Serialization
///
/// Uses [`serde`] for serializing the data. I did my best to keep the metadata acessible, but
/// there are still some missing fields that will later be included.
///
pub mod parser;

/// Definitions for parsing and building AGLDT postag data.
pub mod features;

/// Utils
pub mod utils;
