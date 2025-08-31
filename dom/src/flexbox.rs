//! Flexbox Layout implementation.
//! 
//! This module provides flexbox layout functionality including
//! flex containers, flex items, sizing, alignment, and wrapping.

use std::collections::HashMap;
use crate::dom::Element;
use crate::layout::LayoutBox;

/// Flex direction
#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection {
    /// Row direction (left to right)
    Row,
    /// Row reverse direction (right to left)
    RowReverse,
    /// Column direction (top to bottom)
    Column,
    /// Column reverse direction (bottom to top)
    ColumnReverse,
}

/// Flex wrap
#[derive(Debug, Clone, PartialEq)]
pub enum FlexWrap {
    /// No wrapping
    NoWrap,
    /// Wrap to next line
    Wrap,
    /// Wrap to previous line
    WrapReverse,
}

/// Justify content alignment
#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
    /// Flex start alignment
    FlexStart,
    /// Flex end alignment
    FlexEnd,
    /// Center alignment
    Center,
    /// Space between alignment
    SpaceBetween,
    /// Space around alignment
    SpaceAround,
    /// Space evenly alignment
    SpaceEvenly,
}

/// Align items alignment
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    /// Stretch alignment (default)
    Stretch,
    /// Flex start alignment
    FlexStart,
    /// Flex end alignment
    FlexEnd,
    /// Center alignment
    Center,
    /// Baseline alignment
    Baseline,
}

/// Align content alignment
#[derive(Debug, Clone, PartialEq)]
pub enum AlignContent {
    /// Stretch alignment (default)
    Stretch,
    /// Flex start alignment
    FlexStart,
    /// Flex end alignment
    FlexEnd,
    /// Center alignment
    Center,
    /// Space between alignment
    SpaceBetween,
    /// Space around alignment
    SpaceAround,
}

/// Align self alignment
#[derive(Debug, Clone, PartialEq)]
pub enum AlignSelf {
    /// Auto alignment (inherit from parent)
    Auto,
    /// Stretch alignment
    Stretch,
    /// Flex start alignment
    FlexStart,
    /// Flex end alignment
    FlexEnd,
    /// Center alignment
    Center,
    /// Baseline alignment
    Baseline,
}

/// Flex grow factor
#[derive(Debug, Clone, PartialEq)]
pub struct FlexGrow(pub f32);

