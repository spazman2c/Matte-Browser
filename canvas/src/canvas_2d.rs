use crate::error::{Error, Result};
use crate::graphics::{Color, Point, Rectangle, Circle, Line, Polygon, Path, Transform, DrawingStyle, BlendMode};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

/// 2D Canvas context
pub struct Canvas2DContext {
    /// Canvas element
    canvas: Arc<CanvasElement>,
    /// Current state
    state: CanvasState,
    /// State stack for save/restore
    state_stack: Vec<CanvasState>,
    /// Rendering context
    rendering_context: Arc<RwLock<graphics::RenderingContext>>,
    /// Path builder
    path_builder: PathBuilder,
    /// Image cache
    image_cache: Arc<RwLock<HashMap<String, Arc<CanvasImage>>>>,
    /// Font cache
    font_cache: Arc<RwLock<HashMap<String, Arc<CanvasFont>>>>,
    /// Pattern cache
    pattern_cache: Arc<RwLock<HashMap<String, Arc<CanvasPattern>>>>,
    /// Gradient cache
    gradient_cache: Arc<RwLock<HashMap<String, Arc<CanvasGradient>>>>,
}

/// Canvas element
#[derive(Debug, Clone)]
pub struct CanvasElement {
    /// Canvas ID
    pub id: String,
    /// Canvas width
    pub width: u32,
    /// Canvas height
    pub height: u32,
    /// Canvas style
    pub style: CanvasStyle,
    /// Canvas attributes
    pub attributes: HashMap<String, String>,
}

/// Canvas style
#[derive(Debug, Clone)]
pub struct CanvasStyle {
    /// CSS width
    pub width: Option<String>,
    /// CSS height
    pub height: Option<String>,
    /// CSS display
    pub display: Option<String>,
    /// CSS position
    pub position: Option<String>,
    /// CSS z-index
    pub z_index: Option<i32>,
}

/// Canvas state
#[derive(Debug, Clone)]
pub struct CanvasState {
    /// Global alpha
    pub global_alpha: f32,
    /// Global composite operation
    pub global_composite_operation: CompositeOperation,
    /// Fill style
    pub fill_style: FillStyle,
    /// Stroke style
    pub stroke_style: StrokeStyle,
    /// Line width
    pub line_width: f32,
    /// Line cap
    pub line_cap: LineCap,
    /// Line join
    pub line_join: LineJoin,
    /// Miter limit
    pub miter_limit: f32,
    /// Shadow offset X
    pub shadow_offset_x: f32,
    /// Shadow offset Y
    pub shadow_offset_y: f32,
    /// Shadow blur
    pub shadow_blur: f32,
    /// Shadow color
    pub shadow_color: Color,
    /// Font
    pub font: String,
    /// Text align
    pub text_align: TextAlign,
    /// Text baseline
    pub text_baseline: TextBaseline,
    /// Transform matrix
    pub transform: Transform,
    /// Image smoothing enabled
    pub image_smoothing_enabled: bool,
    /// Image smoothing quality
    pub image_smoothing_quality: ImageSmoothingQuality,
}

/// Composite operation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CompositeOperation {
    /// Source over
    SourceOver,
    /// Source in
    SourceIn,
    /// Source out
    SourceOut,
    /// Source atop
    SourceAtop,
    /// Destination over
    DestinationOver,
    /// Destination in
    DestinationIn,
    /// Destination out
    DestinationOut,
    /// Destination atop
    DestinationAtop,
    /// Lighter
    Lighter,
    /// Copy
    Copy,
    /// XOR
    Xor,
    /// Multiply
    Multiply,
    /// Screen
    Screen,
    /// Overlay
    Overlay,
    /// Darken
    Darken,
    /// Lighten
    Lighten,
    /// Color dodge
    ColorDodge,
    /// Color burn
    ColorBurn,
    /// Hard light
    HardLight,
    /// Soft light
    SoftLight,
    /// Difference
    Difference,
    /// Exclusion
    Exclusion,
    /// Hue
    Hue,
    /// Saturation
    Saturation,
    /// Color
    Color,
    /// Luminosity
    Luminosity,
}

