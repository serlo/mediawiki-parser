use std::fmt;
use serde_yaml;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag="type", rename_all="lowercase")]
pub enum Element {
    Document {position: Position, content: Vec<Element>},
    Heading {position: Position, depth: usize, caption: Box<Element>, content: Vec<Element>},
    Text {position: Position, text: String},
    Formatted {position: Position, markup: MarkupType, content: Vec<Element>},
    Paragraph {position: Position, content: Vec<Element>},
    Template {position: Position, content: Vec<Element>},
    TemplateAttribute {position: Position, name: Box<Option<Element>>, value: Vec<Element>},
    Error{message: String},
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum MarkupType {
    Plain,
    Bold,
    Italic,
    Math,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub struct Position {
    start: usize,
    line: usize,
    col: usize,
}

pub struct SourceLine {
    start: usize,
    end: usize,
}

pub fn get_source_lines(source: &str) -> Vec<SourceLine> {

    let mut pos = 0;
    let mut result = Vec::new();

    for line in source.split("\n") {
        result.push( SourceLine {
            start: pos,
            end: pos + line.len() + 1,
        });
        pos += line.len() + 1;
    }
    result
}

impl Position {
    pub fn new(start: usize, slocs: &Vec<SourceLine>) -> Self {

        let mut line = 0;
        let mut col = 0;

        for (i, sloc) in slocs.iter().enumerate() {
            if start >= sloc.start && start <= sloc.end {
                line = i + 1;
                col = start - sloc.start + 1;
                break;
            }
        }

        Position {
            start: start,
            line: line,
            col: col,
        }
    }
}

impl fmt::Display for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self).unwrap())
    }
}
