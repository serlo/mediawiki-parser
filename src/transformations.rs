//! Common structures and macros for implementing parse tree transformations.

use ast::*;

macro_rules! apply_func {
    ($func:ident, $content:expr, $path:expr) => {{
        let mut result = Vec::new();
        for child in $content {
            result.push($func(child, $path));
        }
        result
    }}
}
macro_rules! recurse_ast {
    ($func:ident, $root:expr, $path:expr) => {{
        $path.push($root.clone());
        let new = match *$root {
            Element::Document {ref position, ref content} => {
                Element::Document {
                    position: position.clone(),
                    content: apply_func!($func, content, $path),
                }
            },

            Element::Heading {ref position, ref depth, ref caption, ref content} => {
                Element::Heading {
                    position: position.clone(),
                    depth: *depth,
                    caption: apply_func!($func, caption, $path),
                    content: apply_func!($func, content, $path),
                }
            },

            _ => $root.clone(),
        };
        $path.pop();
        new
    }}
}

pub fn test_transformation<'a>(root: &'a Element, path: &mut Vec<Element>) -> Element {

    /// append following deeper headings than current_depth in content to the result list.
    fn append_deeper_headings<'a>(current_depth: usize, content: &'a[Element], result: &'a mut Vec<Element>) {
        for child in content.iter() {
            match *child {
                Element::Heading {depth, ..} => {
                    println!("{} {}", current_depth, depth);
                    if depth > current_depth {
                        result.push(child.clone());
                    } else {
                        // quit when equal or more shallow headings where encountered.
                        return
                    }
                },
                _ => (),
            };
        }
    }

    /// Takes a list of contents and moves deeper headings into the content lists of shallow ones.
    fn transform_contents_list(content: &Vec<Element>) -> Vec<Element> {

        let mut result = vec![];
        let mut index = 0;

        while index < content.len() {

            let child = &content[index];
            let remaining = &content[index + 1..];

            match child {
                &Element::Heading {ref position, ref content, ref caption, ref depth} => {

                   // A list of headings deeper than the current one.
                    let mut deeper = vec![];
                    append_deeper_headings(*depth, remaining, &mut deeper);
                    let moved_count = deeper.len();
                    let mut new_content = content.clone();
                    new_content.append(&mut deeper);

                    result.push(Element::Heading {
                        position: position.clone(),
                        content: new_content,
                        caption: caption.clone(),
                        depth: *depth,
                    });

                    index += moved_count + 1;
                },
                _ => {
                    result.push(child.clone());
                    index += 1;
                }
            };
        }
        result
    }

    let new_root: Element = match *root {
        Element::Document {ref position, ref content} => {
            Element::Document {
                position: position.clone(),
                content: transform_contents_list(content),
            }
        },
        Element::Heading {ref position, ref depth, ref caption, ref content} => {
            Element::Heading {
                position: position.clone(),
                depth: *depth,
                content: transform_contents_list(content),
                caption: caption.clone(),
            }
        },
        _ => {
            return recurse_ast!(test_transformation, root, path);
        }
    };

    recurse_ast!(test_transformation, &new_root, path)
}
