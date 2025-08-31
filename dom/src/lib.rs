//! DOM (Document Object Model) implementation for the Matte browser.
//! 
//! This crate provides HTML parsing and DOM manipulation functionality.

pub mod dom;
pub mod error;
pub mod html_parser;
pub mod events;
pub mod mutation_observer;
pub mod traversal;
pub mod css_tokenizer;
pub mod css_selector;
pub mod cssom;

// Re-export main types
pub use dom::{Document, Element, Node, TextNode, CommentNode, DocumentTypeNode, DomTraversal};
pub use html_parser::HtmlParser;
pub use events::{Event, EventType, EventListener, EventManager, EventDispatcher, EventTarget, EventPhase};
pub use mutation_observer::{MutationObserver, MutationObserverInit, MutationRecord, MutationType, MutationObserverManager};
pub use traversal::{NodeIterator, TreeWalker, NodeFilter, NodeFilterFn, BreadthFirstTraversal, DepthFirstTraversal};
pub use css_tokenizer::{CssToken, CssTokenizer};
pub use css_selector::{CssSelectorParser, SelectorList, ComplexSelector, SimpleSelector, Specificity, PseudoClass, PseudoElement, AttributeSelector, Combinator};
pub use cssom::{CssStyleSheet, CssStyleRule, CssDeclaration, CssValue, CssRule, CssRuleType, ComputedValue, CssCascade};
pub use error::{Error, Result};
