use crate::error::Result;
use crate::{Element, LayoutBox, Dimensions, PositionType, Display, BoxType, Float, Clear, Position};
use std::collections::HashMap;

/// Grid layout direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridDirection {
    Row,
    Column,
}

/// Grid template unit
#[derive(Debug, Clone, PartialEq)]
pub enum GridTemplateUnit {
    /// Fixed pixel value
    Px(f32),
    /// Fractional unit (fr)
    Fraction(f32),
    /// Percentage
    Percentage(f32),
    /// Auto size
    Auto,
    /// Min-content
    MinContent,
    /// Max-content
    MaxContent,
    /// Fit-content
    FitContent(f32),
}

/// Grid line definition
#[derive(Debug, Clone, PartialEq)]
pub struct GridLine {
    pub name: Option<String>,
    pub start: GridTemplateUnit,
    pub end: Option<GridTemplateUnit>,
}

/// Grid template definition
#[derive(Debug, Clone, PartialEq)]
pub struct GridTemplate {
    pub lines: Vec<GridLine>,
}

/// Grid area definition
#[derive(Debug, Clone, PartialEq)]
pub struct GridArea {
    pub name: String,
    pub row_start: i32,
    pub row_end: i32,
    pub column_start: i32,
    pub column_end: i32,
}

/// Grid item placement
#[derive(Debug, Clone, PartialEq)]
pub struct GridItemPlacement {
    pub row_start: Option<i32>,
    pub row_end: Option<i32>,
    pub column_start: Option<i32>,
    pub column_end: Option<i32>,
    pub grid_area: Option<String>,
}

/// Grid alignment options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridAlignment {
    Start,
    Center,
    End,
    Stretch,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

/// Grid container properties
#[derive(Debug, Clone, PartialEq)]
pub struct GridContainer {
    pub display: Display,
    pub grid_template_rows: GridTemplate,
    pub grid_template_columns: GridTemplate,
    pub grid_template_areas: Vec<GridArea>,
    pub grid_gap_row: f32,
    pub grid_gap_column: f32,
    pub justify_items: GridAlignment,
    pub align_items: GridAlignment,
    pub justify_content: GridAlignment,
    pub align_content: GridAlignment,
    pub grid_auto_rows: GridTemplateUnit,
    pub grid_auto_columns: GridTemplateUnit,
    pub grid_auto_flow: GridDirection,
}

/// Grid item properties
#[derive(Debug, Clone, PartialEq)]
pub struct GridItem {
    pub element: Element,
    pub placement: GridItemPlacement,
    pub justify_self: GridAlignment,
    pub align_self: GridAlignment,
    pub grid_row: Option<(i32, i32)>,
    pub grid_column: Option<(i32, i32)>,
}

/// Grid layout engine
#[derive(Debug)]
pub struct GridLayoutEngine {
    container: GridContainer,
    items: Vec<GridItem>,
    grid_lines: HashMap<String, usize>,
}

impl GridContainer {
    /// Create a new grid container with default values
    pub fn new() -> Self {
        Self {
            display: Display::Grid,
            grid_template_rows: GridTemplate { lines: vec![] },
            grid_template_columns: GridTemplate { lines: vec![] },
            grid_template_areas: vec![],
            grid_gap_row: 0.0,
            grid_gap_column: 0.0,
            justify_items: GridAlignment::Stretch,
            align_items: GridAlignment::Stretch,
            justify_content: GridAlignment::Start,
            align_content: GridAlignment::Start,
            grid_auto_rows: GridTemplateUnit::Auto,
            grid_auto_columns: GridTemplateUnit::Auto,
            grid_auto_flow: GridDirection::Row,
        }
    }

    /// Set grid template rows
    pub fn with_template_rows(mut self, template: GridTemplate) -> Self {
        self.grid_template_rows = template;
        self
    }

    /// Set grid template columns
    pub fn with_template_columns(mut self, template: GridTemplate) -> Self {
        self.grid_template_columns = template;
        self
    }

    /// Set grid gap
    pub fn with_gap(mut self, row: f32, column: f32) -> Self {
        self.grid_gap_row = row;
        self.grid_gap_column = column;
        self
    }

    /// Set grid alignment
    pub fn with_alignment(
        mut self,
        justify_items: GridAlignment,
        align_items: GridAlignment,
    ) -> Self {
        self.justify_items = justify_items;
        self.align_items = align_items;
        self
    }
}

impl GridItem {
    /// Create a new grid item
    pub fn new(element: Element) -> Self {
        Self {
            element,
            placement: GridItemPlacement {
                row_start: None,
                row_end: None,
                column_start: None,
                column_end: None,
                grid_area: None,
            },
            justify_self: GridAlignment::Stretch,
            align_self: GridAlignment::Stretch,
            grid_row: None,
            grid_column: None,
        }
    }

