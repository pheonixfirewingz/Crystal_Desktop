//! Window management system optimized for CPU performance.
//!
//! This module provides a lightweight, CPU-efficient window implementation that uses
//! bitwise operations for state management and minimal memory operations for core
//! functionality.
//!
//! # Performance Characteristics
//!
//! - Uses bitwise operations instead of booleans for state flags
//! - Minimizes memory operations and allocations
//! - All methods are inlined for reduced function call overhead
//! - State changes are atomic and efficient
//!
//! # Example
//!
//! ```rust
//! use window::{Window, Rect};
//!
//! // Create a new window with title
//! let mut window = Window::new_titled(
//!     "My Window".to_string(),
//!     Rect::new(100, 100, 800, 600)
//! );
//!
//! // Manipulate window state
//! window.maximize();
//! assert!(window.is_maximized());
//!
//! window.set_active(true);
//! assert!(window.is_active());
//!
//! // Restore to original size/position
//! window.restore();
//! ```

use crate::common::ScreenSize;
use crate::render::api::texture::Texture;
use crate::render::util::rect::{Position, Rect, Size};

/// Represents a window in the windowing system.
///
/// The `Window` struct is designed for CPU-efficient state management using
/// bitwise operations and minimal memory footprint. It maintains the window's
/// geometric properties, state flags, and appearance attributes.
///
/// # State Management
///
/// Window state is managed through bit flags for efficiency:
/// - Title bar visibility
/// - Maximized state
/// - Minimized state
/// - Active (focused) state
/// - Icon presence
///
/// # Memory Layout
///
/// The struct is organized to minimize memory operations:
/// - Core data (rect, flags) is placed first for frequent access
/// - Less frequently accessed data (icon, title) is placed last
/// - Restoration data is maintained for maximize/minimize operations
pub struct Window {
    /// Current geometric properties of the window
    rect: Rect,
    /// Bit flags for window state management
    /// - Bit 0: Title bar visibility
    /// - Bit 1: Maximized state
    /// - Bit 2: Minimized state
    /// - Bit 3: Active (focused) state
    /// - Bit 4: Has icon
    flags: u8,

    /// Stores the window's geometry before maximize/minimize
    restore_rect: Option<Rect>,

    /// Window icon texture
    icon: Option<Texture>,
    /// Optional window title
    title: Option<String>,
}

// Bit flag constants for window state
const TITLE_BAR_FLAG: u8    = 0b0000_0001;
const MAXIMIZED_FLAG: u8    = 0b0000_0010;
const MINIMIZED_FLAG: u8    = 0b0000_0100;
const ACTIVE_FLAG: u8       = 0b0000_1000;
const HAS_ICON_FLAG: u8     = 0b0001_0000;

/// Padding used for window layout calculations
pub const WINDOW_PADDING: ScreenSize = 3;

impl Window {
    /// Creates a new window with a title bar and specified title.
    ///
    /// # Arguments
    ///
    /// * `title` - The window's title text
    /// * `rect` - Initial position and size of the window
    ///
    /// # Examples
    ///
    /// ```rust
    /// let window = Window::new_titled(
    ///     "Main Window".to_string(),
    ///     Rect::new(0, 0, 800, 600)
    /// );
    /// ```
    #[inline]
    pub fn new_titled(title: String, rect: Rect) -> Self {
        Self {
            rect,
            flags: TITLE_BAR_FLAG,  // Initialize with title bar visible
            restore_rect: None,
            icon: None,
            title: Some(title),
        }
    }

    /// Creates a new window without a title bar.
    ///
    /// # Arguments
    ///
    /// * `rect` - Initial position and size of the window
    ///
    /// # Examples
    ///
    /// ```rust
    /// let window = Window::new_non_titled(Rect::new(0, 0, 800, 600));
    /// ```
    #[inline]
    pub fn new_non_titled(rect: Rect) -> Self {
        Self {
            rect,
            flags: 0,
            restore_rect: None,
            icon: None,
            title: None,
        }
    }

    /// Moves the window to a new position if not maximized.
    ///
    /// # Arguments
    ///
    /// * `x` - New x-coordinate
    /// * `y` - New y-coordinate
    ///
    /// # Notes
    ///
    /// This operation is ignored if the window is maximized.
    #[inline]
    pub fn move_window(&mut self, x: ScreenSize, y: ScreenSize) {
        if !self.is_maximized() {
            self.rect.set_pos(x, y);
        }
    }

    /// Resizes the window if neither maximized nor minimized.
    ///
    /// # Arguments
    ///
    /// * `width` - New width
    /// * `height` - New height
    ///
    /// # Notes
    ///
    /// This operation is ignored if the window is maximized or minimized.
    #[inline]
    pub fn resize_window(&mut self, width: ScreenSize, height: ScreenSize) {
        if !self.is_maximized() && !self.is_minimized() {
            self.rect.set_size(width, height);
        }
    }

    /// Maximizes the window, storing current geometry for later restoration.
    ///
    /// # State Changes
    ///
    /// - Sets maximized flag
    /// - Clears minimized flag
    /// - Stores current geometry in restore_rect
    ///
    /// # Notes
    ///
    /// Multiple calls to maximize() while already maximized have no effect.
    #[inline]
    pub fn maximize(&mut self) {
        if !self.is_maximized() {
            self.restore_rect = Some(self.rect.clone());
            self.flags |= MAXIMIZED_FLAG;
            self.flags &= !MINIMIZED_FLAG;
        }
    }

