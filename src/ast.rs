use std::fmt;
use serde_yaml;

/// Element types used in the abstract syntax tree (AST).
///
/// Each element must keep track of its position in the original
/// input document. After parsing, the document tree can be serialized by serde.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag="type", rename_all="lowercase", deny_unknown_fields)]
pub enum Element {
    Document {position: Position, content: Vec<Element>},
    Heading {position: Position, depth: usize, caption: Box<Element>, content: Vec<Element>},
    Text {position: Position, text: String},
    Formatted {position: Position, markup: MarkupType, content: Vec<Element>},
    Paragraph {position: Position, content: Vec<Element>},
    Template {position: Position, name: Box<Element>, content: Vec<Element>},
    TemplateArgument {position: Position, name: Box<Option<Element>>, value: Vec<Element>},
    InternalReference {position: Position, target: Vec<Element>, options: Vec<Vec<Element>>, caption: Vec<Element>},
    ExternalReference {position: Position, target: String, caption: Vec<Element>},
    ListItem {position: Position, depth: usize, kind: ListItemKind, content: Vec<Element>},
    List {position: Position, content: Vec<Element>},
    Table {position: Position, attributes: String, caption: Vec<Element>, caption_attributes: String, rows: Vec<Element>},
    TableRow {position: Position, attributes: String, cells: Vec<Element>},
    TableCell {position: Position, header: bool, attributes: String, content: Vec<Element>},
}

/// Types of markup a section of text may have.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum MarkupType {
    NoWiki,
    Bold,
    Italic,
    Math,
    Definition,
    StrikeThrough,
    Underline,
    Code,
    Blockquote,
    Preformatted,
}


/// Types of markup a section of text may have.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum ListItemKind {
    Unordered,
    Definition,
    DefinitionTerm,
    Ordered
}

/// Represents the position of a document element in the source document.
///
/// The PartialEq implementation allows for a "any" position (all zero), which is
/// equal to any other position. This is used to reduce clutter in tests, where
/// a default Position ("{}") can be used where the actual representation is irrelevant.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="lowercase", default="Position::any_position")]
pub struct Position {
    offset: usize,
    line: usize,
    col: usize,
}

/// Position of a source line of code.
pub struct SourceLine<'input> {
    start: usize,
    line: &'input str,
    end: usize,
}

/// Match an HTML tag name to it's markup type
pub fn get_markup_by_tag_name(tag: &str) -> MarkupType {
    match &tag.to_lowercase()[..] {
        "math" => MarkupType::Math,
        "del" => MarkupType::StrikeThrough,
        "s" => MarkupType::StrikeThrough,
        "nowiki" => MarkupType::NoWiki,
        "dfn" => MarkupType::Definition,
        "u" => MarkupType::Underline,
        "ins" => MarkupType::Underline,
        "code" => MarkupType::Code,
        "blockquote" => MarkupType::Blockquote,
        "pre" => MarkupType::Preformatted,
        _ => panic!("markup type lookup not implemented for {}!", tag),
    }
}

/// Compiles a list of start and end positions of the input source lines.
///
/// This representation is used to calculate line and column position from the input offset.
pub fn get_source_lines<'input>(source: &'input str) -> Vec<SourceLine> {

    let mut pos = 0;
    let mut result = Vec::new();

    for line in source.split("\n") {
        result.push( SourceLine {
            start: pos,
            line: line,
            end: pos + line.len() + 1,
        });
        pos += line.len() + 1;
    }
    result
}

impl Position {
    pub fn new(offset: usize, slocs: &Vec<SourceLine>) -> Self {
        for (i, sloc) in slocs.iter().enumerate() {
            if offset >= sloc.start && offset < sloc.end {
                return Position {
                    offset: offset,
                    line: i + 1,
                    col: sloc.line[0..offset - sloc.start].chars().count() + 1,
                }
            }
        }
        Position {offset: offset, line: slocs.len() + 1, col: 0}
    }

    pub fn any_position() -> Self {
        Position {
            offset: 0,
            line: 0,
            col: 0,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        // comparing with "any" position is always true
        if (other.offset == 0 && other.line == 0 && other.col == 0) ||
           (self.offset == 0 && self.line == 0 && self.col == 0) {
            return true;
        }

        return self.offset == other.offset && self.line == other.line && self.col == other.col;
    }

    fn ne(&self, other: &Position) -> bool {!self.eq(other)}
}

impl fmt::Display for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self).unwrap())
    }
}
