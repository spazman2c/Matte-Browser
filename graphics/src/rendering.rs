use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Point in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Circle
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

/// Line
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

/// Polygon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Point>,
}

/// Path segment type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    CurveTo(Point, Point, Point), // Control1, Control2, End
    ArcTo(Point, Point, f32),     // Control1, Control2, Radius
    ClosePath,
}

/// Path
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

/// Fill rule
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

/// Line cap style
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Line join style
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Gradient type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GradientType {
    Linear { start: Point, end: Point },
    Radial { center: Point, radius: f32 },
    Conic { center: Point, angle: f32 },
}

/// Gradient stop
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    pub offset: f32, // 0.0 to 1.0
    pub color: Color,
}

/// Gradient
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    pub gradient_type: GradientType,
    pub stops: Vec<GradientStop>,
}

/// Pattern repeat mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PatternRepeat {
    NoRepeat,
    Repeat,
    RepeatX,
    RepeatY,
}

/// Pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    pub image: Arc<Image>,
    pub repeat: PatternRepeat,
    pub transform: Transform,
}

/// Transform matrix
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub a: f32, pub b: f32, pub c: f32,
    pub d: f32, pub e: f32, pub f: f32,
}

/// Drawing style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawingStyle {
    pub fill_color: Option<Color>,
    pub fill_gradient: Option<Gradient>,
    pub fill_pattern: Option<Pattern>,
    pub stroke_color: Option<Color>,
    pub stroke_gradient: Option<Gradient>,
    pub stroke_pattern: Option<Pattern>,
    pub stroke_width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub fill_rule: FillRule,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Font family
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontFamily {
    pub name: String,
    pub styles: Vec<FontStyle>,
}

/// Font style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontStyle {
    pub weight: FontWeight,
    pub style: FontStyleType,
    pub stretch: FontStretch,
    pub file_path: Option<PathBuf>,
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Font style type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontStyleType {
    Normal,
    Italic,
    Oblique,
}

/// Font stretch
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontStretch {
    UltraCondensed = 50,
    ExtraCondensed = 62,
    Condensed = 75,
    SemiCondensed = 87,
    Normal = 100,
    SemiExpanded = 112,
    Expanded = 125,
    ExtraExpanded = 150,
    UltraExpanded = 200,
}

/// Text metrics
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

/// Text baseline
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    GIF,
    WebP,
    BMP,
    ICO,
    SVG,
}

/// Image data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
    pub channels: u8, // 1 (grayscale), 3 (RGB), 4 (RGBA)
}

/// CSS property value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CSSValue {
    Keyword(String),
    Length(f32, CSSUnit),
    Percentage(f32),
    Color(Color),
    Number(f32),
    String(String),
    Function(String, Vec<CSSValue>),
    List(Vec<CSSValue>),
}

/// CSS unit
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CSSUnit {
    Px,
    Em,
    Rem,
    Ex,
    Ch,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Percent,
    Pt,
    Pc,
    In,
    Cm,
    Mm,
}

/// CSS rule
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSSRule {
    pub selector: String,
    pub properties: HashMap<String, CSSValue>,
    pub specificity: u32,
}

/// CSS stylesheet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSSStylesheet {
    pub rules: Vec<CSSRule>,
}

/// Rendering context
pub struct RenderingContext {
    /// Current transform
    transform: Transform,
    /// Current drawing style
    style: DrawingStyle,
    /// Font registry
    fonts: Arc<RwLock<HashMap<String, FontFamily>>>,
    /// Image cache
    images: Arc<RwLock<HashMap<String, Arc<Image>>>>,
    /// CSS stylesheets
    stylesheets: Arc<RwLock<Vec<CSSStylesheet>>>,
    /// Rendering surface
    surface: Arc<RwLock<Vec<u8>>>,
    /// Surface dimensions
    width: u32,
    height: u32,
    /// Background color
    background: Color,
}

/// 2D graphics primitives renderer
pub struct GraphicsPrimitives {
    /// Rendering context
    context: Arc<RwLock<RenderingContext>>,
}

/// Text renderer
pub struct TextRenderer {
    /// Font registry
    fonts: Arc<RwLock<HashMap<String, FontFamily>>>,
    /// Text cache
    text_cache: Arc<RwLock<HashMap<String, Arc<Image>>>>,
}

