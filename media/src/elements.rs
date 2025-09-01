use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

/// Media element base trait
pub trait MediaElement: Send + Sync {
    /// Get media source
    fn source(&self) -> &str;
    /// Get media duration
    fn duration(&self) -> f64;
    /// Get current time
    fn current_time(&self) -> f64;
    /// Set current time
    fn set_current_time(&mut self, time: f64) -> Result<()>;
    /// Get playback rate
    fn playback_rate(&self) -> f64;
    /// Set playback rate
    fn set_playback_rate(&mut self, rate: f64) -> Result<()>;
    /// Get volume
    fn volume(&self) -> f32;
    /// Set volume
    fn set_volume(&mut self, volume: f32) -> Result<()>;
    /// Get muted state
    fn muted(&self) -> bool;
    /// Set muted state
    fn set_muted(&mut self, muted: bool) -> Result<()>;
    /// Get paused state
    fn paused(&self) -> bool;
    /// Play media
    fn play(&mut self) -> Result<()>;
    /// Pause media
    fn pause(&mut self) -> Result<()>;
    /// Get ready state
    fn ready_state(&self) -> ReadyState;
    /// Get network state
    fn network_state(&self) -> NetworkState;
    /// Get error
    fn error(&self) -> Option<MediaError>;
    /// Get media events
    fn events(&self) -> &Arc<RwLock<Vec<MediaEvent>>>;
    /// Add event listener
    fn add_event_listener(&self, event_type: MediaEventType, callback: Box<dyn Fn(MediaEvent) + Send>);
    /// Remove event listener
    fn remove_event_listener(&self, event_type: MediaEventType, callback_id: String);
}

/// Video element
pub struct VideoElement {
    /// Video source
    source: String,
    /// Video width
    width: u32,
    /// Video height
    height: u32,
    /// Video poster
    poster: Option<String>,
    /// Video preload
    preload: PreloadType,
    /// Video autoplay
    autoplay: bool,
    /// Video loop
    loop_: bool,
    /// Video muted
    muted: bool,
    /// Video controls
    controls: bool,
    /// Video cross origin
    cross_origin: Option<String>,
    /// Video media source
    media_source: Arc<RwLock<Option<MediaSource>>>,
    /// Video player
    player: Arc<RwLock<VideoPlayer>>,
    /// Video events
    events: Arc<RwLock<Vec<MediaEvent>>>,
    /// Video event listeners
    event_listeners: Arc<RwLock<HashMap<MediaEventType, Vec<EventCallback>>>>,
}

/// Audio element
pub struct AudioElement {
    /// Audio source
    source: String,
    /// Audio preload
    preload: PreloadType,
    /// Audio autoplay
    autoplay: bool,
    /// Audio loop
    loop_: bool,
    /// Audio muted
    muted: bool,
    /// Audio controls
    controls: bool,
    /// Audio cross origin
    cross_origin: Option<String>,
    /// Audio media source
    media_source: Arc<RwLock<Option<MediaSource>>>,
    /// Audio player
    player: Arc<RwLock<AudioPlayer>>,
    /// Audio events
    events: Arc<RwLock<Vec<MediaEvent>>>,
    /// Audio event listeners
    event_listeners: Arc<RwLock<HashMap<MediaEventType, Vec<EventCallback>>>>,
}

/// Media source
#[derive(Debug, Clone)]
pub struct MediaSource {
    /// Source URL
    pub url: String,
    /// Source type
    pub source_type: MediaSourceType,
    /// Source format
    pub format: MediaFormat,
    /// Source codec
    pub codec: Option<String>,
    /// Source bitrate
    pub bitrate: Option<u32>,
    /// Source width
    pub width: Option<u32>,
    /// Source height
    pub height: Option<u32>,
    /// Source duration
    pub duration: Option<f64>,
    /// Source data
    pub data: Vec<u8>,
}

/// Media source type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MediaSourceType {
    /// File source
    File,
    /// Stream source
    Stream,
    /// Data URL source
    DataUrl,
    /// Blob source
    Blob,
}

