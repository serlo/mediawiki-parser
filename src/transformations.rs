//! Common structures and macros for implementing parse tree transformations.

use ast::*;

/// Apply a given transformation function to a list of elements.
macro_rules! apply_func {
    ($func:ident, $content:expr, $path:expr) => {{
        let mut result = Vec::new();
        for child in $content {
            result.push($func(child, $path));
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

pub fn test_transformation<'a>(root: &'a Element, path: &Vec<&Element>) -> Element {

    /// append following deeper headings than current_depth in content to the result list.
    fn append_deeper_headings(current_depth: usize, content: &[Element]) -> Vec<Element> {
        let mut result = vec![];
        for child in content.iter() {
            match *child {
                Element::Heading {depth, ..} => {

                    if depth > current_depth {
                        result.push(child.clone());
                    } else {
                        // quit when equal or more shallow headings where encountered.
                        return result;
                    }
                },
                _ => (),
            };
        }
        result
    }

    /// Takes a list of contents and moves deeper headings into the content lists of shallow ones.
    fn transform_contents_list(content: &Vec<Element>) -> Vec<Element> {

        let mut result = vec![];
        let mut index = 0;

        while index < content.len() {

            let child = &content[index];
            let remaining = &content[index + 1..];

            match *child {
                Element::Heading {ref position, ref content, ref caption, ref depth} => {

                    // A list of headings deeper than the current one.
                    let mut deeper = append_deeper_headings(*depth, remaining);
                    index += deeper.len() + 1;

                    let mut new_content = content.clone();
                    new_content.append(&mut deeper);

                    result.push(Element::Heading {
                        position: position.clone(),
                        content: new_content,
                        caption: caption.clone(),
                        depth: *depth,
                    });
                },
                _ => {
                    result.push(child.clone());
                    index += 1;
                }
            };
        }
        result
    }

    // Create a new root element for headings and documents to reflect the new hierarchy.
    match *root {
        Element::Document {ref position, ref content} => {
            Element::Document {
                position: position.clone(),
                content: apply_func!(test_transformation, &transform_contents_list(content), path),
            }
        },
        Element::Heading {ref position, ref depth, ref caption, ref content} => {
            Element::Heading {
                position: position.clone(),
                depth: *depth,
                content: apply_func!(test_transformation, &transform_contents_list(content), path),
                caption: caption.clone(),
            }
        },
        _ => {
            recurse_ast!(test_transformation, root, path)
        }
    }
}
