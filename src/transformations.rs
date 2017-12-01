//! Common structures and macros for implementing parse tree transformations.

use ast::*;
use error::*;


/// Transformation result type
pub type TResult = Result<Element, TransformationError>;

/// Signature of an in-place transformation function
pub type TFuncInplace = Fn(Element) -> TResult;

/// Signature of a cloning transformation function
pub type TFunc = Fn(&Element, &Vec<&Element>) -> TResult;

/// Result type for a list of transformed elements.
pub type TListResult = Result<Vec<Element>, TransformationError>;


/// Apply a given transformation function to a list of elements, without mutating the original.
pub fn apply_func_clone(func: &TFunc, content: &Vec<Element>, path: &Vec<&Element>) -> TListResult {
    let mut result = vec![];
    for child in content {
        result.push(func(child, path)?);
    }
    Ok(result)
}

/// Apply a given transformation to every item in a list, consuming this list.
pub fn apply_func_drain(func: &TFuncInplace, content: &mut Vec<Element>) -> TListResult {
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
    content_func: &Fn(&TFuncInplace, &mut Vec<Element>) -> TListResult) -> TResult {

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
    content_func: &Fn(&TFunc, &Vec<Element>, &Vec<&Element>) -> TListResult) -> TResult {

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

