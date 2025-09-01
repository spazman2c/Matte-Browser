use crate::error::{Error, Result};
use crate::layout::{LayoutBox, LayoutContext, LayoutResult, BoxType, Display, Position};
use crate::style::{Style, ComputedStyle, Length, Percentage, Auto, GridTemplateColumns, GridTemplateRows, GridTemplateAreas, GridArea, GridGap, JustifyItems, AlignItems, JustifyContent, AlignContent};
use std::collections::HashMap;

/// Grid container information
#[derive(Debug, Clone)]
pub struct GridContainer {
    /// Grid template columns
    pub template_columns: GridTemplateColumns,
    /// Grid template rows
    pub template_rows: GridTemplateRows,
    /// Grid template areas
    pub template_areas: Option<GridTemplateAreas>,
    /// Grid gap
    pub gap: GridGap,
    /// Justify items
    pub justify_items: JustifyItems,
    /// Align items
    pub align_items: AlignItems,
    /// Justify content
    pub justify_content: JustifyContent,
    /// Align content
    pub align_content: AlignContent,
    /// Container width
    pub width: f32,
    /// Container height
    pub height: f32,
    /// Available width
    pub available_width: f32,
    /// Available height
    pub available_height: f32,
    /// Grid items
    pub items: Vec<GridItem>,
    /// Grid lines
    pub lines: GridLines,
    /// Grid areas
    pub areas: HashMap<String, GridArea>,
}

/// Grid item information
#[derive(Debug, Clone)]
pub struct GridItem {
    /// Layout box
    pub box_: LayoutBox,
    /// Grid area
    pub area: GridArea,
    /// Column start
    pub column_start: i32,
    /// Column end
    pub column_end: i32,
    /// Row start
    pub row_start: i32,
    /// Row end
    pub row_end: i32,
    /// Justify self
    pub justify_self: JustifyItems,
    /// Align self
    pub align_self: AlignItems,
    /// Item width
    pub width: f32,
    /// Item height
    pub height: f32,
    /// Item x position
    pub x: f32,
    /// Item y position
    pub y: f32,
    /// Min width
    pub min_width: f32,
    /// Max width
    pub max_width: f32,
    /// Min height
    pub min_height: f32,
    /// Max height
    pub max_height: f32,
}

/// Grid lines
#[derive(Debug, Clone)]
pub struct GridLines {
    /// Column lines
    pub columns: Vec<GridLine>,
    /// Row lines
    pub rows: Vec<GridLine>,
}

/// Grid line
#[derive(Debug, Clone)]
pub struct GridLine {
    /// Line position
    pub position: f32,
    /// Line size
    pub size: f32,
    /// Line type
    pub line_type: GridLineType,
}

/// Grid line type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridLineType {
    /// Fixed size line
    Fixed,
    /// Flexible size line
    Flexible,
    /// Auto size line
    Auto,
    /// Minmax line
    MinMax,
}

/// Grid track
#[derive(Debug, Clone)]
pub struct GridTrack {
    /// Track size
    pub size: GridTrackSize,
    /// Track min size
    pub min_size: f32,
    /// Track max size
    pub max_size: f32,
    /// Track base size
    pub base_size: f32,
    /// Track growth limit
    pub growth_limit: f32,
    /// Track is frozen
    pub is_frozen: bool,
}

/// Grid track size
#[derive(Debug, Clone)]
pub enum GridTrackSize {
    /// Fixed size
    Fixed(Length),
    /// Percentage size
    Percentage(Percentage),
    /// Flexible size (fr)
    Flexible(f32),
    /// Auto size
    Auto,
    /// Minmax size
    MinMax(Box<GridTrackSize>, Box<GridTrackSize>),
    /// Fit content size
    FitContent(Length),
}

/// Grid layout algorithm
pub struct GridLayout {
    /// Container
    container: GridContainer,
    /// Column tracks
    column_tracks: Vec<GridTrack>,
    /// Row tracks
    row_tracks: Vec<GridTrack>,
    /// Grid lines
    lines: GridLines,
}

