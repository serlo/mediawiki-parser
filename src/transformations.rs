//! Common structures and macros for implementing parse tree transformations.

use std::usize;
use ast::*;
use error::*;
use util;

/// Transformation result type
pub type TResult = Result<Element, MWError>;

/// Signature of an in-place transformation function
pub type TFuncInplace = Fn(Element) -> TResult;

/// Signature of a cloning transformation function
pub type TFunc = Fn(&Element, &Vec<&Element>) -> TResult;


/// Apply a given transformation function to a list of elements, without mutating the original.
pub fn apply_func_clone(func: &TFunc, content: &Vec<Element>, path: &Vec<&Element>) -> Result<Vec<Element>, MWError> {
    let mut result = vec![];
    for child in content {
        result.push(func(child, path)?);
    }
    Ok(result)
}

/// Apply a given transformation to every item in a list, consuming this list.
pub fn apply_func_drain(func: &TFuncInplace, content: &mut Vec<Element>) -> Result<Vec<Element>, MWError> {
    let mut result = vec![];
    for child in content.drain(..) {
        result.push(func(child)?);
    }
    Ok(result)
}

/// Recursively apply a transformation function `func` to all children of element `root`.
pub fn recurse_ast_inplace(func: &TFuncInplace, root: Element) -> TResult {
    recurse_inplace_template(func, root, &apply_func_drain)
}