    /// Set grid area placement
    pub fn with_grid_area(mut self, area: String) -> Self {
        self.placement.grid_area = Some(area);
        self
    }

    /// Set explicit grid placement
    pub fn with_placement(
        mut self,
        row_start: Option<i32>,
        row_end: Option<i32>,
        column_start: Option<i32>,
        column_end: Option<i32>,
    ) -> Self {
        self.placement.row_start = row_start;
        self.placement.row_end = row_end;
        self.placement.column_start = column_start;
        self.placement.column_end = column_end;
        self
    }

    /// Set self alignment
    pub fn with_self_alignment(
        mut self,
        justify_self: GridAlignment,
        align_self: GridAlignment,
    ) -> Self {
        self.justify_self = justify_self;
        self.align_self = align_self;
        self
    }
}

impl GridLayoutEngine {
    /// Create a new grid layout engine
    pub fn new(container: GridContainer) -> Self {
        Self {
            container,
            items: vec![],
            grid_lines: HashMap::new(),
        }
    }

    /// Add a grid item to the layout
    pub fn add_item(&mut self, item: GridItem) {
        self.items.push(item);
    }

    /// Calculate grid layout
    pub fn calculate_layout(&mut self, container_width: f32, container_height: f32) -> Result<Vec<LayoutBox>> {
        // Build grid lines
        self.build_grid_lines()?;
        
        // Calculate grid tracks
        let (row_tracks, column_tracks) = self.calculate_tracks(container_width, container_height)?;
        
        // Place grid items
        let placed_items = self.place_items(&row_tracks, &column_tracks)?;
        
        // Generate layout boxes
        let layout_boxes = self.generate_layout_boxes(placed_items, &row_tracks, &column_tracks)?;
        
        Ok(layout_boxes)
    }

    /// Build grid lines from templates
    fn build_grid_lines(&mut self) -> Result<()> {
        self.grid_lines.clear();
        
        // Add named lines from row template
        for (i, line) in self.container.grid_template_rows.lines.iter().enumerate() {
            if let Some(ref name) = line.name {
                self.grid_lines.insert(name.clone(), i);
            }
        }
        
        // Add named lines from column template
        for (i, line) in self.container.grid_template_columns.lines.iter().enumerate() {
            if let Some(ref name) = line.name {
                self.grid_lines.insert(name.clone(), i + self.container.grid_template_rows.lines.len());
            }
        }
        
        Ok(())
    }

    /// Calculate grid tracks
    fn calculate_tracks(&self, container_width: f32, container_height: f32) -> Result<(Vec<f32>, Vec<f32>)> {
        let row_tracks = self.calculate_track_sizes(
            &self.container.grid_template_rows,
            container_height,
            self.container.grid_gap_row,
        )?;
        
        let column_tracks = self.calculate_track_sizes(
            &self.container.grid_template_columns,
            container_width,
            self.container.grid_gap_column,
        )?;
        
        Ok((row_tracks, column_tracks))
    }

    /// Calculate track sizes for a template
    fn calculate_track_sizes(
        &self,
        template: &GridTemplate,
        container_size: f32,
        gap: f32,
    ) -> Result<Vec<f32>> {
        let mut track_sizes = vec![];
        let mut fixed_size = 0.0;
        let mut fractional_units = 0.0;
        let mut auto_tracks = 0;
        
        // First pass: calculate fixed sizes and count fractional units
        for line in &template.lines {
            match &line.start {
                GridTemplateUnit::Px(size) => {
                    track_sizes.push(*size);
                    fixed_size += *size;
                }
                GridTemplateUnit::Percentage(percent) => {
                    let size = container_size * percent / 100.0;
                    track_sizes.push(size);
                    fixed_size += size;
                }
                GridTemplateUnit::Fraction(fr) => {
                    track_sizes.push(0.0); // Placeholder
                    fractional_units += fr;
                }
                GridTemplateUnit::Auto => {
                    track_sizes.push(0.0); // Placeholder
                    auto_tracks += 1;
                }
                _ => {
                    track_sizes.push(0.0); // Placeholder for other units
                }
            }
        }
        
        // Calculate remaining space for fractional units
        let total_gaps = (track_sizes.len().saturating_sub(1)) as f32 * gap;
        let remaining_space = container_size - fixed_size - total_gaps;
        
        // Distribute remaining space to fractional units
        if fractional_units > 0.0 {
            let unit_size = remaining_space / fractional_units;
            for (i, line) in template.lines.iter().enumerate() {
                if let GridTemplateUnit::Fraction(fr) = line.start {
                    track_sizes[i] = unit_size * fr;
                }
            }
        }
        
        // Handle auto tracks
        if auto_tracks > 0 {
            let auto_size = if fractional_units == 0.0 {
                remaining_space / auto_tracks as f32
            } else {
                0.0 // Auto tracks get minimum content size
            };
            
            for (i, line) in template.lines.iter().enumerate() {
                if matches!(line.start, GridTemplateUnit::Auto) {
                    track_sizes[i] = auto_size;
                }
            }
        }
        
        Ok(track_sizes)
    }