/// Image decoder
pub struct ImageDecoder {
    /// Supported formats
    supported_formats: Vec<ImageFormat>,
    /// Decoder cache
    decoder_cache: Arc<RwLock<HashMap<String, Arc<Image>>>>,
}

/// CSS renderer
pub struct CSSRenderer {
    /// Stylesheets
    stylesheets: Arc<RwLock<Vec<CSSStylesheet>>>,
    /// Computed styles cache
    computed_styles: Arc<RwLock<HashMap<String, HashMap<String, CSSValue>>>>,
}

impl Color {
    /// Create new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create color from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create color from RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create color from hex string
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 && hex.len() != 8 {
            return Err(Error::graphics("Invalid hex color format".to_string()));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| Error::graphics("Invalid hex color".to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| Error::graphics("Invalid hex color".to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| Error::graphics("Invalid hex color".to_string()))?;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| Error::graphics("Invalid hex color".to_string()))?
        } else {
            255
        };

        Ok(Self { r, g, b, a })
    }

    /// Blend with another color
    pub fn blend(&self, other: &Color, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        let inv_factor = 1.0 - factor;
        
        Self {
            r: ((self.r as f32 * inv_factor) + (other.r as f32 * factor)) as u8,
            g: ((self.g as f32 * inv_factor) + (other.g as f32 * factor)) as u8,
            b: ((self.b as f32 * inv_factor) + (other.b as f32 * factor)) as u8,
            a: ((self.a as f32 * inv_factor) + (other.a as f32 * factor)) as u8,
        }
    }

    /// Convert to RGBA bytes
    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Point {
    /// Create new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Transform point
    pub fn transform(&self, transform: &Transform) -> Point {
        Point {
            x: transform.a * self.x + transform.c * self.y + transform.e,
            y: transform.b * self.x + transform.d * self.y + transform.f,
        }
    }
}

impl Rectangle {
    /// Create new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if point is inside rectangle
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    /// Check if rectangle intersects with another
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x &&
        self.y < other.y + other.height && self.y + self.height > other.y
    }

    /// Get intersection rectangle
    pub fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = (self.x + self.width).min(other.x + other.width) - x;
        let height = (self.y + self.height).min(other.y + other.height) - y;

        Some(Rectangle { x, y, width, height })
    }

    /// Get area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Circle {
    /// Create new circle
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check if point is inside circle
    pub fn contains(&self, point: &Point) -> bool {
        self.center.distance_to(point) <= self.radius
    }

    /// Check if circle intersects with another
    pub fn intersects(&self, other: &Circle) -> bool {
        let distance = self.center.distance_to(&other.center);
        distance <= self.radius + other.radius
    }

    /// Get area
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

impl Line {
    /// Create new line
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Get length
    pub fn length(&self) -> f32 {
        self.start.distance_to(&self.end)
    }

    /// Get midpoint
    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }

    /// Check if point is on line (within tolerance)
    pub fn contains_point(&self, point: &Point, tolerance: f32) -> bool {
        let line_length = self.length();
        if line_length == 0.0 {
            return self.start.distance_to(point) <= tolerance;
        }

        let t = ((point.x - self.start.x) * (self.end.x - self.start.x) +
                 (point.y - self.start.y) * (self.end.y - self.start.y)) / (line_length * line_length);
        
        if t < 0.0 || t > 1.0 {
            return false;
        }

        let closest = Point {
            x: self.start.x + t * (self.end.x - self.start.x),
            y: self.start.y + t * (self.end.y - self.start.y),
        };

        point.distance_to(&closest) <= tolerance
    }
}

impl Polygon {
    /// Create new polygon
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Check if point is inside polygon (ray casting algorithm)
    pub fn contains(&self, point: &Point) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = self.points.len() - 1;

        for i in 0..self.points.len() {
            if ((self.points[i].y > point.y) != (self.points[j].y > point.y)) &&
               (point.x < (self.points[j].x - self.points[i].x) * (point.y - self.points[i].y) /
                         (self.points[j].y - self.points[i].y) + self.points[i].x) {
                inside = !inside;
            }
            j = i;
        }

