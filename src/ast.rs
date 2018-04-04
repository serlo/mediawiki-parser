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
    /// The document root.
    Document {
        #[serde(default)]
        position: Span,
        content: Vec<Element>,
    },
    /// Headings make a hierarchical document structure.
    /// Headings of higher depths have other headings as parents.
    Heading {
        #[serde(default)]
        position: Span,
        depth: usize,
        caption: Vec<Element>,
        content: Vec<Element>,
    },
    /// Simple text.
    Text {
        #[serde(default)]
        position: Span,
        text: String
     },
    /// A formatting wrapper, usually around text.
    Formatted {
        #[serde(default)]
        position: Span,
        markup: MarkupType,
        content: Vec<Element>,
    },
    /// Paragraphs are separated by newlines in the input document.
    Paragraph {
        #[serde(default)]
        position: Span,
        content: Vec<Element>,
    },
    /// A mediawiki template.
    Template {
        #[serde(default)]
        position: Span,
        name: Vec<Element>,
        content: Vec<Element>,
    },
    /// Argument of a mediawiki template.
    /// Empty name indicate anonymous arguments.
    TemplateArgument {
        #[serde(default)]
        position: Span,
        name: String,
        value: Vec<Element>,
    },
    /// A reference to internal data, such as embedded files
    /// or other articles.
    InternalReference {
        #[serde(default)]
        position: Span,
        target: Vec<Element>,
        options: Vec<Vec<Element>>,
        caption: Vec<Element>,
    },
    /// External reference, usually hyperlinks.
    ExternalReference {
        #[serde(default)]
        position: Span,
        target: String,
        caption: Vec<Element>,
    },
    /// List item of a certain `ListItemKind`.
    ListItem {
        #[serde(default)]
        position: Span,
        depth: usize,
        kind: ListItemKind,
        content: Vec<Element>,
    },
    /// List of items. The `ListItemKind` of its children
    /// can be heterogenous.
    List {
        #[serde(default)]
        position: Span,
        content: Vec<Element>,
    },
    /// A mediawiki table. `attributes` represent html
    /// attributes assigned to the table.
    Table {
        #[serde(default)]
        position: Span,
        attributes: Vec<TagAttribute>,
        caption: Vec<Element>,
        caption_attributes: Vec<TagAttribute>,
        rows: Vec<Element>,
    },
    /// A table row. `attributes` represent html
    /// attributes assigned to the table.
    TableRow {
        #[serde(default)]
        position: Span,
        attributes: Vec<TagAttribute>,
        cells: Vec<Element>,
    },
    /// A single table cell. `attributes` represent html
    /// attributes assigned to the table. `header` is true
    /// if this cell is marked as a header cell.
    TableCell {
        #[serde(default)]
        position: Span,
        header: bool,
        attributes: Vec<TagAttribute>,
        content: Vec<Element>,
    },
    /// Comments in the input document.
    Comment {
        #[serde(default)]
        position: Span,
        text: String
    },
    /// Html tags not encoding formatting elements.
    HtmlTag {
        #[serde(default)]
        position: Span,
        name: String,
        attributes: Vec<TagAttribute>,
        content: Vec<Element>,
    },
    /// Gallery of images (or interal references in general).
    Gallery {
        #[serde(default)]
        position: Span,
        attributes: Vec<TagAttribute>,
        content: Vec<Element>,
    },
    /// Indicates an erroneous part of the document tree.
    Error {
        #[serde(default)]
        position: Span,
        message: String
    },
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
            Element::Document { ref position, .. }
            | Element::Heading { ref position, .. }
            | Element::Text { ref position, .. }
            | Element::Formatted { ref position, .. }
            | Element::Paragraph { ref position, .. }
            | Element::Template { ref position, .. }
            | Element::TemplateArgument { ref position, .. }
            | Element::InternalReference { ref position, .. }
            | Element::ExternalReference { ref position, .. }
            | Element::List { ref position, .. }
            | Element::ListItem { ref position, .. }
            | Element::Table { ref position, .. }
            | Element::TableRow { ref position, .. }
            | Element::TableCell { ref position, .. }
            | Element::Comment { ref position, .. }
            | Element::HtmlTag { ref position, .. }
            | Element::Gallery { ref position, .. }
            | Element::Error { ref position, .. }
            => position,
        }
    }

    /// returns a mutable reference the source code position of an element.
    pub fn get_position_mut(&mut self) -> &mut Span {
        match *self {
            Element::Document { ref mut position, .. }
            | Element::Heading { ref mut position, .. }
            | Element::Text { ref mut position, .. }
            | Element::Formatted { ref mut position, .. }
            | Element::Paragraph { ref mut position, .. }
            | Element::Template { ref mut position, .. }
            | Element::TemplateArgument { ref mut position, .. }
            | Element::InternalReference { ref mut position, .. }
            | Element::ExternalReference { ref mut position, .. }
            | Element::List { ref mut position, .. }
            | Element::ListItem { ref mut position, .. }
            | Element::Table { ref mut position, .. }
            | Element::TableRow { ref mut position, .. }
            | Element::TableCell { ref mut position, .. }
            | Element::Comment { ref mut position, .. }
            | Element::HtmlTag { ref mut position, .. }
            | Element::Gallery { ref mut position, .. }
            | Element::Error { ref mut position, .. }
            => position,
        }
    }

    /// returns the variant name of an element.
    pub fn get_variant_name(&self) -> &str {
        match *self {
            Element::Document { .. } => "Document",
            Element::Heading { .. } => "Heading",
            Element::Text { .. } => "Text",
            Element::Formatted { .. } => "Formatted",
            Element::Paragraph { .. } => "Paragraph",
            Element::Template { .. } => "Template",
            Element::TemplateArgument { .. } => "TemplateArgument",
            Element::InternalReference { .. } => "InternalReference",
            Element::ExternalReference { .. } => "ExternalReference",
            Element::List { .. } => "List",
            Element::ListItem { .. } => "ListItem",
            Element::Table { .. } => "Table",
            Element::TableRow { .. } => "TableRow",
            Element::TableCell { .. } => "TableCell",
            Element::Comment { .. } => "Comment",
            Element::HtmlTag { .. } => "HtmlTag",
            Element::Gallery { .. } => "Gallery",
            Element::Error { .. } => "Error",
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