/// Media format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MediaFormat {
    /// MP4 format
    MP4,
    /// WebM format
    WebM,
    /// OGG format
    OGG,
    /// AVI format
    AVI,
    /// MOV format
    MOV,
    /// WMV format
    WMV,
    /// FLV format
    FLV,
    /// MKV format
    MKV,
    /// MP3 format
    MP3,
    /// WAV format
    WAV,
    /// AAC format
    AAC,
    /// OGG audio format
    OGG_AUDIO,
    /// FLAC format
    FLAC,
}

/// Preload type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PreloadType {
    /// No preload
    None,
    /// Metadata preload
    Metadata,
    /// Auto preload
    Auto,
}

/// Ready state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReadyState {
    /// Have nothing
    HaveNothing,
    /// Have metadata
    HaveMetadata,
    /// Have current data
    HaveCurrentData,
    /// Have future data
    HaveFutureData,
    /// Have enough data
    HaveEnoughData,
}

/// Network state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NetworkState {
    /// Empty
    Empty,
    /// Idle
    Idle,
    /// Loading
    Loading,
    /// No source
    NoSource,
}

/// Media error
#[derive(Debug, Clone)]
pub struct MediaError {
    /// Error code
    pub code: MediaErrorCode,
    /// Error message
    pub message: String,
}

/// Media error code
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MediaErrorCode {
    /// Media aborted
    MediaAborted,
    /// Network error
    NetworkError,
    /// Media decode error
    MediaDecodeError,
    /// Media source not supported
    MediaSourceNotSupported,
}

/// Media event
#[derive(Debug, Clone)]
pub struct MediaEvent {
    /// Event type
    pub event_type: MediaEventType,
    /// Event timestamp
    pub timestamp: Instant,
    /// Event data
    pub data: MediaEventData,
}

/// Media event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaEventType {
    /// Load start event
    LoadStart,
    /// Duration change event
    DurationChange,
    /// Loaded metadata event
    LoadedMetadata,
    /// Loaded data event
    LoadedData,
    /// Progress event
    Progress,
    /// Can play event
    CanPlay,
    /// Can play through event
    CanPlayThrough,
    /// Play event
    Play,
    /// Playing event
    Playing,
    /// Pause event
    Pause,
    /// Time update event
    TimeUpdate,
    /// Rate change event
    RateChange,
    /// Volume change event
    VolumeChange,
    /// Seeked event
    Seeked,
    /// Seeking event
    Seeking,
    /// Ended event
    Ended,
    /// Error event
    Error,
    /// Abort event
    Abort,
    /// Emptied event
    Emptied,
    /// Stalled event
    Stalled,
    /// Suspend event
    Suspend,
    /// Waiting event
    Waiting,
}

/// Media event data
#[derive(Debug, Clone)]
pub enum MediaEventData {
    /// No data
    None,
    /// Error data
    Error(MediaError),
    /// Time data
    Time { current: f64, duration: f64 },
    /// Progress data
    Progress { loaded: f64, total: f64 },
    /// Rate data
    Rate { old_rate: f64, new_rate: f64 },
    /// Volume data
    Volume { old_volume: f32, new_volume: f32 },
}

/// Event callback
#[derive(Debug)]
pub struct EventCallback {
    /// Callback ID
    pub id: String,
    /// Callback function
    pub callback: Box<dyn Fn(MediaEvent) + Send>,
}

/// Video player
pub struct VideoPlayer {
    /// Player state
    state: VideoPlayerState,
    /// Video decoder
    decoder: Option<VideoDecoder>,
    /// Video renderer
    renderer: Option<VideoRenderer>,
    /// Video buffer
    buffer: VideoBuffer,
    /// Video controls
    controls: VideoControls,
}