impl Default for FlexGrow {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Flex shrink factor
#[derive(Debug, Clone, PartialEq)]
pub struct FlexShrink(pub f32);

impl Default for FlexShrink {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Flex basis
#[derive(Debug, Clone, PartialEq)]
pub enum FlexBasis {
    /// Auto basis
    Auto,
    /// Content basis
    Content,
    /// Fixed basis
    Fixed(f32),
    /// Percentage basis
    Percentage(f32),
}

impl Default for FlexBasis {
    fn default() -> Self {
        Self::Auto
    }
}

/// Order
#[derive(Debug, Clone, PartialEq)]
pub struct Order(pub i32);

impl Default for Order {
    fn default() -> Self {
        Self(0)
    }
}

/// Flex item properties
#[derive(Debug, Clone, PartialEq)]
pub struct FlexItem {
    /// The layout box for this flex item
    pub box_: LayoutBox,
    /// Flex grow factor
    pub flex_grow: FlexGrow,
    /// Flex shrink factor
    pub flex_shrink: FlexShrink,
    /// Flex basis
    pub flex_basis: FlexBasis,
    /// Order
    pub order: Order,
    /// Align self
    pub align_self: AlignSelf,
    /// Whether this item is a flex item
    pub is_flex_item: bool,
    /// Whether this item is a flex container
    pub is_flex_container: bool,
    /// Whether this item is a flex line
    pub is_flex_line: bool,
    /// Whether this item is a flex wrap
    pub is_flex_wrap: bool,
    /// Whether this item is a flex nowrap
    pub is_flex_nowrap: bool,
    /// Whether this item is a flex wrap reverse
    pub is_flex_wrap_reverse: bool,
    /// Whether this item is a flex direction row
    pub is_flex_direction_row: bool,
    /// Whether this item is a flex direction row reverse
    pub is_flex_direction_row_reverse: bool,
    /// Whether this item is a flex direction column
    pub is_flex_direction_column: bool,
    /// Whether this item is a flex direction column reverse
    pub is_flex_direction_column_reverse: bool,
    /// Whether this item is a justify content flex start
    pub is_justify_content_flex_start: bool,
    /// Whether this item is a justify content flex end
    pub is_justify_content_flex_end: bool,
    /// Whether this item is a justify content center
    pub is_justify_content_center: bool,
    /// Whether this item is a justify content space between
    pub is_justify_content_space_between: bool,
    /// Whether this item is a justify content space around
    pub is_justify_content_space_around: bool,
    /// Whether this item is a justify content space evenly
    pub is_justify_content_space_evenly: bool,
    /// Whether this item is an align items stretch
    pub is_align_items_stretch: bool,
    /// Whether this item is an align items flex start
    pub is_align_items_flex_start: bool,
    /// Whether this item is an align items flex end
    pub is_align_items_flex_end: bool,
    /// Whether this item is an align items center
    pub is_align_items_center: bool,
    /// Whether this item is an align items baseline
    pub is_align_items_baseline: bool,
    /// Whether this item is an align content stretch
    pub is_align_content_stretch: bool,
    /// Whether this item is an align content flex start
    pub is_align_content_flex_start: bool,
    /// Whether this item is an align content flex end
    pub is_align_content_flex_end: bool,
    /// Whether this item is an align content center
    pub is_align_content_center: bool,
    /// Whether this item is an align content space between
    pub is_align_content_space_between: bool,
    /// Whether this item is an align content space around
    pub is_align_content_space_around: bool,
    /// Whether this item is an align self auto
    pub is_align_self_auto: bool,
    /// Whether this item is an align self stretch
    pub is_align_self_stretch: bool,
    /// Whether this item is an align self flex start
    pub is_align_self_flex_start: bool,
    /// Whether this item is an align self flex end
    pub is_align_self_flex_end: bool,
    /// Whether this item is an align self center
    pub is_align_self_center: bool,
    /// Whether this item is an align self baseline
    pub is_align_self_baseline: bool,
}

impl FlexItem {
    /// Create a new flex item
    pub fn new(box_: LayoutBox) -> Self {
        Self {
            box_,
            flex_grow: FlexGrow::default(),
            flex_shrink: FlexShrink::default(),
            flex_basis: FlexBasis::default(),
            order: Order::default(),
            align_self: AlignSelf::Auto,
            is_flex_item: true,
            is_flex_container: false,
            is_flex_line: false,
            is_flex_wrap: false,
            is_flex_nowrap: true,
            is_flex_wrap_reverse: false,
            is_flex_direction_row: true,
            is_flex_direction_row_reverse: false,
            is_flex_direction_column: false,
            is_flex_direction_column_reverse: false,
            is_justify_content_flex_start: true,
            is_justify_content_flex_end: false,
            is_justify_content_center: false,
            is_justify_content_space_between: false,
            is_justify_content_space_around: false,
            is_justify_content_space_evenly: false,
            is_align_items_stretch: true,
            is_align_items_flex_start: false,
            is_align_items_flex_end: false,
            is_align_items_center: false,
            is_align_items_baseline: false,
            is_align_content_stretch: true,
            is_align_content_flex_start: false,
            is_align_content_flex_end: false,
            is_align_content_center: false,
            is_align_content_space_between: false,
            is_align_content_space_around: false,
            is_align_self_auto: true,
            is_align_self_stretch: false,
            is_align_self_flex_start: false,
            is_align_self_flex_end: false,
            is_align_self_center: false,
            is_align_self_baseline: false,
        }
    }
    
    /// Get the flex basis value
    pub fn get_flex_basis_value(&self, container_size: f32) -> f32 {
        match &self.flex_basis {
            FlexBasis::Auto => self.box_.dimensions.content_width,
            FlexBasis::Content => self.box_.dimensions.content_width,
            FlexBasis::Fixed(value) => *value,
            FlexBasis::Percentage(percentage) => container_size * percentage / 100.0,
        }
    }
    
    /// Get the flex shrink value
    pub fn get_flex_shrink_value(&self) -> f32 {
        self.flex_shrink.0
    }
    