        inside
    }

    /// Get bounding rectangle
    pub fn bounding_rect(&self) -> Option<Rectangle> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;

        for point in &self.points[1..] {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Some(Rectangle {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }
}

impl Path {
    /// Create new path
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    /// Move to point
    pub fn move_to(&mut self, point: Point) {
        self.segments.push(PathSegment::MoveTo(point));
    }

    /// Line to point
    pub fn line_to(&mut self, point: Point) {
        self.segments.push(PathSegment::LineTo(point));
    }

    /// Curve to point
    pub fn curve_to(&mut self, control1: Point, control2: Point, end: Point) {
        self.segments.push(PathSegment::CurveTo(control1, control2, end));
    }

    /// Arc to point
    pub fn arc_to(&mut self, control1: Point, control2: Point, radius: f32) {
        self.segments.push(PathSegment::ArcTo(control1, control2, radius));
    }

    /// Close path
    pub fn close(&mut self) {
        self.segments.push(PathSegment::ClosePath);
    }

    /// Get bounding rectangle
    pub fn bounding_rect(&self) -> Option<Rectangle> {
        if self.segments.is_empty() {
            return None;
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for segment in &self.segments {
            match segment {
                PathSegment::MoveTo(point) |
                PathSegment::LineTo(point) => {
                    min_x = min_x.min(point.x);
                    min_y = min_y.min(point.y);
                    max_x = max_x.max(point.x);
                    max_y = max_y.max(point.y);
                }
                PathSegment::CurveTo(_, _, end) => {
                    min_x = min_x.min(end.x);
                    min_y = min_y.min(end.y);
                    max_x = max_x.max(end.x);
                    max_y = max_y.max(end.y);
                }
                PathSegment::ArcTo(_, control2, _) => {
                    min_x = min_x.min(control2.x);
                    min_y = min_y.min(control2.y);
                    max_x = max_x.max(control2.x);
                    max_y = max_y.max(control2.y);
                }
                PathSegment::ClosePath => {}
            }
        }

        if min_x == f32::INFINITY {
            None
        } else {
            Some(Rectangle {
                x: min_x,
                y: min_y,
                width: max_x - min_x,
                height: max_y - min_y,
            })
        }
    }
}

impl Transform {
    /// Create identity transform
    pub fn identity() -> Self {
        Self {
            a: 1.0, b: 0.0, c: 0.0,
            d: 1.0, e: 0.0, f: 0.0,
        }
    }

    /// Create translation transform
    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            a: 1.0, b: 0.0, c: 0.0,
            d: 1.0, e: x, f: y,
        }
    }

    /// Create scale transform
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            a: sx, b: 0.0, c: 0.0,
            d: sy, e: 0.0, f: 0.0,
        }
    }

    /// Create rotation transform
    pub fn rotate(angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            a: cos_a, b: sin_a, c: -sin_a,
            d: cos_a, e: 0.0, f: 0.0,
        }
    }

    /// Multiply with another transform
    pub fn multiply(&self, other: &Transform) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }

    /// Invert transform
    pub fn invert(&self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(Self {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            e: (self.c * self.f - self.d * self.e) * inv_det,
            f: (self.b * self.e - self.a * self.f) * inv_det,
        })
    }
}

