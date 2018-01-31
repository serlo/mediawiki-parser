//! Helper trait for operations reading from the document tree.

use ast::*;
use std::io;


/// Implements a traversion over a tree of `Element`.
///
/// All fields of the traversion struct can be mutated,
/// external settings cannot.
pub trait Traversion<'a, S: Copy + Sized> {
    /// push to the traversion path.
    fn path_push(&mut self, &'a Element);
    /// pop from the traversion path.
    fn path_pop(&mut self) -> Option<&'a Element>;
    /// get the traversion path.
    fn get_path(&self) -> &Vec<&'a Element>;
    /// template method for handling single nodes.
    /// if the result is `false`, handling is complete and
    /// children of this node are not considered,
    /// otherwise `work()` is recursively called for all children.
    fn work(&mut self,
            _root: &'a Element,
            _settings: S,
            _out: &mut io::Write) -> io::Result<bool> {
        Ok(true)
    }

    /// template method for handling a vector of nodes.
    /// if the result is `false`, handling is complete and
    /// children of the vector's elements are not considered,
    /// otherwise `work()` is recursively called for all children.
    fn work_vec(&mut self,
            _root: &'a Vec<Element>,
            _settings: S,
            _out: &mut io::Write) -> io::Result<bool> {
        Ok(true)
    }


    /// run this traversion for a vector of elements.
    fn run_vec(&mut self,
               content: &'a Vec<Element>,
               settings: S,
               out: &mut io::Write) -> io::Result<()> {

        if !self.work_vec(content, settings, out)? {
            return Ok(());
        }
        for elem in &content[..] {
            self.run(elem, settings, out)?;
        }
        Ok(())
    }
    /// run this traversion for an element.
    fn run(&mut self,
           root: &'a Element,
           settings: S,
           out: &mut io::Write) -> io::Result<()> {

        self.path_push(root);

        // break if work function breaks recursion.
        if !self.work(root, settings, out)? {
            return Ok(());
        }
        match root {
            &Element::Document { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::Heading {
                ref caption,
                ref content,
                ..
            } => {
                self.run_vec(caption, settings, out)?;
                self.run_vec(content, settings, out)?;
            }
            &Element::Text { .. } => (),
            &Element::Formatted { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::Paragraph { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::Template { ref content, ref name, .. } => {
                self.run_vec(content, settings, out)?;
                self.run_vec(name, settings, out)?;
            }
            &Element::TemplateArgument { ref value, .. } => {
                self.run_vec(value, settings, out)?;
            }
            &Element::InternalReference {
                ref target,
                ref options,
                ref caption,
                ..
            } => {
                self.run_vec(target, settings, out)?;
                for option in options {
                    self.run_vec(option, settings, out)?;
                }
                self.run_vec(caption, settings, out)?;
            }
            &Element::ExternalReference { ref caption, .. } => {
                self.run_vec(caption, settings, out)?;
            }
            &Element::ListItem { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::List { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::Table {
                ref caption,
                ref rows,
                ..
            } => {
                self.run_vec(caption, settings, out)?;
                self.run_vec(rows, settings, out)?;
            }
            &Element::TableRow { ref cells, .. } => {
                self.run_vec(cells, settings, out)?;
            }
            &Element::TableCell { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            }
            &Element::Comment { .. } => (),
            &Element::HtmlTag { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            },
            &Element::Gallery { ref content, .. } => {
                self.run_vec(content, settings, out)?;
            },
            &Element::Error { .. } => (),
        }
        self.path_pop();
        Ok(())
    }
}