    /// Get the flex grow value
    pub fn get_flex_grow_value(&self) -> f32 {
        self.flex_grow.0
    }
    
    /// Get the order value
    pub fn get_order_value(&self) -> i32 {
        self.order.0
    }
}

/// Flex line containing flex items
#[derive(Debug, Clone)]
pub struct FlexLine {
    /// Items in this flex line
    pub items: Vec<FlexItem>,
    /// Line width
    pub width: f32,
    /// Line height
    pub height: f32,
    /// Line cross size
    pub cross_size: f32,
    /// Line main size
    pub main_size: f32,
    /// Line cross start
    pub cross_start: f32,
    /// Line main start
    pub main_start: f32,
}

impl FlexLine {
    /// Create a new flex line
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            width: 0.0,
            height: 0.0,
            cross_size: 0.0,
            main_size: 0.0,
            cross_start: 0.0,
            main_start: 0.0,
        }
    }
    
    /// Add a flex item to this line
    pub fn add_item(&mut self, item: FlexItem) {
        self.items.push(item);
    }
    
    /// Calculate the main size of this line
    pub fn calculate_main_size(&mut self, direction: &FlexDirection) {
        self.main_size = match direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.items.iter().map(|item| item.box_.dimensions.total_width()).sum()
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.items.iter().map(|item| item.box_.dimensions.total_height()).sum()
            }
        };
    }
    
    /// Calculate the cross size of this line
    pub fn calculate_cross_size(&mut self, direction: &FlexDirection) {
        self.cross_size = match direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.items.iter().map(|item| item.box_.dimensions.total_height()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.items.iter().map(|item| item.box_.dimensions.total_width()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
            }
        };
    }
}

/// Flex container properties
#[derive(Debug, Clone)]
pub struct FlexContainer {
    /// The layout box for this flex container
    pub box_: LayoutBox,
    /// Flex direction
    pub direction: FlexDirection,
    /// Flex wrap
    pub wrap: FlexWrap,
    /// Justify content
    pub justify_content: JustifyContent,
    /// Align items
    pub align_items: AlignItems,
    /// Align content
    pub align_content: AlignContent,
    /// Flex lines
    pub lines: Vec<FlexLine>,
    /// Container width
    pub width: f32,
    /// Container height
    pub height: f32,
    /// Available main space
    pub available_main_space: f32,
    /// Available cross space
    pub available_cross_space: f32,
    /// Whether this container is a flex container
    pub is_flex_container: bool,
    /// Whether this container is a flex item
    pub is_flex_item: bool,
    /// Whether this container is a flex line
    pub is_flex_line: bool,
    /// Whether this container is a flex wrap
    pub is_flex_wrap: bool,
    /// Whether this container is a flex nowrap
    pub is_flex_nowrap: bool,
    /// Whether this container is a flex wrap reverse
    pub is_flex_wrap_reverse: bool,
    /// Whether this container is a flex direction row
    pub is_flex_direction_row: bool,
    /// Whether this container is a flex direction row reverse
    pub is_flex_direction_row_reverse: bool,
    /// Whether this container is a flex direction column
    pub is_flex_direction_column: bool,
    /// Whether this container is a flex direction column reverse
    pub is_flex_direction_column_reverse: bool,
    /// Whether this container is a justify content flex start
    pub is_justify_content_flex_start: bool,
    /// Whether this container is a justify content flex end
    pub is_justify_content_flex_end: bool,
    /// Whether this container is a justify content center
    pub is_justify_content_center: bool,
    /// Whether this container is a justify content space between
    pub is_justify_content_space_between: bool,
    /// Whether this container is a justify content space around
    pub is_justify_content_space_around: bool,
    /// Whether this container is a justify content space evenly
    pub is_justify_content_space_evenly: bool,
    /// Whether this container is an align items stretch
    pub is_align_items_stretch: bool,
    /// Whether this container is an align items flex start
    pub is_align_items_flex_start: bool,
    /// Whether this container is an align items flex end
    pub is_align_items_flex_end: bool,
    /// Whether this container is an align items center
    pub is_align_items_center: bool,
    /// Whether this container is an align items baseline
    pub is_align_items_baseline: bool,
    /// Whether this container is an align content stretch
    pub is_align_content_stretch: bool,
    /// Whether this container is an align content flex start
    pub is_align_content_flex_start: bool,
    /// Whether this container is an align content flex end
    pub is_align_content_flex_end: bool,
    /// Whether this container is an align content center
    pub is_align_content_center: bool,
    /// Whether this container is an align content space between
    pub is_align_content_space_between: bool,
    /// Whether this container is an align content space around
    pub is_align_content_space_around: bool,
}

