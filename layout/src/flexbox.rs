use crate::error::{Error, Result};
use crate::layout::{LayoutBox, LayoutContext, LayoutResult, BoxType, Display, Position};
use crate::style::{Style, ComputedStyle, Length, Percentage, Auto, FlexDirection, FlexWrap, JustifyContent, AlignItems, AlignSelf, AlignContent};
use std::collections::HashMap;

/// Flex container information
#[derive(Debug, Clone)]
pub struct FlexContainer {
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
    /// Container main size
    pub main_size: f32,
    /// Container cross size
    pub cross_size: f32,
    /// Available main space
    pub available_main_space: f32,
    /// Available cross space
    pub available_cross_space: f32,
    /// Flex items
    pub items: Vec<FlexItem>,
    /// Main axis start
    pub main_start: f32,
    /// Cross axis start
    pub cross_start: f32,
    /// Main axis end
    pub main_end: f32,
    /// Cross axis end
    pub cross_end: f32,
}

/// Flex item information
#[derive(Debug, Clone)]
pub struct FlexItem {
    /// Layout box
    pub box_: LayoutBox,
    /// Flex grow
    pub flex_grow: f32,
    /// Flex shrink
    pub flex_shrink: f32,
    /// Flex basis
    pub flex_basis: FlexBasis,
    /// Align self
    pub align_self: AlignSelf,
    /// Main size
    pub main_size: f32,
    /// Cross size
    pub cross_size: f32,
    /// Main margin start
    pub main_margin_start: f32,
    /// Main margin end
    pub main_margin_end: f32,
    /// Cross margin start
    pub cross_margin_start: f32,
    /// Cross margin end
    pub cross_margin_end: f32,
    /// Main position
    pub main_position: f32,
    /// Cross position
    pub cross_position: f32,
    /// Min main size
    pub min_main_size: f32,
    /// Max main size
    pub max_main_size: f32,
    /// Min cross size
    pub min_cross_size: f32,
    /// Max cross size
    pub max_cross_size: f32,
    /// Is frozen
    pub is_frozen: bool,
    /// Violation
    pub violation: f32,
}

/// Flex basis
#[derive(Debug, Clone)]
pub enum FlexBasis {
    /// Auto flex basis
    Auto,
    /// Content flex basis
    Content,
    /// Length flex basis
    Length(Length),
    /// Percentage flex basis
    Percentage(Percentage),
}

/// Flex line
#[derive(Debug, Clone)]
pub struct FlexLine {
    /// Items in this line
    pub items: Vec<FlexItem>,
    /// Line main size
    pub main_size: f32,
    /// Line cross size
    pub cross_size: f32,
    /// Line cross start
    pub cross_start: f32,
    /// Line cross end
    pub cross_end: f32,
}

/// Flex layout algorithm
pub struct FlexLayout {
    /// Container
    container: FlexContainer,
    /// Lines
    lines: Vec<FlexLine>,
    /// Main axis
    main_axis: Axis,
    /// Cross axis
    cross_axis: Axis,
}

/// Axis information
#[derive(Debug, Clone)]
pub struct Axis {
    /// Axis direction
    pub direction: AxisDirection,
    /// Start edge
    pub start: Edge,
    /// End edge
    pub end: Edge,
    /// Size property
    pub size: SizeProperty,
    /// Position property
    pub position: PositionProperty,
    /// Margin start property
    pub margin_start: MarginProperty,
    /// Margin end property
    pub margin_end: MarginProperty,
}

/// Axis direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisDirection {
    /// Horizontal axis
    Horizontal,
    /// Vertical axis
    Vertical,
}

/// Edge
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge {
    /// Top edge
    Top,
    /// Right edge
    Right,
    /// Bottom edge
    Bottom,
    /// Left edge
    Left,
}

/// Size property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeProperty {
    /// Width property
    Width,
    /// Height property
    Height,
}

/// Position property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionProperty {
    /// Top property
    Top,
    /// Right property
    Right,
    /// Bottom property
    Bottom,
    /// Left property
    Left,
}

/// Margin property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarginProperty {
    /// Margin top property
    MarginTop,
    /// Margin right property
    MarginRight,
    /// Margin bottom property
    MarginBottom,
    /// Margin left property
    MarginLeft,
}