    /// Place grid items
    fn place_items(
        &self,
        row_tracks: &[f32],
        column_tracks: &[f32],
    ) -> Result<Vec<(GridItem, (i32, i32, i32, i32))>> {
        let mut placed_items = vec![];
        
        for item in &self.items {
            let placement = self.resolve_item_placement(item, row_tracks.len(), column_tracks.len())?;
            placed_items.push((item.clone(), placement));
        }
        
        Ok(placed_items)
    }

    /// Resolve item placement
    fn resolve_item_placement(
        &self,
        item: &GridItem,
        num_rows: usize,
        num_columns: usize,
    ) -> Result<(i32, i32, i32, i32)> {
        // Check for grid area placement first
        if let Some(ref area_name) = item.placement.grid_area {
            if let Some(area) = self.find_grid_area(area_name) {
                return Ok((area.row_start, area.row_end, area.column_start, area.column_end));
            }
        }
        
        // Use explicit placement
        let row_start = item.placement.row_start.unwrap_or(1);
        let row_end = item.placement.row_end.unwrap_or(row_start + 1);
        let column_start = item.placement.column_start.unwrap_or(1);
        let column_end = item.placement.column_end.unwrap_or(column_start + 1);
        
        Ok((row_start, row_end, column_start, column_end))
    }

    /// Find grid area by name
    fn find_grid_area(&self, name: &str) -> Option<&GridArea> {
        self.container.grid_template_areas.iter().find(|area| area.name == name)
    }

    /// Generate layout boxes for placed items
    fn generate_layout_boxes(
        &self,
        placed_items: Vec<(GridItem, (i32, i32, i32, i32))>,
        row_tracks: &[f32],
        column_tracks: &[f32],
    ) -> Result<Vec<LayoutBox>> {
        let mut layout_boxes = vec![];
        
        for (item, (row_start, row_end, col_start, col_end)) in placed_items {
            let x = self.calculate_position(col_start, col_end, column_tracks, self.container.grid_gap_column);
            let y = self.calculate_position(row_start, row_end, row_tracks, self.container.grid_gap_row);
            let width = self.calculate_size(col_start, col_end, column_tracks, self.container.grid_gap_column);
            let height = self.calculate_size(row_start, row_end, row_tracks, self.container.grid_gap_row);
            
            let mut layout_box = LayoutBox::new(item.element);
            layout_box.box_type = BoxType::Grid;
            layout_box.display = Display::Grid;
            layout_box.dimensions.content_width = width;
            layout_box.dimensions.content_height = height;
            layout_box.position_coords.x = x;
            layout_box.position_coords.y = y;
            layout_box.establishes_formatting_context = true;
            
            layout_boxes.push(layout_box);
        }
        
        Ok(layout_boxes)
    }

    /// Calculate position for a grid item
    fn calculate_position(&self, start: i32, end: i32, tracks: &[f32], gap: f32) -> f32 {
        let start_idx = (start - 1).max(0) as usize;
        let end_idx = (end - 1).min((tracks.len() - 1) as i32) as usize;
        
        let mut position = 0.0;
        for i in 0..start_idx {
            position += tracks[i] + gap;
        }
        position
    }