    /// Minimizes the window, storing current geometry for later restoration.
    ///
    /// # State Changes
    ///
    /// - Sets minimized flag
    /// - Clears maximized flag
    /// - Stores current geometry in restore_rect if not already stored
    ///
    /// # Notes
    ///
    /// Multiple calls to minimize() while already minimized have no effect.
    #[inline]
    pub fn minimize(&mut self) {
        if !self.is_minimized() {
            if self.restore_rect.is_none() {
                self.restore_rect = Some(self.rect.clone());
            }
            self.flags |= MINIMIZED_FLAG;
            self.flags &= !MAXIMIZED_FLAG;
        }
    }

    /// Restores the window to its geometry before maximization or minimization.
    ///
    /// # State Changes
    ///
    /// - Clears both maximized and minimized flags
    /// - Restores previous geometry if available
    /// - Consumes the stored restore_rect
    ///
    /// # Notes
    ///
    /// Has no effect if there's no stored geometry to restore to.
    #[inline]
    pub fn restore(&mut self) {
        if let Some(stored_rect) = self.restore_rect.take() {
            self.rect = stored_rect;
            self.flags &= !(MAXIMIZED_FLAG | MINIMIZED_FLAG);
        }
    }

    /// Sets the window's active (focused) state.
    ///
    /// # Arguments
    ///
    /// * `active` - true to set as active, false to deactivate
    #[inline]
    pub fn set_active(&mut self, active: bool) {
        if active {
            self.flags |= ACTIVE_FLAG;
        } else {
            self.flags &= !ACTIVE_FLAG;
        }
    }

    /// Sets the window's icon.
    ///
    /// # Arguments
    ///
    /// * `icon` - The texture to use as the window icon
    ///
    /// # Examples
    ///
    /// ```rust
    /// window.set_icon(some_texture);
    /// assert!(window.has_icon());
    /// ```
    #[inline]
    pub fn set_icon(&mut self, icon: Texture) {
        self.icon = Some(icon);
        self.flags |= HAS_ICON_FLAG;
    }

    /// Removes the window's icon if it exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// window.remove_icon();
    /// assert!(!window.has_icon());
    /// ```
    #[inline]
    pub fn remove_icon(&mut self) {
        self.icon = None;
        self.flags &= !HAS_ICON_FLAG;
    }

    /// Returns a reference to the window's icon if it exists.
    ///
    /// # Returns
    ///
    /// * `Option<&Texture>` - Reference to the window's icon texture if present
    #[inline]
    pub fn get_icon(&self) -> Option<&Texture> {
        self.icon.as_ref()
    }

    /// Checks if the window has an icon.
    ///
    /// # Returns
    ///
    /// * `bool` - true if the window has an icon
    ///
    /// # Notes
    ///
    /// Uses efficient bitwise operations for state checking.
    #[inline]
    pub fn has_icon(&self) -> bool {
        (self.flags & HAS_ICON_FLAG) != 0
    }

    /// Toggles the visibility of the window's title bar.
    ///
    /// Uses efficient bitwise XOR operation for toggle.
    #[inline]
    pub fn toggle_title_bar(&mut self) {
        self.flags ^= TITLE_BAR_FLAG;
    }

    /// Updates the window's title text.
    ///
    /// # Arguments
    ///
    /// * `title` - New title text, or None to remove title
    #[inline]
    pub fn update_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Checks if the window has a title bar.
    ///
    /// # Returns
    ///
    /// * `bool` - true if title bar should be drawn
    #[inline]
    pub fn has_title_bar(&self) -> bool {
        (self.flags & TITLE_BAR_FLAG) != 0
    }

    /// Checks if the window is currently maximized.
    ///
    /// # Returns
    ///
    /// * `bool` - true if window is maximized
    #[inline]
    pub fn is_maximized(&self) -> bool {
        (self.flags & MAXIMIZED_FLAG) != 0
    }

    /// Checks if the window is currently minimized.
    ///
    /// # Returns
    ///
    /// * `bool` - true if window is minimized
    #[inline]
    pub fn is_minimized(&self) -> bool {
        (self.flags & MINIMIZED_FLAG) != 0
    }

    /// Checks if the window is currently active (focused).
    ///
    /// # Returns
    ///
    /// * `bool` - true if window is active
    #[inline]
    pub fn is_active(&self) -> bool {
        (self.flags & ACTIVE_FLAG) != 0
    }

    /// Returns the current size of the window.
    ///
    /// # Returns
    ///
    /// * `(ScreenSize, ScreenSize)` - Size
    #[inline]
    pub fn get_size(&self) -> Size {
        self.rect.size
    }

    /// Returns the current position of the window.
    ///
    /// # Returns
    ///
    /// * `(ScreenSize, ScreenSize)` - Position
    #[inline]
    pub fn get_position(&self) -> Position {
        self.rect.position
    }

    /// Returns a reference to the window's title.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - Reference to the optional title text
    #[inline]
    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }

    /// Returns a mutable reference to the window's render rectangle.
    ///
    /// # Safety
    ///
    /// Care should be taken when modifying the render rectangle directly,
    /// as it may not maintain proper state consistency with window flags.
    /// Prefer using move_window() and resize_window() when possible.
    #[inline]
    pub fn get_mut_render_rect(&mut self) -> &mut Rect {
        &mut self.rect
    }

    /// Returns a reference to the window's render rectangle.
    ///
    /// # Returns
    ///
    /// * `&Rect` - Reference to the window's geometric properties
    #[inline]
    pub fn get_render_rect(&self) -> &Rect {
        &self.rect
    }
}