impl FlexContainer {
    /// Create new flex container
    pub fn new(style: &ComputedStyle, main_size: f32, cross_size: f32) -> Self {
        Self {
            direction: style.flex_direction,
            wrap: style.flex_wrap,
            justify_content: style.justify_content,
            align_items: style.align_items,
            align_content: style.align_content,
            main_size,
            cross_size,
            available_main_space: main_size,
            available_cross_space: cross_size,
            items: Vec::new(),
            main_start: 0.0,
            cross_start: 0.0,
            main_end: main_size,
            cross_end: cross_size,
        }
    }

    /// Add flex item
    pub fn add_item(&mut self, item: FlexItem) {
        self.items.push(item);
    }

    /// Get main axis
    pub fn get_main_axis(&self) -> Axis {
        match self.direction {
            FlexDirection::Row | FlexDirection::RowReverse => Axis {
                direction: AxisDirection::Horizontal,
                start: if self.direction == FlexDirection::Row { Edge::Left } else { Edge::Right },
                end: if self.direction == FlexDirection::Row { Edge::Right } else { Edge::Left },
                size: SizeProperty::Width,
                position: if self.direction == FlexDirection::Row { PositionProperty::Left } else { PositionProperty::Right },
                margin_start: if self.direction == FlexDirection::Row { MarginProperty::MarginLeft } else { MarginProperty::MarginRight },
                margin_end: if self.direction == FlexDirection::Row { MarginProperty::MarginRight } else { MarginProperty::MarginLeft },
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => Axis {
                direction: AxisDirection::Vertical,
                start: if self.direction == FlexDirection::Column { Edge::Top } else { Edge::Bottom },
                end: if self.direction == FlexDirection::Column { Edge::Bottom } else { Edge::Top },
                size: SizeProperty::Height,
                position: if self.direction == FlexDirection::Column { PositionProperty::Top } else { PositionProperty::Bottom },
                margin_start: if self.direction == FlexDirection::Column { MarginProperty::MarginTop } else { MarginProperty::MarginBottom },
                margin_end: if self.direction == FlexDirection::Column { MarginProperty::MarginBottom } else { MarginProperty::MarginTop },
            },
        }
    }

    /// Get cross axis
    pub fn get_cross_axis(&self) -> Axis {
        match self.direction {
            FlexDirection::Row | FlexDirection::RowReverse => Axis {
                direction: AxisDirection::Vertical,
                start: Edge::Top,
                end: Edge::Bottom,
                size: SizeProperty::Height,
                position: PositionProperty::Top,
                margin_start: MarginProperty::MarginTop,
                margin_end: MarginProperty::MarginBottom,
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => Axis {
                direction: AxisDirection::Horizontal,
                start: Edge::Left,
                end: Edge::Right,
                size: SizeProperty::Width,
                position: PositionProperty::Left,
                margin_start: MarginProperty::MarginLeft,
                margin_end: MarginProperty::MarginRight,
            },
        }
    }
}

impl FlexItem {
    /// Create new flex item
    pub fn new(box_: LayoutBox) -> Self {
        let style = &box_.style;
        Self {
            box_,
            flex_grow: style.flex_grow,
            flex_shrink: style.flex_shrink,
            flex_basis: FlexBasis::Auto,
            align_self: style.align_self,
            main_size: 0.0,
            cross_size: 0.0,
            main_margin_start: 0.0,
            main_margin_end: 0.0,
            cross_margin_start: 0.0,
            cross_margin_end: 0.0,
            main_position: 0.0,
            cross_position: 0.0,
            min_main_size: 0.0,
            max_main_size: f32::INFINITY,
            min_cross_size: 0.0,
            max_cross_size: f32::INFINITY,
            is_frozen: false,
            violation: 0.0,
        }
    }

    /// Get main size with margins
    pub fn get_main_size_with_margins(&self) -> f32 {
        self.main_size + self.main_margin_start + self.main_margin_end
    }

    /// Get cross size with margins
    pub fn get_cross_size_with_margins(&self) -> f32 {
        self.cross_size + self.cross_margin_start + self.cross_margin_end
    }

    /// Set flex basis from style
    pub fn set_flex_basis_from_style(&mut self) {
        let style = &self.box_.style;
        self.flex_basis = match &style.flex_basis {
            crate::style::FlexBasis::Auto => FlexBasis::Auto,
            crate::style::FlexBasis::Content => FlexBasis::Content,
            crate::style::FlexBasis::Length(length) => FlexBasis::Length(length.clone()),
            crate::style::FlexBasis::Percentage(percentage) => FlexBasis::Percentage(percentage.clone()),
        };
    }

    /// Calculate flex base size
    pub fn calculate_flex_base_size(&self, container_main_size: f32) -> f32 {
        match &self.flex_basis {
            FlexBasis::Auto => {
                // Use the main size property if it's not auto
                match self.box_.style.get_main_size_property() {
                    Some(size) => size.resolve(container_main_size),
                    None => 0.0, // Content-based size
                }
            }
            FlexBasis::Content => 0.0, // Content-based size
            FlexBasis::Length(length) => length.resolve(container_main_size),
            FlexBasis::Percentage(percentage) => percentage.resolve(container_main_size),
        }
    }

    /// Calculate hypothetical main size
    pub fn calculate_hypothetical_main_size(&self, container_main_size: f32) -> f32 {
        let flex_base_size = self.calculate_flex_base_size(container_main_size);
        let main_size = self.box_.style.get_main_size_property().map(|s| s.resolve(container_main_size));
        
        match main_size {
            Some(size) => size,
            None => flex_base_size,
        }
    }
}

impl FlexLayout {
    /// Create new flex layout
    pub fn new(container: FlexContainer) -> Self {
        let main_axis = container.get_main_axis();
        let cross_axis = container.get_cross_axis();
        
        Self {
            container,
            lines: Vec::new(),
            main_axis,
            cross_axis,
        }
    }

    /// Layout flex container
    pub fn layout(&mut self, context: &LayoutContext) -> Result<LayoutResult> {
        // Step 1: Determine the main size of each flex item
        self.determine_main_sizes(context)?;
        
        // Step 2: Collect flex items into flex lines
        self.collect_flex_lines()?;
        
        // Step 3: Resolve the flexible lengths
        self.resolve_flexible_lengths()?;
        
        // Step 4: Determine the cross size of each flex item
        self.determine_cross_sizes(context)?;
        
        // Step 5: Determine the cross size of each flex line
        self.determine_line_cross_sizes()?;
        
        // Step 6: Align the flex lines
        self.align_flex_lines()?;
        
        // Step 7: Align the flex items within each line
        self.align_flex_items()?;
        
        // Step 8: Determine the main size of the flex container
        self.determine_container_main_size()?;
        
        // Step 9: Determine the cross size of the flex container
        self.determine_container_cross_size()?;
        
        // Step 10: Position the flex items
        self.position_flex_items()?;
        
        Ok(LayoutResult {
            width: self.container.main_size,
            height: self.container.cross_size,
            children: self.container.items.iter().map(|item| item.box_.clone()).collect(),
        })
    }

    /// Step 1: Determine the main size of each flex item
    fn determine_main_sizes(&mut self, context: &LayoutContext) -> Result<()> {
        for item in &mut self.container.items {
            // Calculate flex base size
            let flex_base_size = item.calculate_flex_base_size(self.container.main_size);
            
            // Calculate hypothetical main size
            let hypothetical_main_size = item.calculate_hypothetical_main_size(self.container.main_size);
            
            // Set main size constraints
            item.min_main_size = item.box_.style.get_min_main_size_property()
                .map(|s| s.resolve(self.container.main_size))
                .unwrap_or(0.0);
            
            item.max_main_size = item.box_.style.get_max_main_size_property()
                .map(|s| s.resolve(self.container.main_size))
                .unwrap_or(f32::INFINITY);
            
            // Set initial main size
            item.main_size = hypothetical_main_size.clamp(item.min_main_size, item.max_main_size);
        }
        
        Ok(())
    }

    /// Step 2: Collect flex items into flex lines
    fn collect_flex_lines(&mut self) -> Result<()> {
        self.lines.clear();
        
        if self.container.wrap == FlexWrap::Nowrap {
            // Single line
            let mut line = FlexLine {
                items: self.container.items.clone(),
                main_size: 0.0,
                cross_size: 0.0,
                cross_start: 0.0,
                cross_end: 0.0,
            };
            
            // Calculate line main size
            line.main_size = line.items.iter().map(|item| item.get_main_size_with_margins()).sum();
            
            self.lines.push(line);
        } else {
            // Multiple lines
            let mut current_line = Vec::new();
            let mut current_line_size = 0.0;
            
            for item in &self.container.items {
                let item_size = item.get_main_size_with_margins();
                
                if !current_line.is_empty() && current_line_size + item_size > self.container.main_size {
                    // Start new line
                    let line = FlexLine {
                        items: current_line,
                        main_size: current_line_size,
                        cross_size: 0.0,
                        cross_start: 0.0,
                        cross_end: 0.0,
                    };
                    self.lines.push(line);
                    
                    current_line = vec![item.clone()];
                    current_line_size = item_size;
                } else {
                    // Add to current line
                    current_line.push(item.clone());
                    current_line_size += item_size;
                }
            }
            
            // Add final line
            if !current_line.is_empty() {
                let line = FlexLine {
                    items: current_line,
                    main_size: current_line_size,
                    cross_size: 0.0,
                    cross_start: 0.0,
                    cross_end: 0.0,
                };
                self.lines.push(line);
            }
        }
        
        Ok(())
    }

    /// Step 3: Resolve the flexible lengths
    fn resolve_flexible_lengths(&mut self) -> Result<()> {
        for line in &mut self.lines {
            self.resolve_flexible_lengths_in_line(line)?;
        }
        
        Ok(())
    }

    /// Resolve flexible lengths in a single line
    fn resolve_flexible_lengths_in_line(&mut self, line: &mut FlexLine) -> Result<()> {
        let available_space = self.container.main_size - line.main_size;
        
        if available_space == 0.0 {
            return Ok(());
        }
        
        // Calculate total flex grow and flex shrink
        let total_flex_grow: f32 = line.items.iter().map(|item| item.flex_grow).sum();
        let total_flex_shrink: f32 = line.items.iter().map(|item| item.flex_shrink).sum();
        
        if available_space > 0.0 && total_flex_grow > 0.0 {
            // Distribute positive free space
            let flex_grow_factor = available_space / total_flex_grow;
            
            for item in &mut line.items {
                if item.flex_grow > 0.0 {
                    let growth = flex_grow_factor * item.flex_grow;
                    item.main_size += growth;
                    item.main_size = item.main_size.clamp(item.min_main_size, item.max_main_size);
                }
            }
        } else if available_space < 0.0 && total_flex_shrink > 0.0 {
            // Distribute negative free space
            let flex_shrink_factor = (-available_space) / total_flex_shrink;
            
            for item in &mut line.items {
                if item.flex_shrink > 0.0 {
                    let shrinkage = flex_shrink_factor * item.flex_shrink;
                    item.main_size -= shrinkage;
                    item.main_size = item.main_size.clamp(item.min_main_size, item.max_main_size);
                }
            }
        }
        
        // Update line main size
        line.main_size = line.items.iter().map(|item| item.get_main_size_with_margins()).sum();
        
        Ok(())
    }

    /// Step 4: Determine the cross size of each flex item
    fn determine_cross_sizes(&mut self, context: &LayoutContext) -> Result<()> {
        for line in &mut self.lines {
            for item in &mut line.items {
                // Calculate cross size based on align-self
                let align_self = if item.align_self == AlignSelf::Auto {
                    self.container.align_items
                } else {
                    item.align_self
                };
                
                match align_self {
                    AlignSelf::Stretch => {
                        // Stretch to fill the line's cross size
                        if let Some(cross_size) = item.box_.style.get_cross_size_property() {
                            item.cross_size = cross_size.resolve(self.container.cross_size);
                        } else {
                            // Will be set in step 5
                            item.cross_size = 0.0;
                        }
                    }
                    AlignSelf::FlexStart | AlignSelf::FlexEnd | AlignSelf::Center | AlignSelf::Baseline => {
                        // Use the item's intrinsic cross size
                        if let Some(cross_size) = item.box_.style.get_cross_size_property() {
                            item.cross_size = cross_size.resolve(self.container.cross_size);
                        } else {
                            // Calculate intrinsic cross size
                            item.cross_size = self.calculate_intrinsic_cross_size(&item.box_, context)?;
                        }
                    }
                }
                
                // Apply min/max cross size constraints
                item.min_cross_size = item.box_.style.get_min_cross_size_property()
                    .map(|s| s.resolve(self.container.cross_size))
                    .unwrap_or(0.0);
                
                item.max_cross_size = item.box_.style.get_max_cross_size_property()
                    .map(|s| s.resolve(self.container.cross_size))
                    .unwrap_or(f32::INFINITY);
                
                item.cross_size = item.cross_size.clamp(item.min_cross_size, item.max_cross_size);
            }
        }
        
        Ok(())
    }

    /// Step 5: Determine the cross size of each flex line
    fn determine_line_cross_sizes(&mut self) -> Result<()> {
        for line in &mut self.lines {
            // Find the maximum cross size of items in this line
            let max_item_cross_size = line.items.iter()
                .map(|item| item.get_cross_size_with_margins())
                .fold(0.0, f32::max);
            
            line.cross_size = max_item_cross_size;
        }
        
        Ok(())
    }

    /// Step 6: Align the flex lines
    fn align_flex_lines(&mut self) -> Result<()> {
        if self.lines.len() <= 1 {
            return Ok(());
        }
        
        let total_cross_size: f32 = self.lines.iter().map(|line| line.cross_size).sum();
        let available_cross_space = self.container.cross_size - total_cross_size;
        
        let cross_start = match self.container.align_content {
            AlignContent::FlexStart => 0.0,
            AlignContent::FlexEnd => available_cross_space,
            AlignContent::Center => available_cross_space / 2.0,
            AlignContent::SpaceBetween => {
                if self.lines.len() > 1 {
                    available_cross_space / (self.lines.len() - 1) as f32
                } else {
                    0.0
                }
            }
            AlignContent::SpaceAround => available_cross_space / self.lines.len() as f32,
            AlignContent::Stretch => 0.0, // Lines will be stretched
        };
        
        let mut current_cross_position = cross_start;
        
        for line in &mut self.lines {
            line.cross_start = current_cross_position;
            line.cross_end = current_cross_position + line.cross_size;
            
            if self.container.align_content == AlignContent::Stretch && available_cross_space > 0.0 {
                // Stretch the line
                let stretch_amount = available_cross_space / self.lines.len() as f32;
                line.cross_size += stretch_amount;
                line.cross_end += stretch_amount;
            }
            
            current_cross_position = line.cross_end;
            
            if self.container.align_content == AlignContent::SpaceBetween && self.lines.len() > 1 {
                current_cross_position += available_cross_space / (self.lines.len() - 1) as f32;
            } else if self.container.align_content == AlignContent::SpaceAround {
                current_cross_position += available_cross_space / self.lines.len() as f32;
            }
        }
        
        Ok(())
    }

    /// Step 7: Align the flex items within each line
    fn align_flex_items(&mut self) -> Result<()> {
        for line in &mut self.lines {
            for item in &mut line.items {
                let align_self = if item.align_self == AlignSelf::Auto {
                    self.container.align_items
                } else {
                    item.align_self
                };
                
                let item_cross_size = item.get_cross_size_with_margins();
                let available_space = line.cross_size - item_cross_size;
                
                item.cross_position = match align_self {
                    AlignSelf::FlexStart => line.cross_start,
                    AlignSelf::FlexEnd => line.cross_start + available_space,
                    AlignSelf::Center => line.cross_start + available_space / 2.0,
                    AlignSelf::Baseline => {
                        // TODO: Implement baseline alignment
                        line.cross_start
                    }
                    AlignSelf::Stretch => {
                        // Item should stretch to fill the line
                        if item.box_.style.get_cross_size_property().is_none() {
                            item.cross_size = line.cross_size - item.cross_margin_start - item.cross_margin_end;
                        }
                        line.cross_start
                    }
                };
            }
        }
        
        Ok(())
    }

    /// Step 8: Determine the main size of the flex container
    fn determine_container_main_size(&mut self) -> Result<()> {
        // The main size is already set in the container
        Ok(())
    }

    /// Step 9: Determine the cross size of the flex container
    fn determine_container_cross_size(&mut self) -> Result<()> {
        if self.lines.is_empty() {
            return Ok(());
        }
        
        let total_cross_size: f32 = self.lines.iter().map(|line| line.cross_size).sum();
        
        match self.container.align_content {
            AlignContent::Stretch => {
                // Container cross size is determined by the available space
                self.container.cross_size = self.container.cross_size.max(total_cross_size);
            }
            _ => {
                // Container cross size is determined by the content
                self.container.cross_size = total_cross_size;
            }
        }
        
        Ok(())
    }

    /// Step 10: Position the flex items
    fn position_flex_items(&mut self) -> Result<()> {
        let mut current_main_position = self.container.main_start;
        
        for line in &mut self.lines {
            // Calculate line main position based on justify-content
            let line_main_start = self.calculate_line_main_start(line, current_main_position);
            let mut item_main_position = line_main_start;
            
            for item in &mut line.items {
                // Set item positions
                item.main_position = item_main_position;
                
                // Update the layout box
                self.update_layout_box_position(item)?;
                
                item_main_position += item.get_main_size_with_margins();
            }
            
            current_main_position += line.main_size;
        }
        
        Ok(())
    }

    /// Calculate line main start position based on justify-content
    fn calculate_line_main_start(&self, line: &FlexLine, current_position: f32) -> f32 {
        let available_space = self.container.main_size - line.main_size;
        
        match self.container.justify_content {
            JustifyContent::FlexStart => current_position,
            JustifyContent::FlexEnd => current_position + available_space,
            JustifyContent::Center => current_position + available_space / 2.0,
            JustifyContent::SpaceBetween => {
                if line.items.len() > 1 {
                    current_position + available_space / (line.items.len() - 1) as f32
                } else {
                    current_position
                }
            }
            JustifyContent::SpaceAround => current_position + available_space / line.items.len() as f32,
            JustifyContent::SpaceEvenly => current_position + available_space / (line.items.len() + 1) as f32,
        }
    }

    /// Update layout box position
    fn update_layout_box_position(&self, item: &mut FlexItem) -> Result<()> {
        let style = &mut item.box_.style;
        
        match self.main_axis.position {
            PositionProperty::Left => style.left = Some(Length::Px(item.main_position)),
            PositionProperty::Right => style.right = Some(Length::Px(item.main_position)),
            PositionProperty::Top => style.top = Some(Length::Px(item.main_position)),
            PositionProperty::Bottom => style.bottom = Some(Length::Px(item.main_position)),
        }
        
        match self.cross_axis.position {
            PositionProperty::Left => style.left = Some(Length::Px(item.cross_position)),
            PositionProperty::Right => style.right = Some(Length::Px(item.cross_position)),
            PositionProperty::Top => style.top = Some(Length::Px(item.cross_position)),
            PositionProperty::Bottom => style.bottom = Some(Length::Px(item.cross_position)),
        }
        
        Ok(())
    }

    /// Calculate intrinsic cross size
    fn calculate_intrinsic_cross_size(&self, box_: &LayoutBox, context: &LayoutContext) -> Result<f32> {
        // TODO: Implement intrinsic cross size calculation
        // This would involve measuring the content's natural size
        Ok(0.0)
    }
}

impl LayoutBox {
    /// Get main size property
    pub fn get_main_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.width.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.height.as_ref(),
        }
    }

    /// Get cross size property
    pub fn get_cross_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.height.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.width.as_ref(),
        }
    }

    /// Get min main size property
    pub fn get_min_main_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.min_width.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.min_height.as_ref(),
        }
    }

    /// Get max main size property
    pub fn get_max_main_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.max_width.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.max_height.as_ref(),
        }
    }

    /// Get min cross size property
    pub fn get_min_cross_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.min_height.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.min_width.as_ref(),
        }
    }

    /// Get max cross size property
    pub fn get_max_cross_size_property(&self) -> Option<&Length> {
        match self.style.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.style.max_height.as_ref(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.style.max_width.as_ref(),
        }
    }
}