impl GridContainer {
    /// Create new grid container
    pub fn new(style: &ComputedStyle, width: f32, height: f32) -> Self {
        Self {
            template_columns: style.grid_template_columns.clone(),
            template_rows: style.grid_template_rows.clone(),
            template_areas: style.grid_template_areas.clone(),
            gap: style.grid_gap.clone(),
            justify_items: style.justify_items,
            align_items: style.align_items,
            justify_content: style.justify_content,
            align_content: style.align_content,
            width,
            height,
            available_width: width,
            available_height: height,
            items: Vec::new(),
            lines: GridLines {
                columns: Vec::new(),
                rows: Vec::new(),
            },
            areas: HashMap::new(),
        }
    }

    /// Add grid item
    pub fn add_item(&mut self, item: GridItem) {
        self.items.push(item);
    }

    /// Parse grid template areas
    pub fn parse_template_areas(&mut self) -> Result<()> {
        if let Some(template_areas) = &self.template_areas {
            let rows = &template_areas.rows;
            let mut areas = HashMap::new();
            
            for (row_index, row) in rows.iter().enumerate() {
                for (col_index, area_name) in row.iter().enumerate() {
                    if area_name != "." {
                        let area = GridArea {
                            name: area_name.clone(),
                            row_start: row_index as i32 + 1,
                            row_end: row_index as i32 + 2,
                            column_start: col_index as i32 + 1,
                            column_end: col_index as i32 + 2,
                        };
                        areas.insert(area_name.clone(), area);
                    }
                }
            }
            
            self.areas = areas;
        }
        
        Ok(())
    }
}

