//! Color module for animation support
//!
//! Provides RGBA color representation and animation interpolation.
//! Supports both normalized (0.0-1.0) and byte (0-255) color values.

use crate::Animatable;

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
    /// use dioxus_motion2::Color;
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
    /// use dioxus_motion2::Color;
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
    /// use dioxus_motion2::Color;
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
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        )
    }

    /// Converts color to a CSS-compatible rgba string
    ///
    /// # Examples
    /// ```
    /// use dioxus_motion2::Color;
    /// assert_eq!(Color::new(1.0, 0.5, 0.0, 0.8).to_css_string(), "rgba(255, 128, 0, 0.8)");
    /// ```
    pub fn to_css_string(&self) -> String {
        let (r, g, b, _) = self.to_rgba();
        // Round the alpha to handle floating point precision
        let alpha = (self.a * 100.0).round() / 100.0;
        format!("rgba({}, {}, {}, {})", r, g, b, alpha)
    }

    /// Converts color to a CSS-compatible hex string
    ///
    /// # Examples
    /// ```
    /// use dioxus_motion2::Color;
    /// assert_eq!(Color::new(1.0, 0.5, 0.0, 1.0).to_hex_string(), "#ff8000");
    /// ```
    pub fn to_hex_string(&self) -> String {
        let (r, g, b, a) = self.to_rgba();

        if a == 255 {
            // Use exact values without rounding
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(0.5, 0.7, 0.3, 1.0);
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.7);
        assert_eq!(color.b, 0.3);
        assert_eq!(color.a, 1.0);

        // Test clamping
        let color = Color::new(1.5, -0.7, 2.0, -0.5);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 1.0);
        assert_eq!(color.a, 0.0);
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::from_rgba(128, 255, 0, 192);
        assert!((color.r - 0.502).abs() < 0.001);
        assert_eq!(color.g, 1.0);
        assert_eq!(color.b, 0.0);
        assert!((color.a - 0.753).abs() < 0.001);
    }

    #[test]
    fn test_color_from_hex() {
        // Test #RGB format
        let color = Color::from_hex("#F80").expect("Invalid hex color");
        assert_eq!(color.to_rgba(), (255, 136, 0, 255));

        // Test #RGBA format
        let color = Color::from_hex("#F80C").expect("Invalid hex color");
        assert_eq!(color.to_rgba(), (255, 136, 0, 204));

        // Test #RRGGBB format
        let color = Color::from_hex("#FF8000").expect("Invalid hex color");
        assert_eq!(color.to_rgba(), (255, 128, 0, 255));

        // Test #RRGGBBAA format
        let color = Color::from_hex("#FF8000CC").expect("Invalid hex color");
        assert_eq!(color.to_rgba(), (255, 128, 0, 204));

        // Test invalid formats
        assert!(Color::from_hex("invalid").is_err());
        assert!(Color::from_hex("#FF").is_err());
        assert!(Color::from_hex("#FFGG00").is_err());
    }

    #[test]
    fn test_color_to_css_string() {
        let color = Color::from_rgba(255, 128, 0, 204);
        assert_eq!(color.to_css_string(), "rgba(255, 128, 0, 0.8)");

        let color = Color::new(1.0, 0.5, 0.0, 1.0);
        assert_eq!(color.to_css_string(), "rgba(255, 128, 0, 1)");
    }

    #[test]
    fn test_color_to_hex_string() {
        let color = Color::from_rgba(255, 128, 0, 255);
        assert_eq!(color.to_hex_string(), "#ff8000");

        let color = Color::from_rgba(255, 128, 0, 204);
        assert_eq!(color.to_hex_string(), "#ff8000cc");
    }

    #[test]
    fn test_predefined_colors() {
        assert_eq!(Color::transparent(), Color::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::red(), Color::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::green(), Color::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(Color::blue(), Color::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(Color::yellow(), Color::new(1.0, 1.0, 0.0, 1.0));
        assert_eq!(Color::cyan(), Color::new(0.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::magenta(), Color::new(1.0, 0.0, 1.0, 1.0));
        assert_eq!(Color::gray(), Color::new(0.5, 0.5, 0.5, 1.0));
    }

    #[test]
    fn test_animatable_implementation() {
        let color1 = Color::new(1.0, 0.0, 0.0, 1.0);
        let color2 = Color::new(0.0, 1.0, 0.0, 1.0);

        // Test zero
        assert_eq!(Color::zero(), Color::transparent());

        // Test magnitude
        assert!((color1.magnitude() - std::f32::consts::SQRT_2).abs() < 0.0001);

        // Test scale
        let scaled = color1.scale(0.5);
        assert_eq!(scaled, Color::new(0.5, 0.0, 0.0, 0.5));

        // Test add
        let sum = color1.add(&color2);
        assert_eq!(sum, Color::new(1.0, 1.0, 0.0, 1.0));

        // Test sub
        let diff = color1.sub(&color2);
        assert_eq!(diff, Color::new(1.0, 0.0, 0.0, 0.0));

        // Test interpolate
        let mid = color1.interpolate(&color2, 0.5);
        assert_eq!(mid, Color::new(0.5, 0.5, 0.0, 1.0));
    }

    #[test]
    fn test_color_edge_cases() {
        // Test extreme values
        let color = Color::new(f32::MAX, f32::MIN, f32::INFINITY, f32::NEG_INFINITY);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 1.0);
        assert_eq!(color.a, 0.0);

        // Test NaN values
        let color = Color::new(f32::NAN, f32::NAN, f32::NAN, f32::NAN);
        assert!(color.r.is_nan());
        assert!(color.g.is_nan());
        assert!(color.b.is_nan());
        assert!(color.a.is_nan());
    }

    #[test]
    fn test_color_interpolation_edge_cases() {
        let color1 = Color::new(0.0, 0.0, 0.0, 1.0);
        let color2 = Color::new(1.0, 1.0, 1.0, 0.0);

        // Test interpolation with t = 0
        let result = color1.interpolate(&color2, 0.0);
        assert_eq!(result, color1);

        // Test interpolation with t = 1
        let result = color1.interpolate(&color2, 1.0);
        assert_eq!(result, color2);

        // Test interpolation with t outside [0,1]
        let result = color1.interpolate(&color2, -0.5);
        assert_eq!(result, color1.interpolate(&color2, 0.0));

        let result = color1.interpolate(&color2, 1.5);
        assert_eq!(result, color1.interpolate(&color2, 1.0));
    }

    #[test]
    fn test_color_arithmetic() {
        let color1 = Color::new(0.5, 0.3, 0.2, 1.0);
        let color2 = Color::new(0.1, 0.2, 0.3, 0.5);

        // Test addition
        let sum = color1.add(&color2);
        assert!((sum.r - 0.6).abs() < f32::EPSILON);
        assert!((sum.g - 0.5).abs() < f32::EPSILON);
        assert!((sum.b - 0.5).abs() < f32::EPSILON);
        assert!((sum.a - 1.0).abs() < f32::EPSILON);

        // Test subtraction
        let diff = color1.sub(&color2);
        assert!((diff.r - 0.4).abs() < f32::EPSILON);
        assert!((diff.g - 0.1).abs() < f32::EPSILON);
        assert!((diff.b - 0.0).abs() < f32::EPSILON);
        assert!((diff.a - 0.5).abs() < f32::EPSILON);

        // Test scaling
        let scaled = color1.scale(0.5);
        assert!((scaled.r - 0.25).abs() < f32::EPSILON);
        assert!((scaled.g - 0.15).abs() < f32::EPSILON);
        assert!((scaled.b - 0.1).abs() < f32::EPSILON);
        assert!((scaled.a - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_color_conversion_precision() {
        // Test precision of conversions between normalized and byte values
        let original = Color::from_rgba(127, 63, 31, 255);
        let rgba = original.to_rgba();
        let converted = Color::from_rgba(rgba.0, rgba.1, rgba.2, rgba.3);

        assert!((original.r - converted.r).abs() < 0.01);
        assert!((original.g - converted.g).abs() < 0.01);
        assert!((original.b - converted.b).abs() < 0.01);
        assert!((original.a - converted.a).abs() < 0.01);
    }

    #[test]
    fn test_color_css_string_formatting() {
        // Test various alpha values
        let color = Color::new(1.0, 0.5, 0.0, 1.0);
        assert_eq!(color.to_css_string(), "rgba(255, 128, 0, 1)");

        let color = Color::new(1.0, 0.5, 0.0, 0.0);
        assert_eq!(color.to_css_string(), "rgba(255, 128, 0, 0)");

        let color = Color::new(1.0, 0.5, 0.0, 0.5);
        assert_eq!(color.to_css_string(), "rgba(255, 128, 0, 0.5)");
    }

    #[test]
    fn test_color_hex_string_formatting() {
        // Test various color combinations
        let color = Color::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(color.to_hex_string(), "#ffffff");

        let color = Color::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(color.to_hex_string(), "#000000");

        let color = Color::new(1.0, 0.0, 0.0, 0.5);
        assert_eq!(color.to_hex_string(), "#ff000080");
    }
}