/// Fill style
#[derive(Debug, Clone)]
pub enum FillStyle {
    /// Color fill
    Color(Color),
    /// Gradient fill
    Gradient(Arc<CanvasGradient>),
    /// Pattern fill
    Pattern(Arc<CanvasPattern>),
}

/// Stroke style
#[derive(Debug, Clone)]
pub enum StrokeStyle {
    /// Color stroke
    Color(Color),
    /// Gradient stroke
    Gradient(Arc<CanvasGradient>),
    /// Pattern stroke
    Pattern(Arc<CanvasPattern>),
}

/// Line cap
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineCap {
    /// Butt cap
    Butt,
    /// Round cap
    Round,
    /// Square cap
    Square,
}

/// Line join
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineJoin {
    /// Miter join
    Miter,
    /// Round join
    Round,
    /// Bevel join
    Bevel,
}

/// Text align
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextAlign {
    /// Start alignment
    Start,
    /// End alignment
    End,
    /// Left alignment
    Left,
    /// Right alignment
    Right,
    /// Center alignment
    Center,
}

/// Text baseline
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextBaseline {
    /// Top baseline
    Top,
    /// Hanging baseline
    Hanging,
    /// Middle baseline
    Middle,
    /// Alphabetic baseline
    Alphabetic,
    /// Ideographic baseline
    Ideographic,
    /// Bottom baseline
    Bottom,
}

/// Image smoothing quality
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ImageSmoothingQuality {
    /// Low quality
    Low,
    /// Medium quality
    Medium,
    /// High quality
    High,
}

/// Path builder
#[derive(Debug, Clone)]
pub struct PathBuilder {
    /// Current path
    pub path: Path,
    /// Current position
    pub current_position: Point,
    /// Subpath start position
    pub subpath_start: Point,
}

/// Canvas image
#[derive(Debug, Clone)]
pub struct CanvasImage {
    /// Image data
    pub data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Image format
    pub format: ImageFormat,
    /// Image source
    pub source: ImageSource,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ImageFormat {
    /// PNG format
    PNG,
    /// JPEG format
    JPEG,
    /// GIF format
    GIF,
    /// WebP format
    WebP,
    /// BMP format
    BMP,
    /// ICO format
    ICO,
    /// SVG format
    SVG,
}

/// Image source
#[derive(Debug, Clone)]
pub enum ImageSource {
    /// URL source
    Url(String),
    /// Data URL source
    DataUrl(String),
    /// Canvas source
    Canvas(Arc<CanvasElement>),
    /// Video source
    Video(Arc<CanvasVideo>>),
    /// Image bitmap source
    ImageBitmap(Arc<ImageBitmap>),
}

/// Canvas font
#[derive(Debug, Clone)]
pub struct CanvasFont {
    /// Font family
    pub family: String,
    /// Font size
    pub size: f32,
    /// Font weight
    pub weight: FontWeight,
    /// Font style
    pub style: FontStyle,
    /// Font stretch
    pub stretch: FontStretch,
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontWeight {
    /// Normal weight
    Normal,
    /// Bold weight
    Bold,
    /// Bolder weight
    Bolder,
    /// Lighter weight
    Lighter,
    /// Custom weight
    Custom(u16),
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontStyle {
    /// Normal style
    Normal,
    /// Italic style
    Italic,
    /// Oblique style
    Oblique,
}

/// Font stretch
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontStretch {
    /// Normal stretch
    Normal,
    /// Ultra condensed
    UltraCondensed,
    /// Extra condensed
    ExtraCondensed,
    /// Condensed
    Condensed,
    /// Semi condensed
    SemiCondensed,
    /// Semi expanded
    SemiExpanded,
    /// Expanded
    Expanded,
    /// Extra expanded
    ExtraExpanded,
    /// Ultra expanded
    UltraExpanded,
}

/// Canvas pattern
#[derive(Debug, Clone)]
pub struct CanvasPattern {
    /// Pattern image
    pub image: Arc<CanvasImage>,
    /// Pattern repeat
    pub repeat: PatternRepeat,
    /// Pattern transform
    pub transform: Transform,
}

/// Pattern repeat
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PatternRepeat {
    /// Repeat pattern
    Repeat,
    /// Repeat X only
    RepeatX,
    /// Repeat Y only
    RepeatY,
    /// No repeat
    NoRepeat,
}

/// Canvas gradient
#[derive(Debug, Clone)]
pub struct CanvasGradient {
    /// Gradient type
    pub gradient_type: GradientType,
    /// Gradient stops
    pub stops: Vec<GradientStop>,
    /// Gradient transform
    pub transform: Transform,
}

/// Gradient type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GradientType {
    /// Linear gradient
    Linear { x0: f32, y0: f32, x1: f32, y1: f32 },
    /// Radial gradient
    Radial { x0: f32, y0: f32, r0: f32, x1: f32, y1: f32, r1: f32 },
    /// Conic gradient
    Conic { x: f32, y: f32, angle: f32 },
}

/// Gradient stop
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// Stop offset
    pub offset: f32,
    /// Stop color
    pub color: Color,
}

/// Canvas video
#[derive(Debug, Clone)]
pub struct CanvasVideo {
    /// Video source
    pub source: String,
    /// Video width
    pub width: u32,
    /// Video height
    pub height: u32,
    /// Video duration
    pub duration: f64,
    /// Video current time
    pub current_time: f64,
    /// Video paused
    pub paused: bool,
    /// Video muted
    pub muted: bool,
    /// Video volume
    pub volume: f32,
}

/// Image bitmap
#[derive(Debug, Clone)]
pub struct ImageBitmap {
    /// Bitmap data
    pub data: Vec<u8>,
    /// Bitmap width
    pub width: u32,
    /// Bitmap height
    pub height: u32,
    /// Bitmap format
    pub format: BitmapFormat,
}

/// Bitmap format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BitmapFormat {
    /// RGBA8 format
    RGBA8,
    /// RGB8 format
    RGB8,
    /// Gray8 format
    Gray8,
    /// Alpha8 format
    Alpha8,
}

