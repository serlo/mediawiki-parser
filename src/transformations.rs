//! Functions and types for source tree transformations.

use ast::*;
use error::*;

/// Transformation result type
pub type TResult = Result<Element, TransformationError>;

/// Result type for a list of transformed elements.
pub type TListResult = Result<Vec<Element>, TransformationError>;

/// Signature of an in-place transformation function
pub type TFuncInplace<S> = Fn(Element, S) -> TResult;

/// Signature of a cloning transformation function
pub type TFunc<S> = Fn(&Element, &[&Element], S) -> TResult;

/// Apply a given transformation function to a list of elements, without mutating the original.
pub fn apply_func_clone<S: Copy>(
    func: &TFunc<S>,
    content: &[Element],
    path: &[&Element],
    settings: S,
) -> TListResult {
    let mut result = vec![];
    for child in content {
        result.push(func(child, path, settings)?);
    }
    Ok(result)
}

/// Apply a given transformation to every item in a list, consuming this list.
pub fn apply_func_drain<S: Copy>(
    func: &TFuncInplace<S>,
    content: &mut Vec<Element>,
    settings: S,
) -> TListResult {
    let mut result = vec![];
    for child in content.drain(..) {
        result.push(func(child, settings)?);
    }
    Ok(result)
}

/// Recursively apply a transformation function `func` to all children of element `root`.
pub fn recurse_inplace<S: Copy>(func: &TFuncInplace<S>, root: Element, settings: S) -> TResult {
    recurse_inplace_template(func, root, settings, &apply_func_drain)
}

/// Recursively apply  a function `content_func` to the children list of a node.
pub fn recurse_inplace_template<S: Copy>(
    func: &TFuncInplace<S>,
    mut root: Element,
    settings: S,
    content_func: &Fn(&TFuncInplace<S>, &mut Vec<Element>, S) -> TListResult,
) -> TResult {
    match root {
        Element::Document(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::Formatted(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::Paragraph(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::ListItem(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::List(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::TableCell(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::HtmlTag(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::Gallery(ref mut e) => {
            let mut temp = content_func(func, &mut e.content, settings)?;
            e.content.append(&mut temp);
        }
        Element::Heading(ref mut e) => {
            let mut content = content_func(func, &mut e.content, settings)?;
            let mut caption = content_func(func, &mut e.caption, settings)?;
            e.caption.append(&mut caption);
            e.content.append(&mut content);
        }
        Element::Template(ref mut e) => {
            let mut name = content_func(func, &mut e.name, settings)?;
            let mut content = content_func(func, &mut e.content, settings)?;
            e.name.append(&mut name);
            e.content.append(&mut content);
        }
        Element::TemplateArgument(ref mut e) => {
            let mut value = content_func(func, &mut e.value, settings)?;
            e.value.append(&mut value);
        }
        Element::InternalReference(ref mut e) => {
            let mut target = content_func(func, &mut e.target, settings)?;
            let mut caption = content_func(func, &mut e.caption, settings)?;

            let mut new_options = vec![];
            for mut option in e.options.drain(..) {
                new_options.push(content_func(func, &mut option, settings)?);
            }

            e.target.append(&mut target);
            e.options.append(&mut new_options);
            e.caption.append(&mut caption);
        }
        Element::ExternalReference(ref mut e) => {
            let mut caption = content_func(func, &mut e.caption, settings)?;
            e.caption.append(&mut caption);
        }
        Element::Table(ref mut e) => {
            let mut caption = content_func(func, &mut e.caption, settings)?;
            let mut rows = content_func(func, &mut e.rows, settings)?;
            e.caption.append(&mut caption);
            e.rows.append(&mut rows);
        }
        Element::TableRow(ref mut e) => {
            let mut cells = content_func(func, &mut e.cells, settings)?;
            e.cells.append(&mut cells);
        }
        Element::Text(_) | Element::Comment(_) | Element::Error(_) => (),
    };
    Ok(root)
}

/// Recursively apply a transformation function `func` to all children of element `root`, cloning the input.
pub fn recurse_clone<S: Copy>(
    func: &TFunc<S>,
    root: &Element,
    path: &[&Element],
    settings: S,
) -> TResult {
    recurse_clone_template(func, root, path, settings, &apply_func_clone)
}

/// Recursively apply  a function `content_func` to the children list of a node, cloning the input.
pub fn recurse_clone_template<S: Copy>(
    func: &TFunc<S>,
    root: &Element,
    path: &[&Element],
    settings: S,
    content_func: &Fn(&TFunc<S>, &[Element], &[&Element], S) -> TListResult,
) -> TResult {
    let mut path = path.to_owned();
    path.push(root);
    let new = match *root {
        Element::Document(ref e) => Element::Document(Document {
            position: e.position.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Heading(ref e) => Element::Heading(Heading {
            position: e.position.clone(),
            depth: e.depth,
            caption: content_func(func, &e.caption, &path, settings)?,
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Formatted(ref e) => Element::Formatted(Formatted {
            position: e.position.clone(),
            markup: e.markup,
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Paragraph(ref e) => Element::Paragraph(Paragraph {
            position: e.position.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Template(ref e) => Element::Template(Template {
            position: e.position.clone(),
            name: content_func(func, &e.name, &path, settings)?,
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::TemplateArgument(ref e) => Element::TemplateArgument(TemplateArgument {
            position: e.position.clone(),
            name: e.name.clone(),
            value: content_func(func, &e.value, &path, settings)?,
        }),
        Element::InternalReference(ref e) => {
            let mut new_options = vec![];
            for option in &e.options {
                new_options.push(content_func(func, &option, &path, settings)?);
            }

            Element::InternalReference(InternalReference {
                position: e.position.clone(),
                target: content_func(func, &e.target, &path, settings)?,
                options: new_options,
                caption: content_func(func, &e.caption, &path, settings)?,
            })
        }
        Element::ExternalReference(ref e) => Element::ExternalReference(ExternalReference {
            position: e.position.clone(),
            target: e.target.clone(),
            caption: content_func(func, &e.caption, &path, settings)?,
        }),
        Element::ListItem(ref e) => Element::ListItem(ListItem {
            position: e.position.clone(),
            depth: e.depth,
            kind: e.kind,
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::List(ref e) => Element::List(List {
            position: e.position.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Table(ref e) => Element::Table(Table {
            position: e.position.clone(),
            attributes: e.attributes.clone(),
            caption: content_func(func, &e.caption, &path, settings)?,
            caption_attributes: e.caption_attributes.clone(),
            rows: content_func(func, &e.rows, &path, settings)?,
        }),
        Element::TableRow(ref e) => Element::TableRow(TableRow {
            position: e.position.clone(),
            attributes: e.attributes.clone(),
            cells: content_func(func, &e.cells, &path, settings)?,
        }),
        Element::TableCell(ref e) => Element::TableCell(TableCell {
            position: e.position.clone(),
            header: e.header,
            attributes: e.attributes.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Comment(ref e) => Element::Comment(e.clone()),
        Element::Text(ref e) => Element::Text(e.clone()),
        Element::Error(ref e) => Element::Error(e.clone()),
        Element::HtmlTag(ref e) => Element::HtmlTag(HtmlTag {
            position: e.position.clone(),
            name: e.name.clone(),
            attributes: e.attributes.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
        Element::Gallery(ref e) => Element::Gallery(Gallery {
            position: e.position.clone(),
            attributes: e.attributes.clone(),
            content: content_func(func, &e.content, &path, settings)?,
        }),
    };
    path.pop();
    Ok(new)
}