/// Recursively apply  a function `content_func` to the children list of a node.
pub fn recurse_inplace_template(func: &TFuncInplace, mut root: Element,
    content_func: &Fn(&TFuncInplace, &mut Vec<Element>) -> Result<Vec<Element>, MWError>) -> TResult {

    match root {
        Element::Document {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::Heading {ref mut caption, ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            let mut new_caption = content_func(func, caption)?;
            caption.append(&mut new_caption);
            content.append(&mut new_content);
        },
        Element::Formatted {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::Paragraph {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::Template {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::TemplateArgument {ref mut value, ..} => {
            let mut new_value = content_func(func, value)?;
            value.append(&mut new_value);
        },
        Element::InternalReference {ref mut target, ref mut options, ref mut caption, ..} => {
            let mut new_target = content_func(func, target)?;
            let mut new_caption = content_func(func, caption)?;
            let mut new_options = vec![];
            for mut option in options.drain(..) {
                new_options.push(content_func(func, &mut option)?);
            };

            target.append(&mut new_target);
            options.append(&mut new_options);
            caption.append(&mut new_caption);
        },
        Element::ExternalReference {ref mut caption, ..} => {
            let mut new_caption = content_func(func, caption)?;
            caption.append(&mut new_caption);
        },
        Element::ListItem {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::List {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::Table {ref mut caption, ref mut rows, ..} => {
            let mut new_caption = content_func(func, caption)?;
            let mut new_rows = content_func(func, rows)?;
            rows.append(&mut new_rows);
            caption.append(&mut new_caption);
        },
        Element::TableRow {ref mut cells, ..} => {
            let mut new_cells = content_func(func, cells)?;
            cells.append(&mut new_cells);
        },
        Element::TableCell {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        Element::HtmlTag {ref mut content, ..} => {
            let mut new_content = content_func(func, content)?;
            content.append(&mut new_content);
        },
        _ => (),
    };
    Ok(root)
}

/// Recursively apply a transformation function `func` to all children of element `root`, cloning the input.
pub fn recurse_clone(func: &TFunc, root: &Element, path: &Vec<&Element>) -> TResult {

    recurse_clone_template(func, root, path, &apply_func_clone)
}


/// Recursively apply  a function `content_func` to the children list of a node, cloning the input.
pub fn recurse_clone_template(func: &TFunc, root: &Element, path: &Vec<&Element>,
    content_func: &Fn(&TFunc, &Vec<Element>, &Vec<&Element>) -> Result<Vec<Element>, MWError>) -> TResult {

        let mut path = path.clone();
        path.push(root);
        let new = match *root {
            Element::Document {ref position, ref content} => {
                Element::Document {
                    position: position.clone(),
                    content: content_func(func, content, &path)?,
                }
            },

            Element::Heading {ref position, ref depth, ref caption, ref content} => {
                Element::Heading {
                    position: position.clone(),
                    depth: *depth,
                    caption: content_func(func, caption, &path)?,
                    content: content_func(func, content, &path)?,
                }
            },
            Element::Text {..} => {
                root.clone()
            },
            Element::Formatted {ref position, ref markup, ref content} => {
                Element::Formatted {
                    position: position.clone(),
                    markup: markup.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::Paragraph {ref position, ref content} => {
                Element::Paragraph {
                    position: position.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::Template {ref position, ref name, ref content} => {
                Element::Template {
                    position: position.clone(),
                    name: name.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::TemplateArgument {ref position, ref name, ref value} => {
                Element::TemplateArgument {
                    position: position.clone(),
                    name: name.clone(),
                    value: content_func(func, value, &path)?,
                }
            },
            Element::InternalReference {ref position, ref target, ref options, ref caption} => {
                let mut new_options = vec![];
                for option in options {
                    new_options.push(content_func(func, option, &path)?);
                }
                Element::InternalReference {
                    position: position.clone(),
                    target: content_func(func, target, &path)?,
                    options: new_options,
                    caption: content_func(func, caption, &path)?,
                }
            },
            Element::ExternalReference {ref position, ref target, ref caption} => {
                Element::ExternalReference {
                    position: position.clone(),
                    target: target.clone(),
                    caption: content_func(func, caption, &path)?,
                }
            },
            Element::ListItem {ref position, ref depth, ref kind, ref content} => {
                Element::ListItem {
                    position: position.clone(),
                    depth: *depth,
                    kind: kind.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::List {ref position, ref content} => {
                Element::List {
                    position: position.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::Table {ref position, ref attributes, ref caption, ref caption_attributes, ref rows} => {
                Element::Table {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    caption: content_func(func, caption, &path)?,
                    caption_attributes: caption_attributes.clone(),
                    rows: content_func(func, rows, &path)?,
                }
            },
            Element::TableRow {ref position, ref attributes, ref cells} => {
                Element::TableRow {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    cells: content_func(func, cells, &path)?,
                }
            },
            Element::TableCell {ref position, ref header, ref attributes, ref content} => {
                Element::TableCell {
                    position: position.clone(),
                    header: *header,
                    attributes: attributes.clone(),
                    content: content_func(func, content, &path)?,
                }
            },
            Element::Comment {..} => {
                root.clone()
            },
            Element::HtmlTag {ref position, ref name, ref attributes, ref content} => {
                Element::HtmlTag {
                    position: position.clone(),
                    name: name.clone(),
                    attributes: attributes.clone(),
                    content: content_func(func, content, &path)?,
                }
            }
        };
        path.pop();
        Ok(new)
}

/// Moves flat headings into a hierarchical structure based on their depth.
pub fn fold_headings_transformation(mut root: Element) -> TResult {

    // append following deeper headings than current_depth in content to the result list.
    let move_deeper_headings = |root_content: &mut Vec<Element>| -> Result<Vec<Element>, MWError> {

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
                        return Err(MWError::TransformationError(err));
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
            content.append(&mut apply_func_drain(&fold_headings_transformation, &mut new_content)?);
        }
        Element::Heading { ref mut content, .. } => {
            let mut new_content = move_deeper_headings(content)?;
            content.append(&mut apply_func_drain(&fold_headings_transformation, &mut new_content)?);
        }
        _ => (),
    }
    Ok(root)
}

/// Moves list items of higher depth into separate sub-lists.
/// If a list is started with a deeper item than one, this transformation still applies,
/// although this should later be a linter error.
pub fn fold_lists_transformation(mut root: Element) -> TResult {

    // move list items which are deeper than the current level into new sub-lists.
    let move_deeper_items = |root_content: &mut Vec<Element>| -> Result<Vec<Element>, MWError> {

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

                        // this error is returned if the sublist to append to was not found
                        let build_found_error = | origin: &Element | {
                            let err = TransformationError {
                                cause: String::from("sublist was not instantiated properly."),
                                transformation_name: String::from("fold_lists_transformation"),
                                position: origin.get_position().clone(),
                                tree: origin.clone(),
                            };
                            MWError::TransformationError(err)
                        };

                        match result.get_mut(result_len) {
                            Some(&mut Element::ListItem { ref mut content, .. }) => {
                                match content.get_mut(0) {
                                    Some(&mut Element::List { ref mut content, .. }) => {
                                        content.push(new);
                                    }
                                    _ => return Err(build_found_error(&new)),
                                }
                            }
                            _ => return Err(build_found_error(&new)),
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
            content.append(&mut apply_func_drain(&fold_lists_transformation, &mut new_content)?);
        }
        _ => {
            root = recurse_ast_inplace(&fold_lists_transformation, root)?;
        }
    }
    Ok(root)
}

/// Transform whitespace-only paragraphs to empty paragraphs.
pub fn whitespace_paragraphs_to_empty(mut root: Element) -> TResult {
    match root {
        Element::Paragraph {ref mut content, ..} => {
            let mut is_only_whitespace = true;
            for child in &content[..] {
                match child {
                    &Element::Text {ref text, ..} => {
                        if !util::is_whitespace(text) {
                            is_only_whitespace = false;
                            break;
                        }
                    },
                    _ => {
                        is_only_whitespace = false;
                        break;
                    }
                }
            }
            if is_only_whitespace {
                content.drain(..);
            }
        },
        _ => {
            root = recurse_ast_inplace(&whitespace_paragraphs_to_empty, root)?;
        }
    }
    Ok(root)
}

// Reduce consecutive empty paragraphs into one.
//pub fn collapse_empty_paragraphs(mut root: Element) -> Result<Element, MWError> {
//    match root
//}
