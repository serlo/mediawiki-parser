use std::fmt;
use serde_yaml;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag="type", rename_all="lowercase")]
pub enum Element {
    Document {start: usize, content: Vec<Element>},
    Heading {start: usize, depth: usize, caption: Box<Element>, content: Vec<Element>}, 
    Text {start: usize, text: String},
    Formatted {start: usize, content: Vec<Element>, markup: MarkupType},
    Paragraph {start: usize, content: Vec<Element>},
    Template {start: usize, content: Vec<Element>},
    TemplateAttribute {start: usize, name: Box<Option<Element>>, value: Vec<Element>},
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

impl fmt::Display for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", serde_yaml::to_string(&self).unwrap())
    }
}