impl FlexContainer {
    /// Create a new flex container
    pub fn new(box_: LayoutBox) -> Self {
        Self {
            box_,
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            lines: Vec::new(),
            width: 0.0,
            height: 0.0,
            available_main_space: 0.0,
            available_cross_space: 0.0,
            is_flex_container: true,
            is_flex_item: false,
            is_flex_line: false,
            is_flex_wrap: false,
            is_flex_nowrap: true,
            is_flex_wrap_reverse: false,
            is_flex_direction_row: true,
            is_flex_direction_row_reverse: false,
            is_flex_direction_column: false,
            is_flex_direction_column_reverse: false,
            is_justify_content_flex_start: true,
            is_justify_content_flex_end: false,
            is_justify_content_center: false,
            is_justify_content_space_between: false,
            is_justify_content_space_around: false,
            is_justify_content_space_evenly: false,
            is_align_items_stretch: true,
            is_align_items_flex_start: false,
            is_align_items_flex_end: false,
            is_align_items_center: false,
            is_align_items_baseline: false,
            is_align_content_stretch: true,
            is_align_content_flex_start: false,
            is_align_content_flex_end: false,
            is_align_content_center: false,
            is_align_content_space_between: false,
            is_align_content_space_around: false,
        }
    }
    
    /// Add a flex line to this container
    pub fn add_line(&mut self, line: FlexLine) {
        self.lines.push(line);
    }
    
    /// Get the main size of the container
    pub fn get_main_size(&self) -> f32 {
        match self.direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.height,
        }
    }
    
    /// Get the cross size of the container
    pub fn get_cross_size(&self) -> f32 {
        match self.direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.width,
        }
    }
    
    /// Calculate available space
    pub fn calculate_available_space(&mut self) {
        self.available_main_space = self.get_main_size();
        self.available_cross_space = self.get_cross_size();
        
        // Subtract item sizes
        for line in &self.lines {
            for item in &line.items {
                match self.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        self.available_main_space -= item.box_.dimensions.total_width();
                    }
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        self.available_main_space -= item.box_.dimensions.total_height();
                    }
                }
            }
        }
    }
}

/// Flexbox layout engine
pub struct FlexboxEngine {
    /// Flex containers by element ID
    containers: HashMap<String, FlexContainer>,
    /// Flex items by element ID
    items: HashMap<String, FlexItem>,
}