/// Video player state
#[derive(Debug, Clone)]
pub struct VideoPlayerState {
    /// Current time
    pub current_time: f64,
    /// Duration
    pub duration: f64,
    /// Playback rate
    pub playback_rate: f64,
    /// Volume
    pub volume: f32,
    /// Muted
    pub muted: bool,
    /// Paused
    pub paused: bool,
    /// Ready state
    pub ready_state: ReadyState,
    /// Network state
    pub network_state: NetworkState,
    /// Error
    pub error: Option<MediaError>,
}

/// Video decoder
pub struct VideoDecoder {
    /// Decoder format
    pub format: MediaFormat,
    /// Decoder codec
    pub codec: String,
    /// Decoder width
    pub width: u32,
    /// Decoder height
    pub height: u32,
    /// Decoder frame rate
    pub frame_rate: f32,
    /// Decoder bitrate
    pub bitrate: u32,
}

/// Video renderer
pub struct VideoRenderer {
    /// Renderer surface
    pub surface: VideoSurface,
    /// Renderer format
    pub format: VideoFormat,
    /// Renderer width
    pub width: u32,
    /// Renderer height
    pub height: u32,
}

/// Video surface
#[derive(Debug, Clone)]
pub struct VideoSurface {
    /// Surface data
    pub data: Vec<u8>,
    /// Surface width
    pub width: u32,
    /// Surface height
    pub height: u32,
    /// Surface format
    pub format: VideoFormat,
    /// Surface stride
    pub stride: u32,
}

/// Video format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VideoFormat {
    /// RGBA8 format
    RGBA8,
    /// BGRA8 format
    BGRA8,
    /// RGB8 format
    RGB8,
    /// BGR8 format
    BGR8,
    /// YUV420P format
    YUV420P,
    /// YUV422P format
    YUV422P,
    /// YUV444P format
    YUV444P,
    /// NV12 format
    NV12,
    /// NV21 format
    NV21,
}

/// Video buffer
pub struct VideoBuffer {
    /// Buffer frames
    pub frames: Vec<VideoFrame>,
    /// Buffer capacity
    pub capacity: usize,
    /// Buffer read position
    pub read_pos: usize,
    /// Buffer write position
    pub write_pos: usize,
}

/// Video frame
#[derive(Debug, Clone)]
pub struct VideoFrame {
    /// Frame data
    pub data: VideoSurface,
    /// Frame timestamp
    pub timestamp: f64,
    /// Frame duration
    pub duration: f64,
    /// Frame key frame
    pub key_frame: bool,
}

/// Video controls
pub struct VideoControls {
    /// Controls visible
    pub visible: bool,
    /// Controls position
    pub position: VideoControlsPosition,
    /// Controls style
    pub style: VideoControlsStyle,
    /// Controls buttons
    pub buttons: Vec<VideoControlButton>,
}

/// Video controls position
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VideoControlsPosition {
    /// Bottom position
    Bottom,
    /// Top position
    Top,
    /// Overlay position
    Overlay,
}

/// Video controls style
#[derive(Debug, Clone)]
pub struct VideoControlsStyle {
    /// Background color
    pub background_color: String,
    /// Foreground color
    pub foreground_color: String,
    /// Border color
    pub border_color: String,
    /// Border width
    pub border_width: u32,
    /// Border radius
    pub border_radius: u32,
    /// Opacity
    pub opacity: f32,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: u32,
}

/// Video control button
#[derive(Debug, Clone)]
pub struct VideoControlButton {
    /// Button type
    pub button_type: VideoControlButtonType,
    /// Button text
    pub text: String,
    /// Button icon
    pub icon: Option<String>,
    /// Button enabled
    pub enabled: bool,
    /// Button visible
    pub visible: bool,
    /// Button action
    pub action: VideoControlAction,
}

/// Video control button type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VideoControlButtonType {
    /// Play button
    Play,
    /// Pause button
    Pause,
    /// Stop button
    Stop,
    /// Rewind button
    Rewind,
    /// Fast forward button
    FastForward,
    /// Volume button
    Volume,
    /// Mute button
    Mute,
    /// Fullscreen button
    Fullscreen,
    /// Settings button
    Settings,
}

