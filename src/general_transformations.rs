use transformations::*;
use ast::*;
use util;
use std::usize;
use error::*;

/// Settings for general transformations.
pub struct GeneralSettings {
}


/// Moves flat headings into a hierarchical structure based on their depth.
pub fn fold_headings_transformation(mut root: Element, settings: &GeneralSettings) -> TResult {

    // append following deeper headings than current_depth in content to the result list.
    fn move_deeper_headings<'a>(trans: &TFuncInplace<&'a GeneralSettings>, root_content: &mut Vec<Element>, settings: &'a GeneralSettings) -> TListResult {

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
                        return Err(TransformationError {
                            cause: String::from("a non-heading element was found after a heading. This should not happen."),
                            position: child.get_position().clone(),
                            transformation_name: String::from("fold_headings_transformation"),
                            tree: child.clone()
                        });
                    }
                    result.push(child);
                }
            };
        }

        // recurse transformation
        result = apply_func_drain(trans, &mut result, settings)?;
        Ok(result)
    };
    root = recurse_inplace_template(&fold_headings_transformation, root, settings, &move_deeper_headings)?;
    Ok(root)
}

/// Moves list items of higher depth into separate sub-lists.
/// If a list is started with a deeper item than one, this transformation still applies,
/// although this should later be a linter error.
pub fn fold_lists_transformation(mut root: Element, settings: &GeneralSettings) -> TResult {

    // move list items which are deeper than the current level into new sub-lists.
    fn move_deeper_items<'a>(trans: &TFuncInplace<&'a GeneralSettings>, root_content: &mut Vec<Element>, settings: &'a GeneralSettings) -> TListResult {

        // the currently least deep list item, every deeper list item will be moved to a new sublist
        let mut lowest_depth = usize::MAX;
        for child in &root_content[..] {
            match child {
                &Element::ListItem { depth, .. } => {
                    if depth < lowest_depth {
                        lowest_depth = depth;
                    }
                }
                _ => {
                    return Err(TransformationError {
                        cause: String::from("A list should not contain non-listitems."),
                        transformation_name: String::from("fold_lists_transformation"),
                        position: child.get_position().clone(),
                        tree: child.clone(),
                    })
                },
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

                        // this error is returned if the sublist to append to was not found
                        let build_found_error = | origin: &Element | {
                            TransformationError {
                                cause: String::from("sublist was not instantiated properly."),
                                transformation_name: String::from("fold_lists_transformation"),
                                position: origin.get_position().clone(),
                                tree: origin.clone(),
                            }
                        };

                        if create_sublist {
                            // create a new sublist
                            create_sublist = false;

                            if result.is_empty() {
                                result.push(Element::ListItem {
                                    position: position_copy.clone(),
                                    depth: lowest_depth,
                                    kind,
                                    content: vec![],
                                });
                            }
                            match result.last_mut() {
                                Some(&mut Element::ListItem { ref mut content, .. }) => {
                                    content.push(Element::List {
                                        position: position_copy,
                                        content: vec![],
                                    });
                                },
                                _ => return Err(build_found_error(&new)),
                            };
                        }

                        match result.last_mut() {
                            Some(&mut Element::ListItem { ref mut content, .. }) => {
                                match content.last_mut() {
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
        result = apply_func_drain(trans, &mut result, settings)?;
        Ok(result)
    };

    match root {
        Element::List { .. } => {
            root = recurse_inplace_template(&fold_lists_transformation, root, settings, &move_deeper_items)?;
        },
        _ => {
            root = recurse_inplace(&fold_lists_transformation, root, settings)?;
        }
    }
    Ok(root)
}

/// Transform whitespace-only paragraphs to empty paragraphs.
pub fn whitespace_paragraphs_to_empty(mut root: Element, settings: &GeneralSettings) -> TResult {
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
            root = recurse_inplace(&whitespace_paragraphs_to_empty, root, settings)?;
        }
    }
    Ok(root)
}

/// Reduce consecutive paragraphs into one, if not separated by a blank paragraph.
pub fn collapse_paragraphs(mut root: Element, settings: &GeneralSettings) -> Result<Element, TransformationError> {
    fn squash_empty_paragraphs<'a>(trans: &TFuncInplace<&'a GeneralSettings>, root_content: &mut Vec<Element>, settings: &'a GeneralSettings) -> TListResult {
        let mut result = vec![];
        let mut last_empty = false;

        for mut child in root_content.drain(..) {
            match &mut child {
                &mut Element::Paragraph{ ref mut content, ref mut position } => {
                    if content.is_empty() {
                        last_empty = true;
                        continue;
                    }
                    // if the last paragraph was not empty, append to it.
                    if !last_empty {
                        let current_content = content;
                        let current_position = position;
                        match result.last_mut() {
                            Some(&mut Element::Paragraph { ref mut content, ref mut position}) => {
                                content.append(current_content);
                                position.end = current_position.end.clone();
                                continue;
                            },
                            _ => (),
                        }
                    }
                },
                _ => (),
            };
            result.push(child);
            last_empty = false;
        }
        result = apply_func_drain(trans, &mut result, settings)?;
        Ok(result)
    }
    root = recurse_inplace_template(&collapse_paragraphs, root, settings, &squash_empty_paragraphs)?;
    Ok(root)
}


/// Collapse consecutive text tags into one.
pub fn collapse_consecutive_text(mut root: Element, settings: &GeneralSettings) -> Result<Element, TransformationError> {
    fn squash_text<'a>(trans: &TFuncInplace<&'a GeneralSettings>, root_content: &mut Vec<Element>, settings: &'a GeneralSettings) -> TListResult {
        let mut result = vec![];

        for mut child in root_content.drain(..) {
            match &mut child {
                &mut Element::Text { ref mut text, ref mut position } => {

                    let new_text = text;
                    let new_position = position;
                    match result.last_mut() {
                        Some(&mut Element::Text { ref mut text, ref mut position}) => {
                            text.push_str(" ");
                            text.push_str(&new_text);
                            position.end = new_position.end.clone();
                            continue;
                        },
                        _ => (),
                    }
                },
                _ => (),
            };
            result.push(child);
        }
        result = apply_func_drain(trans, &mut result, settings)?;
        Ok(result)
    }
    root = recurse_inplace_template(&collapse_consecutive_text, root, settings, &squash_text)?;
    Ok(root)
}