impl FlexboxEngine {
    /// Create a new flexbox engine
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            items: HashMap::new(),
        }
    }
    
    /// Add a flex container
    pub fn add_container(&mut self, container: FlexContainer) {
        let id = container.box_.element.id.clone();
        self.containers.insert(id, container);
    }
    
    /// Add a flex item
    pub fn add_item(&mut self, item: FlexItem) {
        let id = item.box_.element.id.clone();
        self.items.insert(id, item);
    }
    
    /// Calculate flexbox layout
    pub fn calculate_layout(&mut self, container: &mut FlexContainer) {
        // Step 1: Collect flex items
        let mut items = self.collect_flex_items(container);
        
        // Step 2: Determine flex lines
        let mut lines = self.determine_flex_lines(container, &mut items);
        
        // Step 3: Calculate main size
        self.calculate_main_size(container, &mut lines);
        
        // Step 4: Calculate cross size
        self.calculate_cross_size(container, &mut lines);
        
        // Step 5: Align items
        self.align_items(container, &mut lines);
        
        // Step 6: Align content
        self.align_content(container, &mut lines);
        
        // Update container lines
        container.lines = lines;
    }
    
    /// Collect flex items from container
    fn collect_flex_items(&self, container: &FlexContainer) -> Vec<FlexItem> {
        let mut items = Vec::new();
        
        for child in &container.box_.children {
            let mut item = FlexItem::new(child.clone());
            
            // Set flex properties based on CSS
            // This is a placeholder implementation
            item.flex_grow = FlexGrow(0.0);
            item.flex_shrink = FlexShrink(1.0);
            item.flex_basis = FlexBasis::Auto;
            item.order = Order(0);
            item.align_self = AlignSelf::Auto;
            
            items.push(item);
        }
        
        // Sort by order
        items.sort_by(|a, b| a.get_order_value().cmp(&b.get_order_value()));
        
        items
    }
    
    /// Determine flex lines based on wrap
    fn determine_flex_lines(&self, container: &FlexContainer, items: &mut Vec<FlexItem>) -> Vec<FlexLine> {
        let mut lines = Vec::new();
        
        match container.wrap {
            FlexWrap::NoWrap => {
                // All items go in a single line
                let mut line = FlexLine::new();
                for item in items {
                    line.add_item(item.clone());
                }
                lines.push(line);
            }
            FlexWrap::Wrap | FlexWrap::WrapReverse => {
                let mut current_line = FlexLine::new();
                let container_main_size = container.get_main_size();
                
                for item in items {
                    let item_main_size = match container.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => item.box_.dimensions.total_width(),
                        FlexDirection::Column | FlexDirection::ColumnReverse => item.box_.dimensions.total_height(),
                    };
                    
                    let current_line_main_size = match container.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => current_line.width,
                        FlexDirection::Column | FlexDirection::ColumnReverse => current_line.height,
                    };
                    
                    // Check if item fits in current line (or if it's the first item)
                    if current_line.items.is_empty() || current_line_main_size + item_main_size <= container_main_size {
                        current_line.add_item(item.clone());
                        
                        // Update line size
                        match container.direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                current_line.width += item_main_size;
                            }
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                current_line.height += item_main_size;
                            }
                        }
                    } else {
                        // Item doesn't fit, start a new line
                        if !current_line.items.is_empty() {
                            lines.push(current_line);
                        }
                        
                        current_line = FlexLine::new();
                        current_line.add_item(item.clone());
                        
                        // Set initial line size
                        match container.direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                current_line.width = item_main_size;
                            }
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                current_line.height = item_main_size;
                            }
                        }
                    }
                }
                
                // Add the last line
                if !current_line.items.is_empty() {
                    lines.push(current_line);
                }
                
                // For wrap-reverse, reverse the order of lines
                if matches!(container.wrap, FlexWrap::WrapReverse) {
                    lines.reverse();
                }
            }
        }
        
        lines
    }
    
    /// Calculate main size for flex items
    fn calculate_main_size(&mut self, container: &FlexContainer, lines: &mut Vec<FlexLine>) {
        for line in lines {
            line.calculate_main_size(&container.direction);
            
            // Calculate flex grow and shrink
            let total_grow: f32 = line.items.iter().map(|item| item.get_flex_grow_value()).sum();
            let total_shrink: f32 = line.items.iter().map(|item| item.get_flex_shrink_value()).sum();
            
            if total_grow > 0.0 {
                // Distribute extra space
                let extra_space = container.get_main_size() - line.main_size;
                let grow_factor = extra_space / total_grow;
                
                for item in &mut line.items {
                    let grow_amount = item.get_flex_grow_value() * grow_factor;
                    match container.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            item.box_.dimensions.content_width += grow_amount;
                        }
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            item.box_.dimensions.content_height += grow_amount;
                        }
                    }
                }
            } else if total_shrink > 0.0 {
                // Distribute negative space
                let negative_space = line.main_size - container.get_main_size();
                let shrink_factor = negative_space / total_shrink;
                
                for item in &mut line.items {
                    let shrink_amount = item.get_flex_shrink_value() * shrink_factor;
                    match container.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            item.box_.dimensions.content_width = (item.box_.dimensions.content_width - shrink_amount).max(0.0);
                        }
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            item.box_.dimensions.content_height = (item.box_.dimensions.content_height - shrink_amount).max(0.0);
                        }
                    }
                }
            }
        }
    }
    
    /// Calculate cross size for flex items
    fn calculate_cross_size(&mut self, container: &FlexContainer, lines: &mut Vec<FlexLine>) {
        for line in lines {
            line.calculate_cross_size(&container.direction);
            
            // Apply align-items to each item
            for item in &mut line.items {
                let align = match item.align_self {
                    AlignSelf::Auto => container.align_items.clone(),
                    AlignSelf::Stretch => AlignItems::Stretch,
                    AlignSelf::FlexStart => AlignItems::FlexStart,
                    AlignSelf::FlexEnd => AlignItems::FlexEnd,
                    AlignSelf::Center => AlignItems::Center,
                    AlignSelf::Baseline => AlignItems::Baseline,
                };
                
                match align {
                    AlignItems::Stretch => {
                        // Stretch to fill container
                        match container.direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                item.box_.dimensions.content_height = line.cross_size;
                            }
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                item.box_.dimensions.content_width = line.cross_size;
                            }
                        }
                    }
                    _ => {
                        // Other alignments don't change size
                    }
                }
            }
        }
    }
    
    /// Align items within lines
    fn align_items(&mut self, container: &FlexContainer, lines: &mut Vec<FlexLine>) {
        for line in lines {
            let mut current_pos = 0.0;
            
            for item in &mut line.items {
                // Position item
                match container.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        item.box_.position_coords.x = current_pos;
                        current_pos += item.box_.dimensions.total_width();
                    }
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        item.box_.position_coords.y = current_pos;
                        current_pos += item.box_.dimensions.total_height();
                    }
                }
            }
        }
    }
    
    /// Align content between lines
    fn align_content(&mut self, container: &FlexContainer, lines: &mut Vec<FlexLine>) {
        if lines.len() <= 1 {
            return;
        }
        
        let total_cross_size: f32 = lines.iter().map(|line| line.cross_size).sum();
        let available_cross_space = container.get_cross_size() - total_cross_size;
        
        let mut current_pos = 0.0;
        
        match container.align_content {
            AlignContent::Stretch => {
                // Distribute space evenly
                let extra_per_line = available_cross_space / lines.len() as f32;
                for line in lines {
                    line.cross_start = current_pos;
                    line.cross_size += extra_per_line;
                    current_pos += line.cross_size;
                }
            }
            AlignContent::FlexStart => {
                // Pack at start
                for line in lines {
                    line.cross_start = current_pos;
                    current_pos += line.cross_size;
                }
            }
            AlignContent::FlexEnd => {
                // Pack at end
                current_pos = available_cross_space;
                for line in lines {
                    line.cross_start = current_pos;
                    current_pos += line.cross_size;
                }
            }
            AlignContent::Center => {
                // Pack at center
                current_pos = available_cross_space / 2.0;
                for line in lines {
                    line.cross_start = current_pos;
                    current_pos += line.cross_size;
                }
            }
            AlignContent::SpaceBetween => {
                // Space between lines
                let space_between = available_cross_space / (lines.len() - 1) as f32;
                for line in lines {
                    line.cross_start = current_pos;
                    current_pos += line.cross_size + space_between;
                }
            }
            AlignContent::SpaceAround => {
                // Space around lines
                let space_around = available_cross_space / lines.len() as f32;
                current_pos = space_around / 2.0;
                for line in lines {
                    line.cross_start = current_pos;
                    current_pos += line.cross_size + space_around;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::Element;

    #[test]
    fn test_flex_item_creation() {
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let item = FlexItem::new(box_);
        
        assert_eq!(item.flex_grow, FlexGrow(0.0));
        assert_eq!(item.flex_shrink, FlexShrink(1.0));
        assert_eq!(item.flex_basis, FlexBasis::Auto);
        assert_eq!(item.order, Order(0));
        assert_eq!(item.align_self, AlignSelf::Auto);
    }

    #[test]
    fn test_flex_container_creation() {
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let container = FlexContainer::new(box_);
        
        assert_eq!(container.direction, FlexDirection::Row);
        assert_eq!(container.wrap, FlexWrap::NoWrap);
        assert_eq!(container.justify_content, JustifyContent::FlexStart);
        assert_eq!(container.align_items, AlignItems::Stretch);
        assert_eq!(container.align_content, AlignContent::Stretch);
    }

    #[test]
    fn test_flex_line_creation() {
        let mut line = FlexLine::new();
        
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let item = FlexItem::new(box_);
        
        line.add_item(item);
        assert_eq!(line.items.len(), 1);
    }

    #[test]
    fn test_flexbox_engine_creation() {
        let engine = FlexboxEngine::new();
        
        assert!(engine.containers.is_empty());
        assert!(engine.items.is_empty());
    }

    #[test]
    fn test_flex_basis_calculation() {
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let mut item = FlexItem::new(box_);
        
        // Test auto basis
        assert_eq!(item.get_flex_basis_value(100.0), 0.0);
        
        // Test fixed basis
        item.flex_basis = FlexBasis::Fixed(50.0);
        assert_eq!(item.get_flex_basis_value(100.0), 50.0);
        
        // Test percentage basis
        item.flex_basis = FlexBasis::Percentage(25.0);
        assert_eq!(item.get_flex_basis_value(100.0), 25.0);
    }

    #[test]
    fn test_container_size_calculation() {
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let mut container = FlexContainer::new(box_);
        
        container.width = 800.0;
        container.height = 600.0;
        
        // Test row direction
        assert_eq!(container.get_main_size(), 800.0);
        assert_eq!(container.get_cross_size(), 600.0);
        
        // Test column direction
        container.direction = FlexDirection::Column;
        assert_eq!(container.get_main_size(), 600.0);
        assert_eq!(container.get_cross_size(), 800.0);
    }

    #[test]
    fn test_flex_wrapping() {
        let mut engine = FlexboxEngine::new();
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let mut container = FlexContainer::new(box_);
        
        // Set up container with wrap enabled
        container.width = 300.0;
        container.height = 200.0;
        container.wrap = FlexWrap::Wrap;
        
        // Create items that will wrap
        let mut items = Vec::new();
        for i in 0..4 {
            let element = Element::new("div".to_string());
            let mut box_ = LayoutBox::new(element);
            box_.dimensions.content_width = 150.0;
            box_.dimensions.content_height = 50.0;
            let item = FlexItem::new(box_);
            items.push(item);
        }
        
        // Test wrapping logic
        let lines = engine.determine_flex_lines(&container, &mut items);
        
        // Should create 2 lines with 2 items each (150px + 150px = 300px container width)
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].items.len(), 2);
        assert_eq!(lines[1].items.len(), 2);
    }

    #[test]
    fn test_flex_nowrap() {
        let mut engine = FlexboxEngine::new();
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let mut container = FlexContainer::new(box_);
        
        // Set up container with no wrap
        container.width = 300.0;
        container.height = 200.0;
        container.wrap = FlexWrap::NoWrap;
        
        // Create items
        let mut items = Vec::new();
        for i in 0..4 {
            let element = Element::new("div".to_string());
            let mut box_ = LayoutBox::new(element);
            box_.dimensions.content_width = 150.0;
            box_.dimensions.content_height = 50.0;
            let item = FlexItem::new(box_);
            items.push(item);
        }
        
        // Test no-wrap logic
        let lines = engine.determine_flex_lines(&container, &mut items);
        
        // Should create 1 line with all 4 items
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].items.len(), 4);
    }

    #[test]
    fn test_flex_wrap_reverse() {
        let mut engine = FlexboxEngine::new();
        let element = Element::new("div".to_string());
        let box_ = LayoutBox::new(element);
        let mut container = FlexContainer::new(box_);
        
        // Set up container with wrap-reverse
        container.width = 300.0;
        container.height = 200.0;
        container.wrap = FlexWrap::WrapReverse;
        
        // Create items that will wrap
        let mut items = Vec::new();
        for i in 0..4 {
            let element = Element::new("div".to_string());
            let mut box_ = LayoutBox::new(element);
            box_.dimensions.content_width = 150.0;
            box_.dimensions.content_height = 50.0;
            let item = FlexItem::new(box_);
            items.push(item);
        }
        
        // Test wrap-reverse logic
        let lines = engine.determine_flex_lines(&container, &mut items);
        
        // Should create 2 lines with 2 items each, but in reverse order
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].items.len(), 2);
        assert_eq!(lines[1].items.len(), 2);
        
        // The lines should be in reverse order compared to normal wrap
        // (This is a basic test - in a real implementation, you'd verify the visual order)
    }
}