impl Canvas2DContext {
    /// Create new 2D canvas context
    pub fn new(canvas: Arc<CanvasElement>) -> Result<Self> {
        let rendering_context = Arc::new(RwLock::new(graphics::RenderingContext::new(
            canvas.width as usize,
            canvas.height as usize,
        )?));
        
        let state = CanvasState::default();
        
        Ok(Self {
            canvas,
            state,
            state_stack: Vec::new(),
            rendering_context,
            path_builder: PathBuilder::new(),
            image_cache: Arc::new(RwLock::new(HashMap::new())),
            font_cache: Arc::new(RwLock::new(HashMap::new())),
            pattern_cache: Arc::new(RwLock::new(HashMap::new())),
            gradient_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // State management
    /// Save current state
    pub fn save(&mut self) {
        self.state_stack.push(self.state.clone());
    }

    /// Restore previous state
    pub fn restore(&mut self) -> Result<()> {
        self.state = self.state_stack.pop()
            .ok_or_else(|| Error::canvas("No state to restore".to_string()))?;
        Ok(())
    }

    // Transform methods
    /// Scale the context
    pub fn scale(&mut self, x: f32, y: f32) {
        self.state.transform = self.state.transform.multiply(&Transform::scale(x, y));
    }

    /// Rotate the context
    pub fn rotate(&mut self, angle: f32) {
        self.state.transform = self.state.transform.multiply(&Transform::rotate(angle));
    }

    /// Translate the context
    pub fn translate(&mut self, x: f32, y: f32) {
        self.state.transform = self.state.transform.multiply(&Transform::translate(x, y));
    }

    /// Set the transform matrix
    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.state.transform = Transform::new(a, b, c, d, e, f);
    }

    /// Reset the transform matrix
    pub fn reset_transform(&mut self) {
        self.state.transform = Transform::identity();
    }

    // Path methods
    /// Begin a new path
    pub fn begin_path(&mut self) {
        self.path_builder = PathBuilder::new();
    }

    /// Close the current path
    pub fn close_path(&mut self) {
        self.path_builder.close_path();
    }

    /// Move to a point
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.path_builder.move_to(x, y);
    }

    /// Line to a point
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path_builder.line_to(x, y);
    }

    /// Quadratic curve to a point
    pub fn quadratic_curve_to(&mut self, cp1x: f32, cp1y: f32, x: f32, y: f32) {
        self.path_builder.quadratic_curve_to(cp1x, cp1y, x, y);
    }