    /// Calculate size for a grid item
    fn calculate_size(&self, start: i32, end: i32, tracks: &[f32], gap: f32) -> f32 {
        let start_idx = (start - 1).max(0) as usize;
        let end_idx = (end - 1).min((tracks.len() - 1) as i32) as usize;
        
        let mut size = 0.0;
        for i in start_idx..end_idx {
            size += tracks[i];
        }
        
        // Add gaps between tracks
        if end_idx > start_idx {
            size += (end_idx - start_idx - 1) as f32 * gap;
        }
        
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::Element;

    #[test]
    fn test_grid_container_creation() {
        let container = GridContainer::new();
        assert_eq!(container.display, Display::Grid);
        assert_eq!(container.grid_gap_row, 0.0);
        assert_eq!(container.grid_gap_column, 0.0);
        assert_eq!(container.justify_items, GridAlignment::Stretch);
        assert_eq!(container.align_items, GridAlignment::Stretch);
    }

    #[test]
    fn test_grid_item_creation() {
        let element = Element::new("div".to_string());
        let item = GridItem::new(element);
        
        assert!(item.placement.grid_area.is_none());
        assert!(item.placement.row_start.is_none());
        assert!(item.placement.column_start.is_none());
        assert_eq!(item.justify_self, GridAlignment::Stretch);
        assert_eq!(item.align_self, GridAlignment::Stretch);
    }

    #[test]
    fn test_grid_template_unit_creation() {
        let px_unit = GridTemplateUnit::Px(100.0);
        let fr_unit = GridTemplateUnit::Fraction(1.0);
        let auto_unit = GridTemplateUnit::Auto;
        
        assert_eq!(px_unit, GridTemplateUnit::Px(100.0));
        assert_eq!(fr_unit, GridTemplateUnit::Fraction(1.0));
        assert_eq!(auto_unit, GridTemplateUnit::Auto);
    }

    #[test]
    fn test_grid_layout_engine_creation() {
        let container = GridContainer::new();
        let engine = GridLayoutEngine::new(container);
        
        assert_eq!(engine.items.len(), 0);
        assert_eq!(engine.grid_lines.len(), 0);
    }

    #[test]
    fn test_simple_grid_layout() {
        let mut container = GridContainer::new();
        
        // Create a 2x2 grid
        let row_template = GridTemplate {
            lines: vec![
                GridLine { name: None, start: GridTemplateUnit::Px(100.0), end: None },
                GridLine { name: None, start: GridTemplateUnit::Px(100.0), end: None },
            ],
        };
        
        let column_template = GridTemplate {
            lines: vec![
                GridLine { name: None, start: GridTemplateUnit::Px(150.0), end: None },
                GridLine { name: None, start: GridTemplateUnit::Px(150.0), end: None },
            ],
        };
        
        container = container
            .with_template_rows(row_template)
            .with_template_columns(column_template);
        
        let mut engine = GridLayoutEngine::new(container);
        
        // Add grid items
        let element1 = Element::new("div".to_string());
        let item1 = GridItem::new(element1).with_placement(Some(1), Some(2), Some(1), Some(3));
        
        let element2 = Element::new("div".to_string());
        let item2 = GridItem::new(element2).with_placement(Some(2), Some(3), Some(1), Some(3));
        
        engine.add_item(item1);
        engine.add_item(item2);
        
        // Calculate layout
        let layout_boxes = engine.calculate_layout(300.0, 200.0).unwrap();
        
        assert_eq!(layout_boxes.len(), 2);
        
        // Check first item (spans both columns)
        assert_eq!(layout_boxes[0].position_coords.x, 0.0);
        assert_eq!(layout_boxes[0].position_coords.y, 0.0);
        assert_eq!(layout_boxes[0].dimensions.content_width, 300.0); // Both columns
        assert_eq!(layout_boxes[0].dimensions.content_height, 100.0); // First row
        
        // Check second item (spans both columns)
        assert_eq!(layout_boxes[1].position_coords.x, 0.0);
        assert_eq!(layout_boxes[1].position_coords.y, 100.0);
        assert_eq!(layout_boxes[1].dimensions.content_width, 300.0); // Both columns
        assert_eq!(layout_boxes[1].dimensions.content_height, 100.0); // Second row
    }

    #[test]
    fn test_fractional_units() {
        let mut container = GridContainer::new();
        
        // Create grid with fractional units
        let row_template = GridTemplate {
            lines: vec![
                GridLine { name: None, start: GridTemplateUnit::Px(50.0), end: None },
                GridLine { name: None, start: GridTemplateUnit::Fraction(1.0), end: None },
                GridLine { name: None, start: GridTemplateUnit::Fraction(2.0), end: None },
            ],
        };
        
        let column_template = GridTemplate {
            lines: vec![
                GridLine { name: None, start: GridTemplateUnit::Fraction(1.0), end: None },
                GridLine { name: None, start: GridTemplateUnit::Fraction(1.0), end: None },
            ],
        };
        
        container = container
            .with_template_rows(row_template)
            .with_template_columns(column_template);
        
        let mut engine = GridLayoutEngine::new(container);
        
        // Add a grid item
        let element = Element::new("div".to_string());
        let item = GridItem::new(element).with_placement(Some(1), Some(4), Some(1), Some(3));
        
        engine.add_item(item);
        
        // Calculate layout with 200px height
        let layout_boxes = engine.calculate_layout(200.0, 200.0).unwrap();
        
        assert_eq!(layout_boxes.len(), 1);
        
        // Check that the item spans the full height
        assert_eq!(layout_boxes[0].dimensions.content_height, 200.0);
        assert_eq!(layout_boxes[0].dimensions.content_width, 200.0);
    }
}
