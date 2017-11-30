//! Common structures and macros for implementing parse tree transformations.

use std::usize;
use ast::*;
use error::TransformationError;

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
            result.push($func(child)?);
        }
        Ok(result)
    }}
}

macro_rules! recurse_ast_inplace {
    ($func:ident, $root:expr) => {{
        match $root {
            Element::Document {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::Heading {ref mut caption, ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                let mut new_caption = apply_func_drain!($func, caption)?;
                caption.append(&mut new_caption);
                content.append(&mut new_content);
            },
            Element::Formatted {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::Paragraph {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::Template {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::TemplateArgument {ref mut value, ..} => {
                let mut new_value = apply_func_drain!($func, value)?;
                value.append(&mut new_value);
            },
            Element::InternalReference {ref mut target, ref mut options, ref mut caption, ..} => {
                let transform_option = |mut option: Vec<Element>| {apply_func_drain!($func, option)};
                let mut new_target = apply_func_drain!($func, target)?;
                let mut new_options = apply_func_drain!(transform_option, options)?;
                let mut new_caption = apply_func_drain!($func, caption)?;
                target.append(&mut new_target);
                options.append(&mut new_options);
                caption.append(&mut new_caption);
            },
            Element::ExternalReference {ref mut caption, ..} => {
                let mut new_caption = apply_func_drain!($func, caption)?;
                caption.append(&mut new_caption);
            },
            Element::ListItem {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::List {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::Table {ref mut caption, ref mut rows, ..} => {
                let mut new_caption = apply_func_drain!($func, caption)?;
                let mut new_rows = apply_func_drain!($func, rows)?;
                rows.append(&mut new_rows);
                caption.append(&mut new_caption);
            },
            Element::TableRow {ref mut cells, ..} => {
                let mut new_cells = apply_func_drain!($func, cells)?;
                cells.append(&mut new_cells);
            },
            Element::TableCell {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            Element::HtmlTag {ref mut content, ..} => {
                let mut new_content = apply_func_drain!($func, content)?;
                content.append(&mut new_content);
            },
            _ => (),
        };
        Ok($root)
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
pub fn fold_headings_transformation(mut root: Element) -> Result<Element, TransformationError> {
    // append following deeper headings than current_depth in content to the result list.
    let move_deeper_headings = |root_content: &mut Vec<Element>| -> Result<Vec<Element>, TransformationError> {
        let mut result = vec![];
        let mut current_heading_index = 0;

        // current maximum depth level, every deeper heading will be moved
        let mut current_depth = usize::MAX;

        for child in root_content.drain(..) {
            match child {
                Element::Heading { position, depth, caption, content } => {
                    let new = Element::Heading {
                        position,
                        depth,
                        caption,
                        content,
                    };

                    if depth > current_depth {
                        match result.get_mut(current_heading_index) {
                            Some(&mut Element::Heading { ref mut content, .. }) => {
                                content.push(new);
                            }
                            _ => (),
                        };
                    } else {
                        // pick a new reference heading if the new one is equally deep or more shallow
                        current_heading_index = result.len();
                        current_depth = depth;
                        result.push(new);
                    }
                }
                _ => {
                    if current_depth < usize::MAX {
                        let err = TransformationError {
                            cause: String::from("a non-heading element was found after a heading. This should not happen."),
                            position: child.get_position().clone(),
                            transformation_name: String::from("fold_headings_transformation"),
                            tree: child.clone()
                        };
                        return Err(err);
                    }
                    result.push(child);
                }
            };
        }
        Ok(result)
    };

    match root {
        Element::Document { ref mut content, .. } => {
            let mut new_content = move_deeper_headings(content)?;
            content.append(&mut apply_func_drain!(fold_headings_transformation, new_content)?);
        }
        Element::Heading { ref mut content, .. } => {
            let mut new_content = move_deeper_headings(content)?;
            content.append(&mut apply_func_drain!(fold_headings_transformation, new_content)?);
        }
        _ => (),
    }
    Ok(root)
}

/// Moves list items of higher depth into separate sub-lists.
/// If a list is started with a deeper item than one, this transformation still applies,
/// although this should later be a linter error.
pub fn fold_lists_transformation(mut root: Element) -> Result<Element, TransformationError> {
    // move list items which are deeper than the current level into new sub-lists.
    let move_deeper_items = |root_content: &mut Vec<Element>| -> Result<Vec<Element>, TransformationError> {
        // the currently least deep list item, every deeper list item will be moved to a new sublist
        let mut lowest_depth = usize::MAX;
        for child in &root_content[..] {
            match child {
                &Element::ListItem { depth, .. } => {
                    if depth < lowest_depth {
                        lowest_depth = depth;
                    }
                }
                _ => (),
            }
        }

        let mut result = vec![];
        // create a new sublist when encountering a lower item
        let mut create_sublist = true;

        for child in root_content.drain(..) {
            match child {
                Element::ListItem { position, depth, kind, content } => {
                    // clone the position and item kind to later use it as list position when creating a sublist.
                    let position_copy = position.clone();

                    let new = Element::ListItem {
                        position,
                        depth,
                        kind,
                        content,
                    };
                    if depth > lowest_depth {
                        if create_sublist {
                            // create a new sublist
                            create_sublist = false;
                            result.push(Element::ListItem {
                                position: position_copy.clone(),
                                depth: lowest_depth,
                                kind,
                                content: vec![Element::List {
                                    position: position_copy,
                                    content: vec![],
                                }],
                            });
                        }
                        let result_len = result.len() - 1;
                        match result.get_mut(result_len) {
                            Some(&mut Element::ListItem { ref mut content, .. }) => {
                                match content.get_mut(0) {
                                    Some(&mut Element::List { ref mut content, .. }) => {
                                        content.push(new);
                                    }
                                    _ => eprintln!("fold_lists: incomplete sublist!"),
                                }
                            }
                            _ => (),
                        };
                    } else {
                        result.push(new);
                        create_sublist = true;
                    }
                }
                _ => {
                    result.push(child);
                }
            };
        }
        Ok(result)
    };

    match root {
        Element::List { ref mut content, .. } => {
            let mut new_content = move_deeper_items(content)?;
            content.append(&mut apply_func_drain!(fold_lists_transformation, new_content)?);
        }
        _ => {
            root = recurse_ast_inplace!(fold_lists_transformation, root)?;
        }
    }
    Ok(root)
}