    /// Bezier curve to a point
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path_builder.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    /// Arc to a point
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
        self.path_builder.arc_to(x1, y1, x2, y2, radius);
    }

    /// Arc
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.path_builder.arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }

    /// Rectangle
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.path_builder.rect(x, y, width, height);
    }

    /// Ellipse
    pub fn ellipse(&mut self, x: f32, y: f32, radius_x: f32, radius_y: f32, rotation: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.path_builder.ellipse(x, y, radius_x, radius_y, rotation, start_angle, end_angle, anticlockwise);
    }

    // Drawing methods
    /// Fill the current path
    pub fn fill(&mut self) -> Result<()> {
        let path = self.path_builder.build();
        let style = self.create_drawing_style_from_fill()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.fill_path(&path)?;
        
        Ok(())
    }

    /// Stroke the current path
    pub fn stroke(&mut self) -> Result<()> {
        let path = self.path_builder.build();
        let style = self.create_drawing_style_from_stroke()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.stroke_path(&path)?;
        
        Ok(())
    }

    /// Fill a rectangle
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<()> {
        let rect = Rectangle::new(x, y, width, height);
        let style = self.create_drawing_style_from_fill()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.fill_rectangle(&rect)?;
        
        Ok(())
    }

    /// Stroke a rectangle
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<()> {
        let rect = Rectangle::new(x, y, width, height);
        let style = self.create_drawing_style_from_stroke()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.stroke_rectangle(&rect)?;
        
        Ok(())
    }

    /// Clear a rectangle
    pub fn clear_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<()> {
        let rect = Rectangle::new(x, y, width, height);
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.clear_rectangle(&rect)?;
        
        Ok(())
    }

    // Text methods
    /// Fill text
    pub fn fill_text(&mut self, text: &str, x: f32, y: f32) -> Result<()> {
        let style = self.create_drawing_style_from_fill()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.fill_text(text, x, y)?;
        
        Ok(())
    }

    /// Stroke text
    pub fn stroke_text(&mut self, text: &str, x: f32, y: f32) -> Result<()> {
        let style = self.create_drawing_style_from_stroke()?;
        
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.set_style(style);
        context.stroke_text(text, x, y)?;
        
        Ok(())
    }

    /// Measure text
    pub fn measure_text(&self, text: &str) -> Result<TextMetrics> {
        let mut context = self.rendering_context.write();
        context.measure_text(text)
    }

    // Image methods
    /// Draw image
    pub fn draw_image(&mut self, image: &CanvasImage, dx: f32, dy: f32) -> Result<()> {
        self.draw_image_scaled(image, dx, dy, image.width as f32, image.height as f32)
    }

    /// Draw image scaled
    pub fn draw_image_scaled(&mut self, image: &CanvasImage, dx: f32, dy: f32, d_width: f32, d_height: f32) -> Result<()> {
        self.draw_image_sliced(image, 0.0, 0.0, image.width as f32, image.height as f32, dx, dy, d_width, d_height)
    }

    /// Draw image sliced
    pub fn draw_image_sliced(&mut self, image: &CanvasImage, sx: f32, sy: f32, s_width: f32, s_height: f32, dx: f32, dy: f32, d_width: f32, d_height: f32) -> Result<()> {
        let mut context = self.rendering_context.write();
        context.set_transform(self.state.transform);
        context.draw_image(image, sx, sy, s_width, s_height, dx, dy, d_width, d_height)?;
        
        Ok(())
    }

    // Image data methods
    /// Get image data
    pub fn get_image_data(&self, sx: i32, sy: i32, sw: u32, sh: u32) -> Result<ImageData> {
        let context = self.rendering_context.read();
        let data = context.get_image_data(sx, sy, sw, sh)?;
        
        Ok(ImageData {
            data,
            width: sw,
            height: sh,
        })
    }

    /// Put image data
    pub fn put_image_data(&mut self, image_data: &ImageData, dx: i32, dy: i32) -> Result<()> {
        let mut context = self.rendering_context.write();
        context.put_image_data(&image_data.data, dx, dy, image_data.width, image_data.height)?;
        
        Ok(())
    }

    /// Put image data with dirty rectangle
    pub fn put_image_data_dirty(&mut self, image_data: &ImageData, dx: i32, dy: i32, dirty_x: i32, dirty_y: i32, dirty_width: u32, dirty_height: u32) -> Result<()> {
        let mut context = self.rendering_context.write();
        context.put_image_data_dirty(&image_data.data, dx, dy, image_data.width, image_data.height, dirty_x, dirty_y, dirty_width, dirty_height)?;
        
        Ok(())
    }

    // Helper methods
    /// Create drawing style from fill style
    fn create_drawing_style_from_fill(&self) -> Result<DrawingStyle> {
        let fill_color = match &self.state.fill_style {
            FillStyle::Color(color) => Some(color.clone()),
            FillStyle::Gradient(gradient) => {
                // TODO: Implement gradient filling
                Some(Color::black())
            }
            FillStyle::Pattern(pattern) => {
                // TODO: Implement pattern filling
                Some(Color::black())
            }
        };
        
        Ok(DrawingStyle {
            fill_color,
            fill_gradient: None,
            fill_pattern: None,
            stroke_color: None,
            stroke_width: 0.0,
            line_cap: graphics::LineCap::Butt,
            line_join: graphics::LineJoin::Miter,
            miter_limit: self.state.miter_limit,
            fill_rule: graphics::FillRule::NonZero,
            opacity: self.state.global_alpha,
            blend_mode: self.state.global_composite_operation.into(),
        })
    }

    /// Create drawing style from stroke style
    fn create_drawing_style_from_stroke(&self) -> Result<DrawingStyle> {
        let stroke_color = match &self.state.stroke_style {
            StrokeStyle::Color(color) => Some(color.clone()),
            StrokeStyle::Gradient(gradient) => {
                // TODO: Implement gradient stroking
                Some(Color::black())
            }
            StrokeStyle::Pattern(pattern) => {
                // TODO: Implement pattern stroking
                Some(Color::black())
            }
        };
        
        Ok(DrawingStyle {
            fill_color: None,
            fill_gradient: None,
            fill_pattern: None,
            stroke_color,
            stroke_width: self.state.line_width,
            line_cap: self.state.line_cap.into(),
            line_join: self.state.line_join.into(),
            miter_limit: self.state.miter_limit,
            fill_rule: graphics::FillRule::NonZero,
            opacity: self.state.global_alpha,
            blend_mode: self.state.global_composite_operation.into(),
        })
    }

    // Getters and setters
    /// Get canvas element
    pub fn canvas(&self) -> &Arc<CanvasElement> {
        &self.canvas
    }

    /// Get canvas width
    pub fn width(&self) -> u32 {
        self.canvas.width
    }

    /// Get canvas height
    pub fn height(&self) -> u32 {
        self.canvas.height
    }

    /// Get global alpha
    pub fn global_alpha(&self) -> f32 {
        self.state.global_alpha
    }

    /// Set global alpha
    pub fn set_global_alpha(&mut self, alpha: f32) {
        self.state.global_alpha = alpha.clamp(0.0, 1.0);
    }

    /// Get global composite operation
    pub fn global_composite_operation(&self) -> CompositeOperation {
        self.state.global_composite_operation
    }

    /// Set global composite operation
    pub fn set_global_composite_operation(&mut self, operation: CompositeOperation) {
        self.state.global_composite_operation = operation;
    }

    /// Get fill style
    pub fn fill_style(&self) -> &FillStyle {
        &self.state.fill_style
    }

    /// Set fill style
    pub fn set_fill_style(&mut self, style: FillStyle) {
        self.state.fill_style = style;
    }

    /// Get stroke style
    pub fn stroke_style(&self) -> &StrokeStyle {
        &self.state.stroke_style
    }

    /// Set stroke style
    pub fn set_stroke_style(&mut self, style: StrokeStyle) {
        self.state.stroke_style = style;
    }

    /// Get line width
    pub fn line_width(&self) -> f32 {
        self.state.line_width
    }

    /// Set line width
    pub fn set_line_width(&mut self, width: f32) {
        self.state.line_width = width.max(0.0);
    }

    /// Get line cap
    pub fn line_cap(&self) -> LineCap {
        self.state.line_cap
    }

    /// Set line cap
    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.state.line_cap = cap;
    }

    /// Get line join
    pub fn line_join(&self) -> LineJoin {
        self.state.line_join
    }

    /// Set line join
    pub fn set_line_join(&mut self, join: LineJoin) {
        self.state.line_join = join;
    }

    /// Get miter limit
    pub fn miter_limit(&self) -> f32 {
        self.state.miter_limit
    }

    /// Set miter limit
    pub fn set_miter_limit(&mut self, limit: f32) {
        self.state.miter_limit = limit.max(0.0);
    }

    /// Get font
    pub fn font(&self) -> &str {
        &self.state.font
    }

    /// Set font
    pub fn set_font(&mut self, font: String) {
        self.state.font = font;
    }

    /// Get text align
    pub fn text_align(&self) -> TextAlign {
        self.state.text_align
    }

    /// Set text align
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.state.text_align = align;
    }

    /// Get text baseline
    pub fn text_baseline(&self) -> TextBaseline {
        self.state.text_baseline
    }

    /// Set text baseline
    pub fn set_text_baseline(&mut self, baseline: TextBaseline) {
        self.state.text_baseline = baseline;
    }

    /// Get image smoothing enabled
    pub fn image_smoothing_enabled(&self) -> bool {
        self.state.image_smoothing_enabled
    }

    /// Set image smoothing enabled
    pub fn set_image_smoothing_enabled(&mut self, enabled: bool) {
        self.state.image_smoothing_enabled = enabled;
    }

    /// Get image smoothing quality
    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.state.image_smoothing_quality
    }

    /// Set image smoothing quality
    pub fn set_image_smoothing_quality(&mut self, quality: ImageSmoothingQuality) {
        self.state.image_smoothing_quality = quality;
    }
}