/// Video control action
#[derive(Debug, Clone)]
pub enum VideoControlAction {
    /// No action
    None,
    /// Play action
    Play,
    /// Pause action
    Pause,
    /// Stop action
    Stop,
    /// Seek action
    Seek { time: f64 },
    /// Set volume action
    SetVolume { volume: f32 },
    /// Set muted action
    SetMuted { muted: bool },
    /// Toggle fullscreen action
    ToggleFullscreen,
    /// Show settings action
    ShowSettings,
}

/// Audio player
pub struct AudioPlayer {
    /// Player state
    state: AudioPlayerState,
    /// Audio decoder
    decoder: Option<AudioDecoder>,
    /// Audio renderer
    renderer: Option<AudioRenderer>,
    /// Audio buffer
    buffer: AudioBuffer,
    /// Audio controls
    controls: AudioControls,
}

/// Audio player state
#[derive(Debug, Clone)]
pub struct AudioPlayerState {
    /// Current time
    pub current_time: f64,
    /// Duration
    pub duration: f64,
    /// Playback rate
    pub playback_rate: f64,
    /// Volume
    pub volume: f32,
    /// Muted
    pub muted: bool,
    /// Paused
    pub paused: bool,
    /// Ready state
    pub ready_state: ReadyState,
    /// Network state
    pub network_state: NetworkState,
    /// Error
    pub error: Option<MediaError>,
}

/// Audio decoder
pub struct AudioDecoder {
    /// Decoder format
    pub format: MediaFormat,
    /// Decoder codec
    pub codec: String,
    /// Decoder sample rate
    pub sample_rate: u32,
    /// Decoder channels
    pub channels: u32,
    /// Decoder bit depth
    pub bit_depth: u32,
    /// Decoder bitrate
    pub bitrate: u32,
}

/// Audio renderer
pub struct AudioRenderer {
    /// Renderer device
    pub device: AudioDevice,
    /// Renderer format
    pub format: AudioFormat,
    /// Renderer sample rate
    pub sample_rate: u32,
    /// Renderer channels
    pub channels: u32,
}

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    /// Device ID
    pub id: String,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: AudioDeviceType,
    /// Device sample rate
    pub sample_rate: u32,
    /// Device channels
    pub channels: u32,
}

/// Audio device type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioDeviceType {
    /// Default device
    Default,
    /// Output device
    Output,
    /// Input device
    Input,
    /// Communication device
    Communication,
}

/// Audio format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioFormat {
    /// PCM16 format
    PCM16,
    /// PCM24 format
    PCM24,
    /// PCM32 format
    PCM32,
    /// Float32 format
    Float32,
    /// Float64 format
    Float64,
}

/// Audio buffer
pub struct AudioBuffer {
    /// Buffer samples
    pub samples: Vec<AudioSample>,
    /// Buffer capacity
    pub capacity: usize,
    /// Buffer read position
    pub read_pos: usize,
    /// Buffer write position
    pub write_pos: usize,
}

/// Audio sample
#[derive(Debug, Clone)]
pub struct AudioSample {
    /// Sample data
    pub data: Vec<f32>,
    /// Sample timestamp
    pub timestamp: f64,
    /// Sample duration
    pub duration: f64,
    /// Sample channels
    pub channels: u32,
}

/// Audio controls
pub struct AudioControls {
    /// Controls visible
    pub visible: bool,
    /// Controls position
    pub position: AudioControlsPosition,
    /// Controls style
    pub style: AudioControlsStyle,
    /// Controls buttons
    pub buttons: Vec<AudioControlButton>,
}

/// Audio controls position
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioControlsPosition {
    /// Bottom position
    Bottom,
    /// Top position
    Top,
    /// Overlay position
    Overlay,
}

/// Audio controls style
#[derive(Debug, Clone)]
pub struct AudioControlsStyle {
    /// Background color
    pub background_color: String,
    /// Foreground color
    pub foreground_color: String,
    /// Border color
    pub border_color: String,
    /// Border width
    pub border_width: u32,
    /// Border radius
    pub border_radius: u32,
    /// Opacity
    pub opacity: f32,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: u32,
}

