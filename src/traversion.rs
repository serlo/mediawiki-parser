//! Helper trait for operations reading from the document tree.

use super::ast::Element;
use std::io;

/// Implements a traversion over a tree of `Element`.
///
/// All fields of the traversion struct can be mutated,
/// external settings cannot.
pub trait Traversion<'a, S: Copy + ?Sized> {
    /// push to the traversion path.
    fn path_push(&mut self, elem: &'a Element);
    /// pop from the traversion path.
    fn path_pop(&mut self) -> Option<&'a Element>;
    /// get the traversion path.
    fn get_path(&self) -> &Vec<&'a Element>;
    /// template method for handling single nodes.
    /// if the result is `false`, handling is complete and
    /// children of this node are not considered,
    /// otherwise `work()` is recursively called for all children.
    fn work(&mut self, _root: &'a Element, _settings: S, _out: &mut io::Write) -> io::Result<bool> {
        Ok(true)
    }

    /// template method for handling a vector of nodes.
    /// if the result is `false`, handling is complete and
    /// children of the vector's elements are not considered,
    /// otherwise `work()` is recursively called for all children.
    fn work_vec(
        &mut self,
        _root: &'a [Element],
        _settings: S,
        _out: &mut io::Write,
    ) -> io::Result<bool> {
        Ok(true)
    }

    /// run this traversion for a vector of elements.
    fn run_vec(
        &mut self,
        content: &'a [Element],
        settings: S,
        out: &mut io::Write,
    ) -> io::Result<()> {
        if !self.work_vec(content, settings, out)? {
            return Ok(());
        }
        for elem in &content[..] {
            self.run(elem, settings, out)?;
        }
        Ok(())
    }
    /// run this traversion for an element.
    fn run(&mut self, root: &'a Element, settings: S, out: &mut io::Write) -> io::Result<()> {
        self.path_push(root);

        // break if work function breaks recursion.
        if !self.work(root, settings, out)? {
            return Ok(());
        }
        match *root {
            Element::Document(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::Formatted(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::Paragraph(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::ListItem(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::List(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::TableCell(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::HtmlTag(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::Gallery(ref e) => self.run_vec(&e.content, settings, out)?,
            Element::Heading(ref e) => {
                self.run_vec(&e.caption, settings, out)?;
                self.run_vec(&e.content, settings, out)?;
            }
            Element::Template(ref e) => {
                self.run_vec(&e.name, settings, out)?;
                self.run_vec(&e.content, settings, out)?;
            }
            Element::TemplateArgument(ref e) => self.run_vec(&e.value, settings, out)?,
            Element::InternalReference(ref e) => {
                self.run_vec(&e.target, settings, out)?;
                for option in &e.options {
                    self.run_vec(option, settings, out)?;
                }
                self.run_vec(&e.caption, settings, out)?;
            }
            Element::ExternalReference(ref e) => self.run_vec(&e.caption, settings, out)?,
            Element::Table(ref e) => {
                self.run_vec(&e.caption, settings, out)?;
                self.run_vec(&e.rows, settings, out)?;
            }
            Element::TableRow(ref e) => self.run_vec(&e.cells, settings, out)?,
            Element::Text(_) | Element::Comment(_) | Element::Error(_) => (),
        }
        self.path_pop();
        Ok(())
    }
}