/// Image data
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Image data
    pub data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
}

/// Text metrics
#[derive(Debug, Clone)]
pub struct TextMetrics {
    /// Text width
    pub width: f32,
    /// Actual bounding box left
    pub actual_bounding_box_left: f32,
    /// Actual bounding box right
    pub actual_bounding_box_right: f32,
    /// Font bounding box ascent
    pub font_bounding_box_ascent: f32,
    /// Font bounding box descent
    pub font_bounding_box_descent: f32,
    /// Actual bounding box ascent
    pub actual_bounding_box_ascent: f32,
    /// Actual bounding box descent
    pub actual_bounding_box_descent: f32,
    /// Em height ascent
    pub em_height_ascent: f32,
    /// Em height descent
    pub em_height_descent: f32,
    /// Hanging baseline
    pub hanging_baseline: f32,
    /// Alphabetic baseline
    pub alphabetic_baseline: f32,
    /// Ideographic baseline
    pub ideographic_baseline: f32,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            global_alpha: 1.0,
            global_composite_operation: CompositeOperation::SourceOver,
            fill_style: FillStyle::Color(Color::black()),
            stroke_style: StrokeStyle::Color(Color::black()),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
            shadow_color: Color::transparent(),
            font: "10px sans-serif".to_string(),
            text_align: TextAlign::Start,
            text_baseline: TextBaseline::Alphabetic,
            transform: Transform::identity(),
            image_smoothing_enabled: true,
            image_smoothing_quality: ImageSmoothingQuality::Low,
        }
    }
}

