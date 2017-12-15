//! Common structures and macros for implementing parse tree transformations.

use ast::*;
use error::*;

/// Transformation result type
pub type TResult = Result<Element, TransformationError>;

/// Result type for a list of transformed elements.
pub type TListResult = Result<Vec<Element>, TransformationError>;

/// Signature of an in-place transformation function
pub type TFuncInplace<S> = Fn(Element, S) -> TResult;

/// Signature of a cloning transformation function
pub type TFunc<S> = Fn(&Element, &Vec<&Element>, S) -> TResult;


/// Apply a given transformation function to a list of elements, without mutating the original.
pub fn apply_func_clone<S: Copy>(func: &TFunc<S>, content: &Vec<Element>, path: &Vec<&Element>, settings: S) -> TListResult {
    let mut result = vec![];
    for child in content {
        result.push(func(child, path, settings)?);
    }
    Ok(result)
}

/// Apply a given transformation to every item in a list, consuming this list.
pub fn apply_func_drain<S: Copy>(func: &TFuncInplace<S>, content: &mut Vec<Element>, settings: S) -> TListResult {
    let mut result = vec![];
    for child in content.drain(..) {
        result.push(func(child, settings)?);
    }
    Ok(result)
}

/// Recursively apply a transformation function `func` to all children of element `root`.
pub fn recurse_inplace<S: Copy>(func: &TFuncInplace<S>, root: Element, settings: S) -> TResult {
    recurse_inplace_template(func, root, settings,  &apply_func_drain)
}

/// Recursively apply  a function `content_func` to the children list of a node.
pub fn recurse_inplace_template<S: Copy>(func: &TFuncInplace<S>, mut root: Element, settings: S,
    content_func: &Fn(&TFuncInplace<S>, &mut Vec<Element>, S) -> TListResult) -> TResult {

    match root {
        Element::Document {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::Heading {ref mut caption, ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            let mut new_caption = content_func(func, caption, settings)?;
            caption.append(&mut new_caption);
            content.append(&mut new_content);
        },
        Element::Formatted {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::Paragraph {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::Template {ref mut content, ref mut name, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            let mut new_name = content_func(func, name, settings)?;
            content.append(&mut new_content);
            name.append(&mut new_name);
        },
        Element::TemplateArgument {ref mut value, ..} => {
            let mut new_value = content_func(func, value, settings)?;
            value.append(&mut new_value);
        },
        Element::InternalReference {ref mut target, ref mut options, ref mut caption, ..} => {
            let mut new_target = content_func(func, target, settings)?;
            let mut new_caption = content_func(func, caption, settings)?;
            let mut new_options = vec![];
            for mut option in options.drain(..) {
                new_options.push(content_func(func, &mut option, settings)?);
            };

            target.append(&mut new_target);
            options.append(&mut new_options);
            caption.append(&mut new_caption);
        },
        Element::ExternalReference {ref mut caption, ..} => {
            let mut new_caption = content_func(func, caption, settings)?;
            caption.append(&mut new_caption);
        },
        Element::ListItem {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::List {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::Table {ref mut caption, ref mut rows, ..} => {
            let mut new_caption = content_func(func, caption, settings)?;
            let mut new_rows = content_func(func, rows, settings)?;
            rows.append(&mut new_rows);
            caption.append(&mut new_caption);
        },
        Element::TableRow {ref mut cells, ..} => {
            let mut new_cells = content_func(func, cells, settings)?;
            cells.append(&mut new_cells);
        },
        Element::TableCell {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        Element::HtmlTag {ref mut content, ..} => {
            let mut new_content = content_func(func, content, settings)?;
            content.append(&mut new_content);
        },
        _ => (),
    };
    Ok(root)
}

/// Recursively apply a transformation function `func` to all children of element `root`, cloning the input.
pub fn recurse_clone<S: Copy>(func: &TFunc<S>, root: &Element, path: &Vec<&Element>, settings: S) -> TResult {

    recurse_clone_template(func, root, path, settings, &apply_func_clone)
}


/// Recursively apply  a function `content_func` to the children list of a node, cloning the input.
pub fn recurse_clone_template<S: Copy>(func: &TFunc<S>, root: &Element, path: &Vec<&Element>, settings: S,
    content_func: &Fn(&TFunc<S>, &Vec<Element>, &Vec<&Element>, S) -> TListResult) -> TResult {

        let mut path = path.clone();
        path.push(root);
        let new = match *root {
            Element::Document {ref position, ref content} => {
                Element::Document {
                    position: position.clone(),
                    content: content_func(func, content, &path, settings)?,
                }
            },

            Element::Heading {ref position, ref depth, ref caption, ref content} => {
                Element::Heading {
                    position: position.clone(),
                    depth: *depth,
                    caption: content_func(func, caption, &path, settings)?,
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::Text {..} => {
                root.clone()
            },
            Element::Formatted {ref position, ref markup, ref content} => {
                Element::Formatted {
                    position: position.clone(),
                    markup: markup.clone(),
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::Paragraph {ref position, ref content} => {
                Element::Paragraph {
                    position: position.clone(),
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::Template {ref position, ref name, ref content} => {
                Element::Template {
                    position: position.clone(),
                    name: content_func(func, name, &path, settings)?,
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::TemplateArgument {ref position, ref name, ref value} => {
                Element::TemplateArgument {
                    position: position.clone(),
                    name: name.clone(),
                    value: content_func(func, value, &path, settings)?,
                }
            },
            Element::InternalReference {ref position, ref target, ref options, ref caption} => {
                let mut new_options = vec![];
                for option in options {
                    new_options.push(content_func(func, option, &path, settings)?);
                }
                Element::InternalReference {
                    position: position.clone(),
                    target: content_func(func, target, &path, settings)?,
                    options: new_options,
                    caption: content_func(func, caption, &path, settings)?,
                }
            },
            Element::ExternalReference {ref position, ref target, ref caption} => {
                Element::ExternalReference {
                    position: position.clone(),
                    target: target.clone(),
                    caption: content_func(func, caption, &path, settings)?,
                }
            },
            Element::ListItem {ref position, ref depth, ref kind, ref content} => {
                Element::ListItem {
                    position: position.clone(),
                    depth: *depth,
                    kind: kind.clone(),
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::List {ref position, ref content} => {
                Element::List {
                    position: position.clone(),
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::Table {ref position, ref attributes, ref caption, ref caption_attributes, ref rows} => {
                Element::Table {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    caption: content_func(func, caption, &path, settings)?,
                    caption_attributes: caption_attributes.clone(),
                    rows: content_func(func, rows, &path, settings)?,
                }
            },
            Element::TableRow {ref position, ref attributes, ref cells} => {
                Element::TableRow {
                    position: position.clone(),
                    attributes: attributes.clone(),
                    cells: content_func(func, cells, &path, settings)?,
                }
            },
            Element::TableCell {ref position, ref header, ref attributes, ref content} => {
                Element::TableCell {
                    position: position.clone(),
                    header: *header,
                    attributes: attributes.clone(),
                    content: content_func(func, content, &path, settings)?,
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
                    content: content_func(func, content, &path, settings)?,
                }
            },
            Element::Error { .. } => {
                root.clone()
            }
        };
        path.pop();
        Ok(new)
}

