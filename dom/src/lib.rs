//! DOM (Document Object Model) implementation for the Matte browser.
//! 
//! This crate provides HTML parsing and DOM manipulation functionality.

pub mod dom;
pub mod error;
pub mod html_parser;
pub mod events;

// Re-export main types
pub use dom::{Document, Element, Node, TextNode, CommentNode, DocumentTypeNode, DomTraversal};
pub use html_parser::HtmlParser;
pub use events::{Event, EventType, EventListener, EventManager, EventDispatcher, EventTarget, EventPhase};
pub use error::{Error, Result};
