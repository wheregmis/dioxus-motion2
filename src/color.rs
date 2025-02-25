//! Color module for animation support
//!
//! Provides RGBA color representation and animation interpolation.
//! Supports both normalized (0.0-1.0) and byte (0-255) color values.

use crate::animatable::Animatable;

/// Represents an RGBA color with normalized components
///
/// Each component (r,g,b,a) is stored as a float between 0.0 and 1.0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    /// Red component (0.0-1.0)
    pub r: f32,
    /// Green component (0.0-1.0)
    pub g: f32,
    /// Blue component (0.0-1.0)
    pub b: f32,
    /// Alpha component (0.0-1.0)
    pub a: f32,
}

impl Color {
    /// Creates a new color with normalized components
    ///
    /// # Examples
    /// ```
    /// let orange = Color::new(1.0, 0.5, 0.0, 1.0);
    /// ```
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Creates a color from 8-bit RGBA values
    ///
    /// # Examples
    /// ```
    /// let orange = Color::from_rgba(255, 128, 0, 255);
    /// ```
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Creates a color from a hex code string
    ///
    /// Supports formats:
    /// - "#RGB"
    /// - "#RGBA"
    /// - "#RRGGBB"
    /// - "#RRGGBBAA"
    ///
    /// # Examples
    /// ```
    /// let orange = Color::from_hex("#FF8000").unwrap();
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // #RGB format
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "Invalid hex color")?;

                // Expand from 4-bit to 8-bit (e.g., 0xA becomes 0xAA)
                Ok(Self::from_rgba(r * 17, g * 17, b * 17, 255))
            }
            4 => {
                // #RGBA format
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "Invalid hex color")?;
                let a = u8::from_str_radix(&hex[3..4], 16).map_err(|_| "Invalid hex color")?;

                Ok(Self::from_rgba(r * 17, g * 17, b * 17, a * 17))
            }
            6 => {
                // #RRGGBB format
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;

                Ok(Self::from_rgba(r, g, b, 255))
            }
            8 => {
                // #RRGGBBAA format
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid hex color")?;

                Ok(Self::from_rgba(r, g, b, a))
            }
            _ => Err("Invalid hex color format"),
        }
    }

    /// Converts color to 8-bit RGBA values
    ///
    /// # Returns
    /// Tuple of (r,g,b,a) with values from 0-255
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    /// Converts color to a CSS-compatible rgba string
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Color::new(1.0, 0.5, 0.0, 0.8).to_css_string(), "rgba(255, 128, 0, 0.8)");
    /// ```
    pub fn to_css_string(&self) -> String {
        let (r, g, b, _) = self.to_rgba();
        format!("rgba({}, {}, {}, {})", r, g, b, self.a)
    }

    /// Converts color to a CSS-compatible hex string
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Color::new(1.0, 0.5, 0.0, 1.0).to_hex_string(), "#ff8000");
    /// ```
    pub fn to_hex_string(&self) -> String {
        let (r, g, b, a) = self.to_rgba();

        if a == 255 {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
        }
    }

    /// Pre-defined color: transparent black
    pub fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Pre-defined color: black
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Pre-defined color: white
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Pre-defined color: red
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Pre-defined color: green
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    /// Pre-defined color: blue
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// Pre-defined color: yellow
    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0, 1.0)
    }

    /// Pre-defined color: cyan
    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0, 1.0)
    }

    /// Pre-defined color: magenta
    pub fn magenta() -> Self {
        Self::new(1.0, 0.0, 1.0, 1.0)
    }

    /// Pre-defined color: gray (50%)
    pub fn gray() -> Self {
        Self::new(0.5, 0.5, 0.5, 1.0)
    }
}

/// Implementation of animation interpolation for Color
impl Animatable for Color {
    /// Creates a fully transparent black color
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Minimum difference between color components
    fn epsilon() -> f32 {
        0.001
    }

    /// Calculates color vector magnitude
    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b + self.a * self.a).sqrt()
    }

    /// Scales color components by a factor
    fn scale(&self, factor: f32) -> Self {
        Self::new(
            (self.r * factor).clamp(0.0, 1.0),
            (self.g * factor).clamp(0.0, 1.0),
            (self.b * factor).clamp(0.0, 1.0),
            (self.a * factor).clamp(0.0, 1.0),
        )
    }

    /// Adds two colors component-wise
    fn add(&self, other: &Self) -> Self {
        Self::new(
            (self.r + other.r).clamp(0.0, 1.0),
            (self.g + other.g).clamp(0.0, 1.0),
            (self.b + other.b).clamp(0.0, 1.0),
            (self.a + other.a).clamp(0.0, 1.0),
        )
    }

    /// Subtracts two colors component-wise
    fn sub(&self, other: &Self) -> Self {
        Self::new(
            (self.r - other.r).clamp(0.0, 1.0),
            (self.g - other.g).clamp(0.0, 1.0),
            (self.b - other.b).clamp(0.0, 1.0),
            (self.a - other.a).clamp(0.0, 1.0),
        )
    }

    /// Linearly interpolates between two colors
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        Self::new(
            self.r * (1.0 - t) + target.r * t,
            self.g * (1.0 - t) + target.g * t,
            self.b * (1.0 - t) + target.b * t,
            self.a * (1.0 - t) + target.a * t,
        )
    }
}