impl DrawingStyle {
    /// Create default drawing style
    pub fn default() -> Self {
        Self {
            fill_color: Some(Color::rgb(0, 0, 0)),
            fill_gradient: None,
            fill_pattern: None,
            stroke_color: None,
            stroke_gradient: None,
            stroke_pattern: None,
            stroke_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            fill_rule: FillRule::NonZero,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Set fill color
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self.fill_gradient = None;
        self.fill_pattern = None;
        self
    }

    /// Set stroke color
    pub fn with_stroke_color(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    /// Set opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl RenderingContext {
    /// Create new rendering context
    pub fn new(width: u32, height: u32) -> Self {
        let surface_size = (width * height * 4) as usize; // RGBA
        Self {
            transform: Transform::identity(),
            style: DrawingStyle::default(),
            fonts: Arc::new(RwLock::new(HashMap::new())),
            images: Arc::new(RwLock::new(HashMap::new())),
            stylesheets: Arc::new(RwLock::new(Vec::new())),
            surface: Arc::new(RwLock::new(vec![0; surface_size])),
            width,
            height,
            background: Color::rgb(255, 255, 255),
        }
    }

    /// Clear surface
    pub fn clear(&self) {
        let mut surface = self.surface.write();
        let bg = self.background.to_rgba();
        for pixel in surface.chunks_exact_mut(4) {
            pixel.copy_from_slice(&bg);
        }
    }

    /// Set pixel
    pub fn set_pixel(&self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = ((y * self.width + x) * 4) as usize;
        let mut surface = self.surface.write();
        if index + 3 < surface.len() {
            let rgba = color.to_rgba();
            surface[index..index + 4].copy_from_slice(&rgba);
        }
    }

    /// Get pixel
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = ((y * self.width + x) * 4) as usize;
        let surface = self.surface.read();
        if index + 3 < surface.len() {
            Some(Color::rgba(
                surface[index],
                surface[index + 1],
                surface[index + 2],
                surface[index + 3],
            ))
        } else {
            None
        }
    }

    /// Save surface to RGBA buffer
    pub fn get_surface_data(&self) -> Vec<u8> {
        self.surface.read().clone()
    }

    /// Set background color
    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl GraphicsPrimitives {
    /// Create new graphics primitives renderer
    pub fn new(context: Arc<RwLock<RenderingContext>>) -> Self {
        Self { context }
    }

    /// Draw rectangle
    pub fn draw_rectangle(&self, rect: &Rectangle) -> Result<()> {
        let context = self.context.read();
        let style = &context.style;
        
        // Fill rectangle
        if let Some(fill_color) = style.fill_color {
            for y in rect.y as u32..(rect.y + rect.height) as u32 {
                for x in rect.x as u32..(rect.x + rect.width) as u32 {
                    context.set_pixel(x, y, fill_color);
                }
            }
        }
        
        // Stroke rectangle
        if let Some(stroke_color) = style.stroke_color {
            let stroke_width = style.stroke_width as u32;
            for i in 0..stroke_width {
                // Top edge
                for x in rect.x as u32..(rect.x + rect.width) as u32 {
                    context.set_pixel(x, rect.y as u32 + i, stroke_color);
                }
                // Bottom edge
                for x in rect.x as u32..(rect.x + rect.width) as u32 {
                    context.set_pixel(x, (rect.y + rect.height) as u32 - i - 1, stroke_color);
                }
                // Left edge
                for y in rect.y as u32..(rect.y + rect.height) as u32 {
                    context.set_pixel(rect.x as u32 + i, y, stroke_color);
                }
                // Right edge
                for y in rect.y as u32..(rect.y + rect.height) as u32 {
                    context.set_pixel((rect.x + rect.width) as u32 - i - 1, y, stroke_color);
                }
            }
        }
        
        Ok(())
    }

    /// Draw circle
    pub fn draw_circle(&self, circle: &Circle) -> Result<()> {
        let context = self.context.read();
        let style = &context.style;
        
        let center_x = circle.center.x as i32;
        let center_y = circle.center.y as i32;
        let radius = circle.radius as i32;
        
        // Fill circle
        if let Some(fill_color) = style.fill_color {
            for y in -radius..=radius {
                for x in -radius..=radius {
                    if x * x + y * y <= radius * radius {
                        let px = (center_x + x) as u32;
                        let py = (center_y + y) as u32;
                        context.set_pixel(px, py, fill_color);
                    }
                }
            }
        }
        
        // Stroke circle (Bresenham's circle algorithm)
        if let Some(stroke_color) = style.stroke_color {
            let mut x = radius;
            let mut y = 0;
            let mut err = 0;
            
            while x >= y {
                context.set_pixel((center_x + x) as u32, (center_y + y) as u32, stroke_color);
                context.set_pixel((center_x + y) as u32, (center_y + x) as u32, stroke_color);
                context.set_pixel((center_x - y) as u32, (center_y + x) as u32, stroke_color);
                context.set_pixel((center_x - x) as u32, (center_y + y) as u32, stroke_color);
                context.set_pixel((center_x - x) as u32, (center_y - y) as u32, stroke_color);
                context.set_pixel((center_x - y) as u32, (center_y - x) as u32, stroke_color);
                context.set_pixel((center_x + y) as u32, (center_y - x) as u32, stroke_color);
                context.set_pixel((center_x + x) as u32, (center_y - y) as u32, stroke_color);
                
                if err <= 0 {
                    y += 1;
                    err += 2 * y + 1;
                }
                if err > 0 {
                    x -= 1;
                    err -= 2 * x + 1;
                }
            }
        }
        
        Ok(())
    }

    /// Draw line
    pub fn draw_line(&self, line: &Line) -> Result<()> {
        let context = self.context.read();
        let style = &context.style;
        
        if let Some(stroke_color) = style.stroke_color {
            // Bresenham's line algorithm
            let mut x0 = line.start.x as i32;
            let mut y0 = line.start.y as i32;
            let x1 = line.end.x as i32;
            let y1 = line.end.y as i32;
            
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx - dy;
            
            loop {
                context.set_pixel(x0 as u32, y0 as u32, stroke_color);
                
                if x0 == x1 && y0 == y1 {
                    break;
                }
                
                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x0 += sx;
                }
                if e2 < dx {
                    err += dx;
                    y0 += sy;
                }
            }
        }
        
        Ok(())
    }

    /// Draw polygon
    pub fn draw_polygon(&self, polygon: &Polygon) -> Result<()> {
        let context = self.context.read();
        let style = &context.style;
        
        if polygon.points.len() < 3 {
            return Ok(());
        }
        
        // Fill polygon (scanline algorithm)
        if let Some(fill_color) = style.fill_color {
            if let Some(bounds) = polygon.bounding_rect() {
                for y in bounds.y as u32..(bounds.y + bounds.height) as u32 {
                    let mut intersections = Vec::new();
                    
                    // Find intersections with polygon edges
                    for i in 0..polygon.points.len() {
                        let j = (i + 1) % polygon.points.len();
                        let p1 = &polygon.points[i];
                        let p2 = &polygon.points[j];
                        
                        if (p1.y <= y as f32 && p2.y > y as f32) ||
                           (p2.y <= y as f32 && p1.y > y as f32) {
                            let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                            intersections.push(x);
                        }
                    }
                    
                    // Sort intersections
                    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    
                    // Fill between pairs of intersections
                    for i in (0..intersections.len()).step_by(2) {
                        if i + 1 < intersections.len() {
                            let start_x = intersections[i] as u32;
                            let end_x = intersections[i + 1] as u32;
                            for x in start_x..=end_x {
                                context.set_pixel(x, y, fill_color);
                            }
                        }
                    }
                }
            }
        }
        
        // Stroke polygon
        if let Some(stroke_color) = style.stroke_color {
            for i in 0..polygon.points.len() {
                let j = (i + 1) % polygon.points.len();
                let line = Line::new(polygon.points[i], polygon.points[j]);
                self.draw_line(&line)?;
            }
        }
        
        Ok(())
    }

    /// Draw path
    pub fn draw_path(&self, path: &Path) -> Result<()> {
        // TODO: Implement path rendering with curve approximation
        // This is a simplified implementation that only handles line segments
        let mut current_point = None;
        
        for segment in &path.segments {
            match segment {
                PathSegment::MoveTo(point) => {
                    current_point = Some(*point);
                }
                PathSegment::LineTo(point) => {
                    if let Some(start) = current_point {
                        let line = Line::new(start, *point);
                        self.draw_line(&line)?;
                        current_point = Some(*point);
                    }
                }
                PathSegment::CurveTo(_, _, end) => {
                    // TODO: Implement Bezier curve rendering
                    current_point = Some(*end);
                }
                PathSegment::ArcTo(_, control2, _) => {
                    // TODO: Implement arc rendering
                    current_point = Some(*control2);
                }
                PathSegment::ClosePath => {
                    // TODO: Close path to first point
                }
            }
        }
        
        Ok(())
    }
}

impl TextRenderer {
    /// Create new text renderer
    pub fn new() -> Self {
        Self {
            fonts: Arc::new(RwLock::new(HashMap::new())),
            text_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register font family
    pub fn register_font(&self, family: FontFamily) {
        self.fonts.write().insert(family.name.clone(), family);
    }

    /// Measure text
    pub fn measure_text(&self, text: &str, font_size: f32, font_family: &str) -> TextMetrics {
        // TODO: Implement proper text measurement
        // This is a simplified implementation
        let char_width = font_size * 0.6; // Approximate character width
        let width = text.len() as f32 * char_width;
        let height = font_size;
        
        TextMetrics {
            width,
            height,
            baseline: height * 0.8,
            ascent: height * 0.8,
            descent: height * 0.2,
            leading: height * 0.2,
        }
    }

    /// Render text to image
    pub fn render_text(&self, text: &str, font_size: f32, font_family: &str, color: Color) -> Result<Arc<Image>> {
        // TODO: Implement proper text rendering
        // This is a simplified implementation that creates a placeholder image
        let metrics = self.measure_text(text, font_size, font_family);
        let width = metrics.width.ceil() as u32;
        let height = metrics.height.ceil() as u32;
        
        let mut data = vec![0; (width * height * 4) as usize];
        let rgba = color.to_rgba();
        
        // Fill with text color (simplified - just a solid rectangle)
        for pixel in data.chunks_exact_mut(4) {
            pixel.copy_from_slice(&rgba);
        }
        
        Ok(Arc::new(Image {
            width,
            height,
            format: ImageFormat::PNG,
            data,
            channels: 4,
        }))
    }
}

impl ImageDecoder {
    /// Create new image decoder
    pub fn new() -> Self {
        Self {
            supported_formats: vec![
                ImageFormat::PNG,
                ImageFormat::JPEG,
                ImageFormat::GIF,
                ImageFormat::WebP,
                ImageFormat::BMP,
                ImageFormat::ICO,
                ImageFormat::SVG,
            ],
            decoder_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if format is supported
    pub fn is_format_supported(&self, format: &ImageFormat) -> bool {
        self.supported_formats.contains(format)
    }

    /// Decode image from bytes
    pub fn decode(&self, data: &[u8], format: ImageFormat) -> Result<Arc<Image>> {
        // TODO: Implement proper image decoding
        // This is a simplified implementation that creates a placeholder image
        let image = Arc::new(Image {
            width: 100,
            height: 100,
            format,
            data: vec![255; 40000], // 100x100 RGBA
            channels: 4,
        });
        
        Ok(image)
    }

    /// Decode image from file
    pub fn decode_file(&self, path: &PathBuf) -> Result<Arc<Image>> {
        // TODO: Implement file-based image decoding
        let data = std::fs::read(path)
            .map_err(|e| Error::graphics(format!("Failed to read image file: {}", e)))?;
        
        // Determine format from file extension
        let format = if let Some(ext) = path.extension() {
            match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "png" => ImageFormat::PNG,
                "jpg" | "jpeg" => ImageFormat::JPEG,
                "gif" => ImageFormat::GIF,
                "webp" => ImageFormat::WebP,
                "bmp" => ImageFormat::BMP,
                "ico" => ImageFormat::ICO,
                "svg" => ImageFormat::SVG,
                _ => ImageFormat::PNG, // Default
            }
        } else {
            ImageFormat::PNG
        };
        
        self.decode(&data, format)
    }
}

impl CSSRenderer {
    /// Create new CSS renderer
    pub fn new() -> Self {
        Self {
            stylesheets: Arc::new(RwLock::new(Vec::new())),
            computed_styles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add stylesheet
    pub fn add_stylesheet(&self, stylesheet: CSSStylesheet) {
        self.stylesheets.write().push(stylesheet);
    }

    /// Parse CSS from string
    pub fn parse_css(&self, css: &str) -> Result<CSSStylesheet> {
        // TODO: Implement proper CSS parsing
        // This is a simplified implementation
        let mut rules = Vec::new();
        
        // Simple rule parsing (very basic)
        let lines: Vec<&str> = css.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.contains('{') && line.contains('}') {
                if let Some(colon_pos) = line.find(':') {
                    let selector = line[..colon_pos].trim();
                    let value_part = line[colon_pos + 1..].trim();
                    if let Some(brace_pos) = value_part.find('}') {
                        let value = value_part[..brace_pos].trim();
                        
                        let mut properties = HashMap::new();
                        properties.insert("color".to_string(), CSSValue::Keyword(value.to_string()));
                        
                        rules.push(CSSRule {
                            selector: selector.to_string(),
                            properties,
                            specificity: 1,
                        });
                    }
                }
            }
            i += 1;
        }
        
        Ok(CSSStylesheet { rules })
    }

    /// Get computed styles for element
    pub fn get_computed_styles(&self, element_selector: &str) -> HashMap<String, CSSValue> {
        let mut computed_styles = HashMap::new();
        let stylesheets = self.stylesheets.read();
        
        for stylesheet in stylesheets.iter() {
            for rule in &stylesheet.rules {
                if self.selector_matches(&rule.selector, element_selector) {
                    for (property, value) in &rule.properties {
                        computed_styles.insert(property.clone(), value.clone());
                    }
                }
            }
        }
        
        computed_styles
    }

    /// Check if selector matches element
    fn selector_matches(&self, selector: &str, element: &str) -> bool {
        // TODO: Implement proper CSS selector matching
        // This is a simplified implementation
        selector == element || selector == "*"
    }
}
