//! Graphics implementation for the Matte browser

pub mod error;
pub mod rendering;
pub mod compositor;

pub use error::{Error, Result};
pub use rendering::{
    Color, Point, Rectangle, Circle, Line, Polygon, Path, PathSegment,
    FillRule, LineCap, LineJoin, GradientType, GradientStop, Gradient,
    PatternRepeat, Pattern, Transform, DrawingStyle, BlendMode,
    FontFamily, FontStyle, FontWeight, FontStyleType, FontStretch,
    TextMetrics, TextAlign, TextBaseline, ImageFormat, Image,
    CSSValue, CSSUnit, CSSRule, CSSStylesheet,
    RenderingContext, GraphicsPrimitives, TextRenderer, ImageDecoder, CSSRenderer,
};
pub use compositor::{
    LayerType, LayerBlendMode, LayerState, Layer, FrameTiming,
    VsyncMode, HardwareAcceleration, WindowState, WindowEvent, TouchPhase,
    WindowConfig, CompositorStats, LayerManager, LayerEvent,
    HardwareAccelerator, GpuContext, Shader, Texture, Buffer,
    ShaderType, TextureFormat, BufferType, UniformValue, DrawCall, DrawMode, GpuCapabilities,
    VsyncManager, WindowManager, Window, Compositor, FrameEvent,
};

#[cfg(test)]
mod rendering_test;
#[cfg(test)]
mod compositor_test;
