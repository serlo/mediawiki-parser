//! Common structures and macros for implementing parse tree transformations.

use std::usize;
use ast::*;

/// Apply a given transformation function to a list of elements.
macro_rules! apply_func {
    ($func:ident, $content:expr, $path:expr) => {{
        let mut result = vec![];
        for child in $content {
            result.push($func(child, $path));
        }
        result
    }}
}

/// Apply a given transformation to every item in a list, consuming this list.
macro_rules! apply_func_drain {
    ($func:ident, $content:expr) => {{
        let mut result = vec![];
        for child in $content.drain(..) {
            result.push($func(child));
        }
        result
    }}
}

/// Take a root element and recursively apply the transformation function, creating a new document tree.
macro_rules! recurse_ast {
    ($func:ident, $root:expr, $path:expr) => {{
        let mut temp_path = $path.clone();
        temp_path.push($root);
        let new = match *$root {
            Element::Document {ref position, ref content} => {
                Element::Document {
                    position: position.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },

            Element::Heading {ref position, ref depth, ref caption, ref content} => {
                Element::Heading {
                    position: position.clone(),
                    depth: *depth,
                    caption: apply_func!($func, caption, &temp_path),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::Text {..} => {
                $root.clone()
            },
            Element::Formatted {ref position, ref markup, ref content} => {
                Element::Formatted {
                    position: position.clone(),
                    markup: markup.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::Paragraph {ref position, ref content} => {
                Element::Paragraph {
                    position: position.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::Template {ref position, ref name, ref content} => {
                Element::Template {
                    position: position.clone(),
                    name: name.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::TemplateArgument {ref position, ref name, ref value} => {
                Element::TemplateArgument {
                    position: position.clone(),
                    name: name.clone(),
                    value: apply_func!($func, value, &temp_path),
                }
            },
            Element::InternalReference {ref position, ref target, ref options, ref caption} => {
                let transform_option = |option, path| {apply_func!($func, option, path)};
                Element::InternalReference {
                    position: position.clone(),
                    target: apply_func!($func, target, &temp_path),
                    options: apply_func!(transform_option, options, &temp_path),
                    caption: apply_func!($func, caption, &temp_path),
                }
            },
            Element::ExternalReference {ref position, ref target, ref caption} => {
                Element::ExternalReference {
                    position: position.clone(),
                    target: target.clone(),
                    caption: apply_func!($func, caption, &temp_path),
                }
            },
            Element::ListItem {ref position, ref depth, ref kind, ref content} => {
                Element::ListItem {
                    position: position.clone(),
                    depth: *depth,
                    kind: kind.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::List {ref position, ref content} => {
                Element::List {
                    position: position.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::Table {ref position, ref attributes, ref caption, ref caption_attributes, ref rows} => {
                Element::Table {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    caption: apply_func!($func, caption, &temp_path),
                    caption_attributes: caption_attributes.clone(),
                    rows: apply_func!($func, rows, &temp_path),
                }
            },
            Element::TableRow {ref position, ref attributes, ref cells} => {
                Element::TableRow {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    cells: apply_func!($func, cells, &temp_path),
                }
            },
            Element::TableCell {ref position, ref header, ref attributes, ref content} => {
                Element::TableCell {
                    position: position.clone(),
                    header: *header,
                    attributes: attributes.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            },
            Element::Comment {..} => {
                $root.clone()
            },
            Element::HtmlTag {ref position, ref name, ref attributes, ref content} => {
                Element::HtmlTag {
                    position: position.clone(),
                    name: name.clone(),
                    attributes: attributes.clone(),
                    content: apply_func!($func, content, &temp_path),
                }
            }
        };
        temp_path.pop();
        new
    }}
}

/// Moves flat headings into a hierarchical structure based on their depth.
pub fn fold_headings_transformation(mut root: Element) -> Element {

    /// append following deeper headings than current_depth in content to the result list.
    fn move_deeper_headings(root_content: &mut Vec<Element>) -> Vec<Element> {

        let mut result = vec![];
        let mut current_heading_index = 0;

        // current maximum depth level, every deeper heading will be moved
        let mut current_depth = usize::MAX;

        for child in root_content.drain(..) {
            match child {
                Element::Heading {position, depth, caption, content} => {

                    let new = Element::Heading {
                        position: position,
                        depth: depth,
                        caption: caption,
                        content: content,
                    };

                    if depth > current_depth {
                        match result.get_mut(current_heading_index) {
                            Some(&mut Element::Heading {ref mut content, ..}) => {
                                content.push(new);
                            },
                            _ => (),
                        };

                    } else {
                        // pick a new reference heading if the new one is equally deep or more shallow
                        current_heading_index = result.len();
                        current_depth = depth;
                        result.push(new);
                    }
                },
                _ => {
                    result.push(child);
                    if current_depth < usize::MAX {
                        eprintln!("fold_headings: a non-heading element was found after a heading. This should not happen.");
                    }
                }
            };
        }
        result
    }

    match root {
        Element::Document {ref mut content, ..} => {
            let mut new_content = move_deeper_headings(content);
            content.append(&mut apply_func_drain!(fold_headings_transformation, new_content));
        },
        Element::Heading {ref mut content, ..} => {
            let mut new_content = move_deeper_headings(content);
            content.append(&mut apply_func_drain!(fold_headings_transformation, new_content));
        },
        _ => (),
    }
    root
}