/// Audio control button
#[derive(Debug, Clone)]
pub struct AudioControlButton {
    /// Button type
    pub button_type: AudioControlButtonType,
    /// Button text
    pub text: String,
    /// Button icon
    pub icon: Option<String>,
    /// Button enabled
    pub enabled: bool,
    /// Button visible
    pub visible: bool,
    /// Button action
    pub action: AudioControlAction,
}

/// Audio control button type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioControlButtonType {
    /// Play button
    Play,
    /// Pause button
    Pause,
    /// Stop button
    Stop,
    /// Rewind button
    Rewind,
    /// Fast forward button
    FastForward,
    /// Volume button
    Volume,
    /// Mute button
    Mute,
    /// Settings button
    Settings,
}

/// Audio control action
#[derive(Debug, Clone)]
pub enum AudioControlAction {
    /// No action
    None,
    /// Play action
    Play,
    /// Pause action
    Pause,
    /// Stop action
    Stop,
    /// Seek action
    Seek { time: f64 },
    /// Set volume action
    SetVolume { volume: f32 },
    /// Set muted action
    SetMuted { muted: bool },
    /// Show settings action
    ShowSettings,
}

impl VideoElement {
    /// Create new video element
    pub fn new(source: String) -> Self {
        Self {
            source,
            width: 640,
            height: 480,
            poster: None,
            preload: PreloadType::Metadata,
            autoplay: false,
            loop_: false,
            muted: false,
            controls: true,
            cross_origin: None,
            media_source: Arc::new(RwLock::new(None)),
            player: Arc::new(RwLock::new(VideoPlayer::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            event_listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set video width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Set video height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    /// Set video poster
    pub fn set_poster(&mut self, poster: Option<String>) {
        self.poster = poster;
    }

    /// Set video preload
    pub fn set_preload(&mut self, preload: PreloadType) {
        self.preload = preload;
    }

    /// Set video autoplay
    pub fn set_autoplay(&mut self, autoplay: bool) {
        self.autoplay = autoplay;
    }

    /// Set video loop
    pub fn set_loop(&mut self, loop_: bool) {
        self.loop_ = loop_;
    }

    /// Set video muted
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    /// Set video controls
    pub fn set_controls(&mut self, controls: bool) {
        self.controls = controls;
    }

    /// Set video cross origin
    pub fn set_cross_origin(&mut self, cross_origin: Option<String>) {
        self.cross_origin = cross_origin;
    }

    /// Load video source
    pub fn load_source(&mut self) -> Result<()> {
        // TODO: Implement video source loading
        Ok(())
    }

    /// Get video width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get video height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get video poster
    pub fn poster(&self) -> Option<&str> {
        self.poster.as_deref()
    }

    /// Get video preload
    pub fn preload(&self) -> PreloadType {
        self.preload
    }

    /// Get video autoplay
    pub fn autoplay(&self) -> bool {
        self.autoplay
    }

    /// Get video loop
    pub fn loop_(&self) -> bool {
        self.loop_
    }

    /// Get video muted
    pub fn muted(&self) -> bool {
        self.muted
    }

    /// Get video controls
    pub fn controls(&self) -> bool {
        self.controls
    }

    /// Get video cross origin
    pub fn cross_origin(&self) -> Option<&str> {
        self.cross_origin.as_deref()
    }
}

impl AudioElement {
    /// Create new audio element
    pub fn new(source: String) -> Self {
        Self {
            source,
            preload: PreloadType::Metadata,
            autoplay: false,
            loop_: false,
            muted: false,
            controls: true,
            cross_origin: None,
            media_source: Arc::new(RwLock::new(None)),
            player: Arc::new(RwLock::new(AudioPlayer::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            event_listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set audio preload
    pub fn set_preload(&mut self, preload: PreloadType) {
        self.preload = preload;
    }

    /// Set audio autoplay
    pub fn set_autoplay(&mut self, autoplay: bool) {
        self.autoplay = autoplay;
    }

    /// Set audio loop
    pub fn set_loop(&mut self, loop_: bool) {
        self.loop_ = loop_;
    }

    /// Set audio muted
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    /// Set audio controls
    pub fn set_controls(&mut self, controls: bool) {
        self.controls = controls;
    }

    /// Set audio cross origin
    pub fn set_cross_origin(&mut self, cross_origin: Option<String>) {
        self.cross_origin = cross_origin;
    }

    /// Load audio source
    pub fn load_source(&mut self) -> Result<()> {
        // TODO: Implement audio source loading
        Ok(())
    }

    /// Get audio preload
    pub fn preload(&self) -> PreloadType {
        self.preload
    }

    /// Get audio autoplay
    pub fn autoplay(&self) -> bool {
        self.autoplay
    }

    /// Get audio loop
    pub fn loop_(&self) -> bool {
        self.loop_
    }

    /// Get audio muted
    pub fn muted(&self) -> bool {
        self.muted
    }

    /// Get audio controls
    pub fn controls(&self) -> bool {
        self.controls
    }

    /// Get audio cross origin
    pub fn cross_origin(&self) -> Option<&str> {
        self.cross_origin.as_deref()
    }
}

impl MediaElement for VideoElement {
    fn source(&self) -> &str {
        &self.source
    }

    fn duration(&self) -> f64 {
        self.player.read().state.duration
    }

    fn current_time(&self) -> f64 {
        self.player.read().state.current_time
    }

    fn set_current_time(&mut self, time: f64) -> Result<()> {
        let mut player = self.player.write();
        player.state.current_time = time.max(0.0).min(player.state.duration);
        Ok(())
    }

    fn playback_rate(&self) -> f64 {
        self.player.read().state.playback_rate
    }

    fn set_playback_rate(&mut self, rate: f64) -> Result<()> {
        let mut player = self.player.write();
        player.state.playback_rate = rate.max(0.0);
        Ok(())
    }

    fn volume(&self) -> f32 {
        self.player.read().state.volume
    }

    fn set_volume(&mut self, volume: f32) -> Result<()> {
        let mut player = self.player.write();
        player.state.volume = volume.clamp(0.0, 1.0);
        Ok(())
    }

    fn muted(&self) -> bool {
        self.player.read().state.muted
    }

    fn set_muted(&mut self, muted: bool) -> Result<()> {
        let mut player = self.player.write();
        player.state.muted = muted;
        Ok(())
    }

    fn paused(&self) -> bool {
        self.player.read().state.paused
    }

    fn play(&mut self) -> Result<()> {
        let mut player = self.player.write();
        player.state.paused = false;
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        let mut player = self.player.write();
        player.state.paused = true;
        Ok(())
    }

    fn ready_state(&self) -> ReadyState {
        self.player.read().state.ready_state
    }

    fn network_state(&self) -> NetworkState {
        self.player.read().state.network_state
    }

    fn error(&self) -> Option<MediaError> {
        self.player.read().state.error.clone()
    }

    fn events(&self) -> &Arc<RwLock<Vec<MediaEvent>>> {
        &self.events
    }

    fn add_event_listener(&self, event_type: MediaEventType, callback: Box<dyn Fn(MediaEvent) + Send>) {
        let mut listeners = self.event_listeners.write();
        listeners.entry(event_type)
            .or_insert_with(Vec::new)
            .push(EventCallback {
                id: format!("{:?}_{}", event_type, listeners.len()),
                callback,
            });
    }

    fn remove_event_listener(&self, event_type: MediaEventType, callback_id: String) {
        let mut listeners = self.event_listeners.write();
        if let Some(callbacks) = listeners.get_mut(&event_type) {
            callbacks.retain(|callback| callback.id != callback_id);
        }
    }
}

impl MediaElement for AudioElement {
    fn source(&self) -> &str {
        &self.source
    }

    fn duration(&self) -> f64 {
        self.player.read().state.duration
    }

    fn current_time(&self) -> f64 {
        self.player.read().state.current_time
    }

    fn set_current_time(&mut self, time: f64) -> Result<()> {
        let mut player = self.player.write();
        player.state.current_time = time.max(0.0).min(player.state.duration);
        Ok(())
    }

    fn playback_rate(&self) -> f64 {
        self.player.read().state.playback_rate
    }

    fn set_playback_rate(&mut self, rate: f64) -> Result<()> {
        let mut player = self.player.write();
        player.state.playback_rate = rate.max(0.0);
        Ok(())
    }

    fn volume(&self) -> f32 {
        self.player.read().state.volume
    }

    fn set_volume(&mut self, volume: f32) -> Result<()> {
        let mut player = self.player.write();
        player.state.volume = volume.clamp(0.0, 1.0);
        Ok(())
    }

    fn muted(&self) -> bool {
        self.player.read().state.muted
    }

    fn set_muted(&mut self, muted: bool) -> Result<()> {
        let mut player = self.player.write();
        player.state.muted = muted;
        Ok(())
    }

    fn paused(&self) -> bool {
        self.player.read().state.paused
    }

    fn play(&mut self) -> Result<()> {
        let mut player = self.player.write();
        player.state.paused = false;
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        let mut player = self.player.write();
        player.state.paused = true;
        Ok(())
    }

    fn ready_state(&self) -> ReadyState {
        self.player.read().state.ready_state
    }

    fn network_state(&self) -> NetworkState {
        self.player.read().state.network_state
    }

    fn error(&self) -> Option<MediaError> {
        self.player.read().state.error.clone()
    }

    fn events(&self) -> &Arc<RwLock<Vec<MediaEvent>>> {
        &self.events
    }

    fn add_event_listener(&self, event_type: MediaEventType, callback: Box<dyn Fn(MediaEvent) + Send>) {
        let mut listeners = self.event_listeners.write();
        listeners.entry(event_type)
            .or_insert_with(Vec::new)
            .push(EventCallback {
                id: format!("{:?}_{}", event_type, listeners.len()),
                callback,
            });
    }

    fn remove_event_listener(&self, event_type: MediaEventType, callback_id: String) {
        let mut listeners = self.event_listeners.write();
        if let Some(callbacks) = listeners.get_mut(&event_type) {
            callbacks.retain(|callback| callback.id != callback_id);
        }
    }
}

impl VideoPlayer {
    /// Create new video player
    pub fn new() -> Self {
        Self {
            state: VideoPlayerState::default(),
            decoder: None,
            renderer: None,
            buffer: VideoBuffer::new(100),
            controls: VideoControls::default(),
        }
    }
}

impl AudioPlayer {
    /// Create new audio player
    pub fn new() -> Self {
        Self {
            state: AudioPlayerState::default(),
            decoder: None,
            renderer: None,
            buffer: AudioBuffer::new(1000),
            controls: AudioControls::default(),
        }
    }
}

impl Default for VideoPlayerState {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            duration: 0.0,
            playback_rate: 1.0,
            volume: 1.0,
            muted: false,
            paused: true,
            ready_state: ReadyState::HaveNothing,
            network_state: NetworkState::Empty,
            error: None,
        }
    }
}

impl Default for AudioPlayerState {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            duration: 0.0,
            playback_rate: 1.0,
            volume: 1.0,
            muted: false,
            paused: true,
            ready_state: ReadyState::HaveNothing,
            network_state: NetworkState::Empty,
            error: None,
        }
    }
}

impl VideoBuffer {
    /// Create new video buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
            capacity,
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Add frame to buffer
    pub fn add_frame(&mut self, frame: VideoFrame) -> Result<()> {
        if self.frames.len() >= self.capacity {
            return Err(Error::media("Video buffer full".to_string()));
        }
        
        self.frames.push(frame);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        
        Ok(())
    }

    /// Get frame from buffer
    pub fn get_frame(&mut self) -> Option<VideoFrame> {
        if self.frames.is_empty() {
            return None;
        }
        
        let frame = self.frames.remove(0);
        self.read_pos = (self.read_pos + 1) % self.capacity;
        
        Some(frame)
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.frames.clear();
        self.read_pos = 0;
        self.write_pos = 0;
    }
}

impl AudioBuffer {
    /// Create new audio buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: Vec::with_capacity(capacity),
            capacity,
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Add sample to buffer
    pub fn add_sample(&mut self, sample: AudioSample) -> Result<()> {
        if self.samples.len() >= self.capacity {
            return Err(Error::media("Audio buffer full".to_string()));
        }
        
        self.samples.push(sample);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        
        Ok(())
    }

    /// Get sample from buffer
    pub fn get_sample(&mut self) -> Option<AudioSample> {
        if self.samples.is_empty() {
            return None;
        }
        
        let sample = self.samples.remove(0);
        self.read_pos = (self.read_pos + 1) % self.capacity;
        
        Some(sample)
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.samples.clear();
        self.read_pos = 0;
        self.write_pos = 0;
    }
}

impl Default for VideoControls {
    fn default() -> Self {
        Self {
            visible: true,
            position: VideoControlsPosition::Bottom,
            style: VideoControlsStyle::default(),
            buttons: vec![
                VideoControlButton {
                    button_type: VideoControlButtonType::Play,
                    text: "Play".to_string(),
                    icon: Some("▶".to_string()),
                    enabled: true,
                    visible: true,
                    action: VideoControlAction::Play,
                },
                VideoControlButton {
                    button_type: VideoControlButtonType::Pause,
                    text: "Pause".to_string(),
                    icon: Some("⏸".to_string()),
                    enabled: true,
                    visible: true,
                    action: VideoControlAction::Pause,
                },
                VideoControlButton {
                    button_type: VideoControlButtonType::Volume,
                    text: "Volume".to_string(),
                    icon: Some("🔊".to_string()),
                    enabled: true,
                    visible: true,
                    action: VideoControlAction::None,
                },
                VideoControlButton {
                    button_type: VideoControlButtonType::Fullscreen,
                    text: "Fullscreen".to_string(),
                    icon: Some("⛶".to_string()),
                    enabled: true,
                    visible: true,
                    action: VideoControlAction::ToggleFullscreen,
                },
            ],
        }
    }
}

impl Default for AudioControls {
    fn default() -> Self {
        Self {
            visible: true,
            position: AudioControlsPosition::Bottom,
            style: AudioControlsStyle::default(),
            buttons: vec![
                AudioControlButton {
                    button_type: AudioControlButtonType::Play,
                    text: "Play".to_string(),
                    icon: Some("▶".to_string()),
                    enabled: true,
                    visible: true,
                    action: AudioControlAction::Play,
                },
                AudioControlButton {
                    button_type: AudioControlButtonType::Pause,
                    text: "Pause".to_string(),
                    icon: Some("⏸".to_string()),
                    enabled: true,
                    visible: true,
                    action: AudioControlAction::Pause,
                },
                AudioControlButton {
                    button_type: AudioControlButtonType::Volume,
                    text: "Volume".to_string(),
                    icon: Some("🔊".to_string()),
                    enabled: true,
                    visible: true,
                    action: AudioControlAction::None,
                },
            ],
        }
    }
}

impl Default for VideoControlsStyle {
    fn default() -> Self {
        Self {
            background_color: "#000000".to_string(),
            foreground_color: "#FFFFFF".to_string(),
            border_color: "#333333".to_string(),
            border_width: 1,
            border_radius: 4,
            opacity: 0.8,
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
        }
    }
}

impl Default for AudioControlsStyle {
    fn default() -> Self {
        Self {
            background_color: "#000000".to_string(),
            foreground_color: "#FFFFFF".to_string(),
            border_color: "#333333".to_string(),
            border_width: 1,
            border_radius: 4,
            opacity: 0.8,
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
        }
    }
}
