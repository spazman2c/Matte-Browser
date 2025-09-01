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

pub mod selector_matching;
pub use selector_matching::{SelectorMatcher, FastPathMatcher, AncestorBloomFilter, MatchResult};

pub mod pseudo_classes;
pub use pseudo_classes::{PseudoClassEvaluator, PseudoClassEventHandler, ElementState};

pub mod layout;
pub use layout::{LayoutEngine, LayoutBox, BlockFormattingContext, InlineFormattingContext, LineBox, BoxType, PositionType, Display, Float, Clear, Dimensions, Position};

pub mod flexbox;
pub use flexbox::{FlexboxEngine, FlexContainer, FlexItem, FlexLine, FlexDirection, FlexWrap, JustifyContent, AlignItems, AlignContent, AlignSelf, FlexGrow, FlexShrink, FlexBasis, Order};

pub mod typography;
pub use typography::{FontManager, FontFace, FontFamily, FontWeight, FontStyle, FontStretch, FontMetrics, FontFallback, FontCacheEntry};

pub mod text_shaping;
pub use text_shaping::{TextShaper, ShapedGlyph, ShapedTextRun, CharProperties, CharCategory, BidiClass, TextDirection, LineBreakOpportunity, LineBreakType};

pub mod shadow_dom;
pub use shadow_dom::{ShadowRoot, ShadowRootMode, ShadowDomManager};

pub mod css_property_parser;
pub use css_property_parser::{CssPropertyParser, PropertyValue, LengthUnit, ColorValue};
pub mod css_at_rules;
pub use css_at_rules::{AtRule, KeyframeRule, AtRuleParser, AtRuleManager, AtRuleHandler};
pub mod selector_indexing;
pub use selector_indexing::{SelectorIndex, SelectorIndexEntry, SelectorIndexStats, IndexedSelectorMatcher};
pub use error::{Error, Result};