impl PathBuilder {
    /// Create new path builder
    pub fn new() -> Self {
        Self {
            path: Path::new(),
            current_position: Point::new(0.0, 0.0),
            subpath_start: Point::new(0.0, 0.0),
        }
    }

    /// Move to a point
    pub fn move_to(&mut self, x: f32, y: f32) {
        let point = Point::new(x, y);
        self.current_position = point;
        self.subpath_start = point;
        self.path.add_segment(graphics::PathSegment::MoveTo(point));
    }

    /// Line to a point
    pub fn line_to(&mut self, x: f32, y: f32) {
        let point = Point::new(x, y);
        self.current_position = point;
        self.path.add_segment(graphics::PathSegment::LineTo(point));
    }

    /// Quadratic curve to a point
    pub fn quadratic_curve_to(&mut self, cp1x: f32, cp1y: f32, x: f32, y: f32) {
        let cp1 = Point::new(cp1x, cp1y);
        let point = Point::new(x, y);
        self.current_position = point;
        self.path.add_segment(graphics::PathSegment::CurveTo(cp1, point));
    }

    /// Bezier curve to a point
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        let cp1 = Point::new(cp1x, cp1y);
        let cp2 = Point::new(cp2x, cp2y);
        let point = Point::new(x, y);
        self.current_position = point;
        self.path.add_segment(graphics::PathSegment::CurveTo(cp1, cp2, point));
    }

    /// Arc to a point
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        self.path.add_segment(graphics::PathSegment::ArcTo(p1, p2, radius));
        self.current_position = p2;
    }

    /// Arc
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        let center = Point::new(x, y);
        self.path.add_segment(graphics::PathSegment::Arc(center, radius, start_angle, end_angle, anticlockwise));
        
        // Update current position to end of arc
        let end_x = x + radius * end_angle.cos();
        let end_y = y + radius * end_angle.sin();
        self.current_position = Point::new(end_x, end_y);
    }

    /// Rectangle
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.move_to(x, y);
        self.line_to(x + width, y);
        self.line_to(x + width, y + height);
        self.line_to(x, y + height);
        self.close_path();
    }

    /// Ellipse
    pub fn ellipse(&mut self, x: f32, y: f32, radius_x: f32, radius_y: f32, rotation: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        // TODO: Implement ellipse path
        // For now, approximate with arc
        self.arc(x, y, radius_x, start_angle, end_angle, anticlockwise);
    }

    /// Close path
    pub fn close_path(&mut self) {
        if self.current_position != self.subpath_start {
            self.line_to(self.subpath_start.x, self.subpath_start.y);
        }
        self.path.add_segment(graphics::PathSegment::ClosePath);
    }

    /// Build the path
    pub fn build(&self) -> Path {
        self.path.clone()
    }
}

