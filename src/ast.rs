/// Data structures describing the parsed document.

#[cfg(feature = "no_position")]
use serde::ser::{Serialize, Serializer, SerializeMap};

/**
 * Element types used in the abstract syntax tree (AST).
 *
 * Each element must keep track of its position in the original
 * input document. After parsing, the document tree can be serialized by serde.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Element {
    Document(Document),
    Heading(Heading),
    Text(Text),
    Formatted(Formatted),
    Paragraph(Paragraph),
    Template(Template),
    TemplateArgument(TemplateArgument),
    InternalReference(InternalReference),
    ExternalReference(ExternalReference),
    ListItem(ListItem),
    List(List),
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    Comment(Comment),
    HtmlTag(HtmlTag),
    Gallery(Gallery),
    Error(Error),
}

/// The document root.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Document {
    #[serde(default)]
    pub position: Span,
    pub content: Vec<Element>,
}

/// Headings make a hierarchical document structure.
/// Headings of higher depths have other headings as parents.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Heading {
    #[serde(default)]
    pub position: Span,
    pub depth: usize,
    pub caption: Vec<Element>,
    pub content: Vec<Element>,
}

/// Simple text.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Text {
    #[serde(default)]
    pub position: Span,
    pub text: String
}

/// A formatting wrapper, usually around text.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Formatted {
    #[serde(default)]
    pub position: Span,
    pub markup: MarkupType,
    pub content: Vec<Element>,
}

/// Paragraphs are separated by newlines in the input document.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Paragraph {
    #[serde(default)]
    pub position: Span,
    pub content: Vec<Element>,
}

/// A mediawiki template.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Template {
    #[serde(default)]
    pub position: Span,
    pub name: Vec<Element>,
    pub content: Vec<Element>,
}

/// Argument of a mediawiki template.
/// Empty name indicate anonymous arguments.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TemplateArgument {
    #[serde(default)]
    pub position: Span,
    pub name: String,
    pub value: Vec<Element>,
}

/// A reference to internal data, such as embedded files
/// or other articles.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct InternalReference {
    #[serde(default)]
    pub position: Span,
    pub target: Vec<Element>,
    pub options: Vec<Vec<Element>>,
    pub caption: Vec<Element>,
}

/// External reference, usually hyperlinks.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct ExternalReference {
    #[serde(default)]
    pub position: Span,
    pub target: String,
    pub caption: Vec<Element>,
}

/// List item of a certain `ListItemKind`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct ListItem {
    #[serde(default)]
    pub position: Span,
    pub depth: usize,
    pub kind: ListItemKind,
    pub content: Vec<Element>,
}

/// List of items. The `ListItemKind` of its children
/// can be heterogenous.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct List {
    #[serde(default)]
    pub position: Span,
    pub content: Vec<Element>,
}

/// A mediawiki table. `attributes` represent html
/// attributes assigned to the table.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Table {
    #[serde(default)]
    pub position: Span,
    pub attributes: Vec<TagAttribute>,
    pub caption: Vec<Element>,
    pub caption_attributes: Vec<TagAttribute>,
    pub rows: Vec<Element>,
}

/// A table row. `attributes` represent html
/// attributes assigned to the table.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TableRow {
    #[serde(default)]
    pub position: Span,
    pub attributes: Vec<TagAttribute>,
    pub cells: Vec<Element>,
}

/// A single table cell. `attributes` represent html
/// attributes assigned to the table. `header` is true
/// if this cell is marked as a header cell.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TableCell {
    #[serde(default)]
    pub position: Span,
    pub header: bool,
    pub attributes: Vec<TagAttribute>,
    pub content: Vec<Element>,
}

/// Comments in the input document.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Comment {
    #[serde(default)]
    pub position: Span,
    pub text: String
}

/// Html tags not encoding formatting elements.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct HtmlTag {
    #[serde(default)]
    pub position: Span,
    pub name: String,
    pub attributes: Vec<TagAttribute>,
    pub content: Vec<Element>,
}

/// Gallery of images (or interal references in general).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Gallery {
    #[serde(default)]
    pub position: Span,
    pub attributes: Vec<TagAttribute>,
    pub content: Vec<Element>,
}

/// Indicates an erroneous part of the document tree.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct Error {
    #[serde(default)]
    pub position: Span,
    pub message: String
}

/// Types of markup a section of text may have.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MarkupType {
    NoWiki,
    Bold,
    Italic,
    Math,
    StrikeThrough,
    Underline,
    Code,
    Blockquote,
    Preformatted,
}


/// Types of markup a section of text may have.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ListItemKind {
    Unordered,
    Definition,
    DefinitionTerm,
    Ordered,
}

/**
 * Represents a position in the source document.
 *
 * The `PartialEq` implementation allows for a "any" position (all zero), which is
 * equal to any other position. This is used to reduce clutter in tests, where
 * a default Position ("{}") can be used where the actual representation is irrelevant.
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", default = "Position::any_position", deny_unknown_fields)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

/// Holds position information (start and end) for one element
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(not(feature = "no_position"), derive(Serialize))]
#[serde(rename_all = "lowercase", default = "Span::any", deny_unknown_fields)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// Represents a pair of html tag attribute and value.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TagAttribute {
    #[serde(default)]
    pub position: Span,
    pub key: String,
    pub value: String,
}

/// Position of a source line of code.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SourceLine<'input> {
    pub start: usize,
    pub content: &'input str,
    pub end: usize,
}

impl<'input> SourceLine<'input> {
    /// checks if `pos` is at a line start
    pub fn starts_line(pos: usize, slocs: &[SourceLine]) -> bool {
        for sloc in slocs {
            if sloc.start == pos {
                return true;
            }
        }
        false
    }
}

impl MarkupType {
    /// Match an HTML tag name to it's markup type.
    pub fn by_tag_name(tag: &str) -> MarkupType {
        match &tag.to_lowercase()[..] {
            "math" => MarkupType::Math,
            "del" | "s" => MarkupType::StrikeThrough,
            "nowiki" => MarkupType::NoWiki,
            "u" | "ins" => MarkupType::Underline,
            "code" => MarkupType::Code,
            "blockquote" => MarkupType::Blockquote,
            "pre" => MarkupType::Preformatted,
            _ => panic!("markup type lookup not implemented for {}!", tag),
        }
    }
}


impl Element {
    /// returns the source code position of an element.
    pub fn get_position(&self) -> &Span {
        match *self {
            Element::Document(ref e) => &e.position,
            Element::Heading(ref e) => &e.position,
            Element::Text(ref e) => &e.position,
            Element::Formatted(ref e) => &e.position,
            Element::Paragraph(ref e) => &e.position,
            Element::Template(ref e) => &e.position,
            Element::TemplateArgument(ref e) => &e.position,
            Element::InternalReference(ref e) => &e.position,
            Element::ExternalReference(ref e) => &e.position,
            Element::List(ref e) => &e.position,
            Element::ListItem(ref e) => &e.position,
            Element::Table(ref e) => &e.position,
            Element::TableRow(ref e) => &e.position,
            Element::TableCell(ref e) => &e.position,
            Element::Comment(ref e) => &e.position,
            Element::HtmlTag(ref e) => &e.position,
            Element::Gallery(ref e) => &e.position,
            Element::Error(ref e) => &e.position,
        }
    }

    /// returns a mutable reference the source code position of an element.
    pub fn get_position_mut(&mut self) -> &mut Span {
        match *self {
            Element::Document(ref mut e) => &mut e.position,
            Element::Heading(ref mut e) => &mut e.position,
            Element::Text(ref mut e) => &mut e.position,
            Element::Formatted(ref mut e) => &mut e.position,
            Element::Paragraph(ref mut e) => &mut e.position,
            Element::Template(ref mut e) => &mut e.position,
            Element::TemplateArgument(ref mut e) => &mut e.position,
            Element::InternalReference(ref mut e) => &mut e.position,
            Element::ExternalReference(ref mut e) => &mut e.position,
            Element::List(ref mut e) => &mut e.position,
            Element::ListItem(ref mut e) => &mut e.position,
            Element::Table(ref mut e) => &mut e.position,
            Element::TableRow(ref mut e) => &mut e.position,
            Element::TableCell(ref mut e) => &mut e.position,
            Element::Comment(ref mut e) => &mut e.position,
            Element::HtmlTag(ref mut e) => &mut e.position,
            Element::Gallery(ref mut e) => &mut e.position,
            Element::Error(ref mut e) => &mut e.position,
        }
    }

    /// returns the variant name of an element.
    pub fn get_variant_name(&self) -> &str {
        match *self {
            Element::Document(_) => "Document",
            Element::Heading(_) => "Heading",
            Element::Text(_) => "Text",
            Element::Formatted(_) => "Formatted",
            Element::Paragraph(_) => "Paragraph",
            Element::Template(_) => "Template",
            Element::TemplateArgument(_) => "TemplateArgument",
            Element::InternalReference(_) => "InternalReference",
            Element::ExternalReference(_) => "ExternalReference",
            Element::List(_) => "List",
            Element::ListItem(_) => "ListItem",
            Element::Table(_) => "Table",
            Element::TableRow(_) => "TableRow",
            Element::TableCell(_) => "TableCell",
            Element::Comment(_) => "Comment",
            Element::HtmlTag(_) => "HtmlTag",
            Element::Gallery(_) => "Gallery",
            Element::Error(_) => "Error",
        }
    }
}


impl Position {
    pub fn new(offset: usize, slocs: &[SourceLine]) -> Self {
        for (i, sloc) in slocs.iter().enumerate() {
            if offset >= sloc.start && offset < sloc.end {
                return Position {
                    offset,
                    line: i + 1,
                    col: sloc.content[0..offset - sloc.start].chars().count() + 1,
                };
            }
        }
        Position {
            offset,
            line: slocs.len() + 1,
            col: 0,
        }
    }

    pub fn any_position() -> Self {
        Position {
            offset: 0,
            line: 0,
            col: 0,
        }
    }
}


impl Span {
    pub fn any() -> Self {
        Span {
            start: Position::any_position(),
            end: Position::any_position(),
        }
    }

    pub fn new(posl: usize, posr: usize, source_lines: &[SourceLine]) -> Self {
        Span {
            start: Position::new(posl, source_lines),
            end: Position::new(posr, source_lines),
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::any()
    }
}


#[cfg(feature = "no_position")]
impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = serializer.serialize_map(None)?;
        map.end()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        // comparing with "any" position is always true
        if (other.offset == 0 && other.line == 0 && other.col == 0) ||
            (self.offset == 0 && self.line == 0 && self.col == 0)
        {
            return true;
        }

        self.offset == other.offset && self.line == other.line && self.col == other.col
    }
}


impl TagAttribute {
    pub fn new(position: Span, key: String, value: String) -> Self {
        TagAttribute {
            position,
            key,
            value,
        }
    }
}