impl GridItem {
    /// Create new grid item
    pub fn new(box_: LayoutBox) -> Self {
        let style = &box_.style;
        Self {
            box_,
            area: GridArea {
                name: String::new(),
                row_start: 1,
                row_end: 2,
                column_start: 1,
                column_end: 2,
            },
            column_start: 1,
            column_end: 2,
            row_start: 1,
            row_end: 2,
            justify_self: style.justify_self,
            align_self: style.align_self,
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    /// Set grid area from style
    pub fn set_grid_area_from_style(&mut self) {
        let style = &self.box_.style;
        self.area = style.grid_area.clone();
        self.column_start = style.grid_column_start;
        self.column_end = style.grid_column_end;
        self.row_start = style.grid_row_start;
        self.row_end = style.grid_row_end;
    }

    /// Get grid area span
    pub fn get_column_span(&self) -> i32 {
        self.column_end - self.column_start
    }

    /// Get grid row span
    pub fn get_row_span(&self) -> i32 {
        self.row_end - self.row_start
    }
}

impl GridLayout {
    /// Create new grid layout
    pub fn new(container: GridContainer) -> Self {
        Self {
            container,
            column_tracks: Vec::new(),
            row_tracks: Vec::new(),
            lines: GridLines {
                columns: Vec::new(),
                rows: Vec::new(),
            },
        }
    }

    /// Layout grid container
    pub fn layout(&mut self, context: &LayoutContext) -> Result<LayoutResult> {
        // Step 1: Initialize the grid
        self.initialize_grid()?;
        
        // Step 2: Resolve the grid track sizing algorithm
        self.resolve_track_sizing()?;
        
        // Step 3: Position the grid items
        self.position_grid_items()?;
        
        // Step 4: Align the grid items
        self.align_grid_items()?;
        
        // Step 5: Determine the grid container size
        self.determine_container_size()?;
        
        Ok(LayoutResult {
            width: self.container.width,
            height: self.container.height,
            children: self.container.items.iter().map(|item| item.box_.clone()).collect(),
        })
    }

    /// Step 1: Initialize the grid
    fn initialize_grid(&mut self) -> Result<()> {
        // Parse grid template areas
        self.container.parse_template_areas()?;
        
        // Create column tracks from template
        self.column_tracks = self.create_tracks_from_template(&self.container.template_columns)?;
        
        // Create row tracks from template
        self.row_tracks = self.create_tracks_from_template(&self.container.template_rows)?;
        
        // Initialize grid lines
        self.initialize_grid_lines()?;
        
        // Set grid areas for items
        for item in &mut self.container.items {
            item.set_grid_area_from_style();
        }
        
        Ok(())
    }

    /// Create tracks from template
    fn create_tracks_from_template(&self, template: &GridTemplateColumns) -> Result<Vec<GridTrack>> {
        let mut tracks = Vec::new();
        
        for track_size in &template.tracks {
            let track = GridTrack {
                size: track_size.clone(),
                min_size: 0.0,
                max_size: f32::INFINITY,
                base_size: 0.0,
                growth_limit: f32::INFINITY,
                is_frozen: false,
            };
            tracks.push(track);
        }
        
        Ok(tracks)
    }

    /// Initialize grid lines
    fn initialize_grid_lines(&mut self) -> Result<()> {
        self.lines.columns.clear();
        self.lines.rows.clear();
        
        // Initialize column lines
        for (i, track) in self.column_tracks.iter().enumerate() {
            let line = GridLine {
                position: 0.0, // Will be calculated later
                size: 0.0,     // Will be calculated later
                line_type: GridLineType::Auto,
            };
            self.lines.columns.push(line);
            
            if i == self.column_tracks.len() - 1 {
                // Add final line
                let final_line = GridLine {
                    position: 0.0,
                    size: 0.0,
                    line_type: GridLineType::Auto,
                };
                self.lines.columns.push(final_line);
            }
        }
        
        // Initialize row lines
        for (i, track) in self.row_tracks.iter().enumerate() {
            let line = GridLine {
                position: 0.0, // Will be calculated later
                size: 0.0,     // Will be calculated later
                line_type: GridLineType::Auto,
            };
            self.lines.rows.push(line);
            
            if i == self.row_tracks.len() - 1 {
                // Add final line
                let final_line = GridLine {
                    position: 0.0,
                    size: 0.0,
                    line_type: GridLineType::Auto,
                };
                self.lines.rows.push(final_line);
            }
        }
        
        Ok(())
    }

    /// Step 2: Resolve the grid track sizing algorithm
    fn resolve_track_sizing(&mut self) -> Result<()> {
        // Resolve column track sizing
        self.resolve_column_track_sizing()?;
        
        // Resolve row track sizing
        self.resolve_row_track_sizing()?;
        
        // Update grid line positions
        self.update_grid_line_positions()?;
        
        Ok(())
    }

    /// Resolve column track sizing
    fn resolve_column_track_sizing(&mut self) -> Result<()> {
        let available_width = self.container.available_width;
        
        for track in &mut self.column_tracks {
            match &track.size {
                GridTrackSize::Fixed(length) => {
                    track.base_size = length.resolve(available_width);
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
                GridTrackSize::Percentage(percentage) => {
                    track.base_size = percentage.resolve(available_width);
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
                GridTrackSize::Flexible(fr) => {
                    track.base_size = 0.0;
                    track.growth_limit = f32::INFINITY;
                    track.is_frozen = false;
                }
                GridTrackSize::Auto => {
                    track.base_size = 0.0;
                    track.growth_limit = f32::INFINITY;
                    track.is_frozen = false;
                }
                GridTrackSize::MinMax(min, max) => {
                    track.min_size = self.resolve_track_size(min, available_width)?;
                    track.max_size = self.resolve_track_size(max, available_width)?;
                    track.base_size = track.min_size;
                    track.growth_limit = track.max_size;
                    track.is_frozen = false;
                }
                GridTrackSize::FitContent(length) => {
                    track.base_size = 0.0;
                    track.growth_limit = length.resolve(available_width);
                    track.is_frozen = false;
                }
            }
        }
        
        // Distribute available space to flexible tracks
        self.distribute_available_space_to_flexible_tracks(&mut self.column_tracks, available_width)?;
        
        Ok(())
    }

    /// Resolve row track sizing
    fn resolve_row_track_sizing(&mut self) -> Result<()> {
        let available_height = self.container.available_height;
        
        for track in &mut self.row_tracks {
            match &track.size {
                GridTrackSize::Fixed(length) => {
                    track.base_size = length.resolve(available_height);
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
                GridTrackSize::Percentage(percentage) => {
                    track.base_size = percentage.resolve(available_height);
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
                GridTrackSize::Flexible(fr) => {
                    track.base_size = 0.0;
                    track.growth_limit = f32::INFINITY;
                    track.is_frozen = false;
                }
                GridTrackSize::Auto => {
                    track.base_size = 0.0;
                    track.growth_limit = f32::INFINITY;
                    track.is_frozen = false;
                }
                GridTrackSize::MinMax(min, max) => {
                    track.min_size = self.resolve_track_size(min, available_height)?;
                    track.max_size = self.resolve_track_size(max, available_height)?;
                    track.base_size = track.min_size;
                    track.growth_limit = track.max_size;
                    track.is_frozen = false;
                }
                GridTrackSize::FitContent(length) => {
                    track.base_size = 0.0;
                    track.growth_limit = length.resolve(available_height);
                    track.is_frozen = false;
                }
            }
        }
        
        // Distribute available space to flexible tracks
        self.distribute_available_space_to_flexible_tracks(&mut self.row_tracks, available_height)?;
        
        Ok(())
    }

    /// Resolve track size
    fn resolve_track_size(&self, track_size: &GridTrackSize, available_size: f32) -> Result<f32> {
        match track_size {
            GridTrackSize::Fixed(length) => Ok(length.resolve(available_size)),
            GridTrackSize::Percentage(percentage) => Ok(percentage.resolve(available_size)),
            GridTrackSize::Flexible(_) => Ok(0.0),
            GridTrackSize::Auto => Ok(0.0),
            GridTrackSize::MinMax(min, max) => {
                let min_size = self.resolve_track_size(min, available_size)?;
                let max_size = self.resolve_track_size(max, available_size)?;
                Ok(min_size.max(max_size))
            }
            GridTrackSize::FitContent(length) => Ok(length.resolve(available_size)),
        }
    }

    /// Distribute available space to flexible tracks
    fn distribute_available_space_to_flexible_tracks(&self, tracks: &mut Vec<GridTrack>, available_size: f32) -> Result<()> {
        let mut remaining_space = available_size;
        let mut flexible_tracks = Vec::new();
        
        // Calculate space used by fixed tracks
        for track in tracks.iter() {
            if track.is_frozen {
                remaining_space -= track.base_size;
            } else {
                flexible_tracks.push(track.clone());
            }
        }
        
        if remaining_space <= 0.0 {
            return Ok(());
        }
        
        // Distribute remaining space to flexible tracks
        let total_fr: f32 = flexible_tracks.iter()
            .filter_map(|track| {
                if let GridTrackSize::Flexible(fr) = &track.size {
                    Some(*fr)
                } else {
                    None
                }
            })
            .sum();
        
        if total_fr > 0.0 {
            let fr_unit = remaining_space / total_fr;
            
            for track in tracks.iter_mut() {
                if let GridTrackSize::Flexible(fr) = &track.size {
                    track.base_size = fr_unit * fr;
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
            }
        } else {
            // Distribute space equally among auto tracks
            let auto_tracks: Vec<_> = tracks.iter_mut()
                .filter(|track| matches!(track.size, GridTrackSize::Auto))
                .collect();
            
            if !auto_tracks.is_empty() {
                let auto_unit = remaining_space / auto_tracks.len() as f32;
                
                for track in auto_tracks {
                    track.base_size = auto_unit;
                    track.growth_limit = track.base_size;
                    track.is_frozen = true;
                }
            }
        }
        
        Ok(())
    }

    /// Update grid line positions
    fn update_grid_line_positions(&mut self) -> Result<()> {
        // Update column line positions
        let mut current_position = 0.0;
        for (i, line) in self.lines.columns.iter_mut().enumerate() {
            line.position = current_position;
            
            if i < self.column_tracks.len() {
                line.size = self.column_tracks[i].base_size;
                current_position += line.size;
                
                // Add gap
                if i < self.column_tracks.len() - 1 {
                    current_position += self.container.gap.column_gap;
                }
            }
        }
        
        // Update row line positions
        let mut current_position = 0.0;
        for (i, line) in self.lines.rows.iter_mut().enumerate() {
            line.position = current_position;
            
            if i < self.row_tracks.len() {
                line.size = self.row_tracks[i].base_size;
                current_position += line.size;
                
                // Add gap
                if i < self.row_tracks.len() - 1 {
                    current_position += self.container.gap.row_gap;
                }
            }
        }
        
        Ok(())
    }

    /// Step 3: Position the grid items
    fn position_grid_items(&mut self) -> Result<()> {
        for item in &mut self.container.items {
            // Calculate item position based on grid area
            let column_start_line = self.lines.columns.get(item.column_start as usize - 1)
                .ok_or_else(|| Error::layout("Invalid column start line".to_string()))?;
            let column_end_line = self.lines.columns.get(item.column_end as usize - 1)
                .ok_or_else(|| Error::layout("Invalid column end line".to_string()))?;
            let row_start_line = self.lines.rows.get(item.row_start as usize - 1)
                .ok_or_else(|| Error::layout("Invalid row start line".to_string()))?;
            let row_end_line = self.lines.rows.get(item.row_end as usize - 1)
                .ok_or_else(|| Error::layout("Invalid row end line".to_string()))?;
            
            // Set item position and size
            item.x = column_start_line.position;
            item.y = row_start_line.position;
            item.width = column_end_line.position - column_start_line.position;
            item.height = row_end_line.position - row_start_line.position;
            
            // Apply min/max constraints
            item.min_width = item.box_.style.min_width.as_ref()
                .map(|w| w.resolve(self.container.width))
                .unwrap_or(0.0);
            item.max_width = item.box_.style.max_width.as_ref()
                .map(|w| w.resolve(self.container.width))
                .unwrap_or(f32::INFINITY);
            item.min_height = item.box_.style.min_height.as_ref()
                .map(|h| h.resolve(self.container.height))
                .unwrap_or(0.0);
            item.max_height = item.box_.style.max_height.as_ref()
                .map(|h| h.resolve(self.container.height))
                .unwrap_or(f32::INFINITY);
            
            item.width = item.width.clamp(item.min_width, item.max_width);
            item.height = item.height.clamp(item.min_height, item.max_height);
        }
        
        Ok(())
    }

    /// Step 4: Align the grid items
    fn align_grid_items(&mut self) -> Result<()> {
        for item in &mut self.container.items {
            // Get alignment values
            let justify_self = if item.justify_self == JustifyItems::Auto {
                self.container.justify_items
            } else {
                item.justify_self
            };
            
            let align_self = if item.align_self == AlignItems::Auto {
                self.container.align_items
            } else {
                item.align_self
            };
            
            // Calculate available space for alignment
            let available_width = item.width - item.box_.style.get_content_width();
            let available_height = item.height - item.box_.style.get_content_height();
            
            // Apply justify-self alignment
            match justify_self {
                JustifyItems::Start => {
                    // Item is already positioned at start
                }
                JustifyItems::End => {
                    item.x += available_width;
                }
                JustifyItems::Center => {
                    item.x += available_width / 2.0;
                }
                JustifyItems::Stretch => {
                    // Item should stretch to fill the grid area
                    // This is handled by setting the item's width/height
                }
            }
            
            // Apply align-self alignment
            match align_self {
                AlignItems::Start => {
                    // Item is already positioned at start
                }
                AlignItems::End => {
                    item.y += available_height;
                }
                AlignItems::Center => {
                    item.y += available_height / 2.0;
                }
                AlignItems::Stretch => {
                    // Item should stretch to fill the grid area
                    // This is handled by setting the item's width/height
                }
                AlignItems::Baseline => {
                    // TODO: Implement baseline alignment
                }
            }
            
            // Update the layout box position
            self.update_layout_box_position(item)?;
        }
        
        Ok(())
    }

    /// Step 5: Determine the grid container size
    fn determine_container_size(&mut self) -> Result<()> {
        // Calculate container size based on grid lines
        if let Some(last_column_line) = self.lines.columns.last() {
            self.container.width = last_column_line.position;
        }
        
        if let Some(last_row_line) = self.lines.rows.last() {
            self.container.height = last_row_line.position;
        }
        
        // Apply justify-content and align-content
        self.apply_container_alignment()?;
        
        Ok(())
    }

    /// Apply container alignment
    fn apply_container_alignment(&mut self) -> Result<()> {
        // TODO: Implement justify-content and align-content
        // This would adjust the position of the entire grid within the container
        Ok(())
    }

    /// Update layout box position
    fn update_layout_box_position(&self, item: &mut GridItem) -> Result<()> {
        let style = &mut item.box_.style;
        
        style.left = Some(crate::style::Length::Px(item.x));
        style.top = Some(crate::style::Length::Px(item.y));
        style.width = Some(crate::style::Length::Px(item.width));
        style.height = Some(crate::style::Length::Px(item.height));
        
        Ok(())
    }
}

impl LayoutBox {
    /// Get content width
    pub fn get_content_width(&self) -> f32 {
        // TODO: Calculate actual content width
        0.0
    }

    /// Get content height
    pub fn get_content_height(&self) -> f32 {
        // TODO: Calculate actual content height
        0.0
    }
}