impl From<CompositeOperation> for BlendMode {
    fn from(operation: CompositeOperation) -> Self {
        match operation {
            CompositeOperation::SourceOver => BlendMode::Normal,
            CompositeOperation::SourceIn => BlendMode::SourceIn,
            CompositeOperation::SourceOut => BlendMode::SourceOut,
            CompositeOperation::SourceAtop => BlendMode::SourceAtop,
            CompositeOperation::DestinationOver => BlendMode::DestinationOver,
            CompositeOperation::DestinationIn => BlendMode::DestinationIn,
            CompositeOperation::DestinationOut => BlendMode::DestinationOut,
            CompositeOperation::DestinationAtop => BlendMode::DestinationAtop,
            CompositeOperation::Lighter => BlendMode::Lighten,
            CompositeOperation::Copy => BlendMode::Copy,
            CompositeOperation::Xor => BlendMode::Xor,
            CompositeOperation::Multiply => BlendMode::Multiply,
            CompositeOperation::Screen => BlendMode::Screen,
            CompositeOperation::Overlay => BlendMode::Overlay,
            CompositeOperation::Darken => BlendMode::Darken,
            CompositeOperation::Lighten => BlendMode::Lighten,
            CompositeOperation::ColorDodge => BlendMode::ColorDodge,
            CompositeOperation::ColorBurn => BlendMode::ColorBurn,
            CompositeOperation::HardLight => BlendMode::HardLight,
            CompositeOperation::SoftLight => BlendMode::SoftLight,
            CompositeOperation::Difference => BlendMode::Difference,
            CompositeOperation::Exclusion => BlendMode::Exclusion,
            CompositeOperation::Hue => BlendMode::Hue,
            CompositeOperation::Saturation => BlendMode::Saturation,
            CompositeOperation::Color => BlendMode::Color,
            CompositeOperation::Luminosity => BlendMode::Luminosity,
        }
    }
}

impl From<LineCap> for graphics::LineCap {
    fn from(cap: LineCap) -> Self {
        match cap {
            LineCap::Butt => graphics::LineCap::Butt,
            LineCap::Round => graphics::LineCap::Round,
            LineCap::Square => graphics::LineCap::Square,
        }
    }
}

impl From<LineJoin> for graphics::LineJoin {
    fn from(join: LineJoin) -> Self {
        match join {
            LineJoin::Miter => graphics::LineJoin::Miter,
            LineJoin::Round => graphics::LineJoin::Round,
            LineJoin::Bevel => graphics::LineJoin::Bevel,
        }
    }
}
