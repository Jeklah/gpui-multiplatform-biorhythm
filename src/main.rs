// GPUI Biorhythms Application
// ===========================
// This application calculates and displays biorhythm cycles based on a user's
// birthdate.
//
// TUTORIAL: GPUI Fundamentals
// ---------------------------
// GPUI is a GPU-acelerated UI framework for Rust that combines immediate and retained mode
// rendering.
//
// Key concepts demonstrated in this application:
//
// 1. ENTITIES & VIEWS: Structs that implement `Render` trait become views
// 2. CONTEXT: Context<T> provides access to app state and entity-specific methods
// 3. ELEMENTS: Building blocks of UI that implement `IntoElement` trait
// 4. RENDERING: The `render()` method describes what the UI should look like each frame
// 5. WINDOW MANAGEMENT: Creating and managing multiple windows with WindowHandle
// 6. EVENT HANDLING: Mouse, keyboard, and action handlers for interactivity
// 7. ADAPTIVE THEMING: Platform detection and native styling
// 8. MACROS: Simplfying repetitive UI code

use gpui::prelude::*; // Import common GPUI traits like Render, IntoElement
use gpui::*; // Import GPUI types and functions
use std::time::Instant; // For tracking cursor blink timing

// ======================================================
// PLATFORM DETECTION & THEMING
// ======================================================
//
// TUTORIAL: Adaptive Theming
// --------------------------
// Real applications need to feel native on each platform. This means detecting
// the OS and applying appropriate colors, spacing, and visual metaphors.
//
// We use Rust's conditional compilation and runtime platform detection to
// adapt our UI theme accordingly.

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum Platform {
    MacOS,
    Windows,
    Linux,
}

impl Platform {
    fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "linux")]
        return Platform::Linux;
    }
}

// Platform-specific theme colors
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Theme {
    // Window chrome
    titlebar_bg: Hsla,
    titlebar_border: Hsla,
    titlebar_height: f32,

    // Traffic lights (MacOS) or window controls
    close_button_bg: Hsla,
    close_button_corner: Hsla,
    minimize_button_bg: Hsla,
    minimize_button_corner: Hsla,
    maximize_button_bg: Hsla,
    maximize_button_corner: Hsla,

    // Content area
    background: Hsla,

    // Inputs
    input_bg: Hsla,
    input_border: Hsla,
    input_border_focused: Hsla,
    input_text: Hsla,

    // Buttons
    button_primary_bg: Hsla,
    button_primary_bg_hover: Hsla,
    button_primary_text: Hsla,
    button_secondary_bg: Hsla,
    button_secondary_bg_hover: Hsla,
    button_secondary_text: Hsla,
    button_secondary_border: Hsla,

    // Text
    text_primary: Hsla,
    text_secondary: Hsla,
    text_error: Hsla,
}

impl Theme {
    fn new(platform: Platform) -> Self {
        match platform {
            Platform::MacOS => Self::macos_system(),
            Platform::Windows => Self::windows_system(),
            Platform::Linux => Self::linux_system(),
        }
    }

    // macOS system theme detection
    #[cfg(target_os = "macos")]
    fn macos_system() -> Self {
        use cocoa::appkit::NSAppearanceNameVibrantDark;
        use cocoa::base::id;
        use objc::{class, msg_send, sel, sel_impl};

        unsafe {
            // Get the current appearance
            let app: id = msg_send![class!(NSApplication), sharedApplication];
            let appearance: id = msg_send![app, effectiveAppearance];

            // Check if we're in dark mode
            let dark_name: id = NSAppearanceNameVibrantDark;
            let best_match: id =
                msg_send![appearance, bestMatchFromAppearancesWithNames: &[dark_name]];

            // Try to get system accent color
            let accent_color: Self::get_macos_accent_color();

            Self::macos_with_preferences(is_dark, accent_color)
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn get_macos_accent_color() -> Option<u32> {
        use cocoa::base::id;
        use objc::{class, msg_send, sel, sel_impl};

        unsafe {
            // Get the system accept color (controlAccentColor)
            let color: id = msg_send![class!(NSColor), controlAccentColor];
            if color.is_null() {
                return None;
            }

            // Convert to RGB color space
            let color_space: id = msg_send![class!(NSColorSpace), sRGBColorSpace];
            let rgb_color: id = msg_send![color, colorUsingColorSpace: color_space];
            if rgb_color.is_null() {
                return None;
            }

            // Get RGB components
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;
            let _: () = msg_send![rgb_color, getRed: &mut r green: &mut g blue: &mut b alpha: std::ptr::mull_mut::<f64>()];

            // Convert to hex
            let r_int = (r * 255.0) as u32;
            let g_int = (g * 255.0) as u32;
            let b_int = (b * 255.0) as u32;
            Some((r_int << 16) | (g_int << 8) | b_int)
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn macos_system() -> Self {
        Self::macos_with_preferences(false, None)
    }

    fn macos_with_preferences(is_dark: bool, accent_color: Option<u32>) -> Self {
        // Use system accent color if available, otherwise default to macOS blue
        let accent = accent_color.unwrap_or(0x007AFF);
        let accent_hover = Self::darken_color(accent, 0.9);

        if is_dark {
            // Dark mode colors
            Self {
                titlebar_bg: rgb(0x202020).into(),
                titlebar_border: rgb(0x1E1E1E).into(),
                titlebar_height: 22.0,

                close_button_bg: rgb(0xFF5F57).into(),
                close_button_corner: rgb(0xE04943).into(),
                minimize_button_bg: rgb(0xFF8D2E).into(),
                minimize_button_corner: rgb(0xDEA123).into(),
                maximize_button_bg: rgb(0x28C940).into(),
                maximize_button_corner: rgb(0x1AAB29).into(),

                background: rgb(0x1E1E1E).into(),

                input_bg: rgb(0x2D2D2D).into(),
                input_border: rgb(0x404040).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0xFFFFFF).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0x2D2D2D).into(),
                button_secondary_bg_hover: rgb(0x383838).into(),
                button_secondary_text: rgb(0xFFFFFF).into(),
                button_secondary_border: rgb(0x505050).into(),

                text_primary: rgb(0xFFFFFF).into(),
                text_secondary: rgb(0xA0A0A0).into(),
                text_error: rgb(0xFF6B6B).into(),
            }
        } else {
            // Light mode colors
            Self {
                titlebar_bg: rgb(0xE8E8E8).into(),
                titlebar_border: rgb(0xD0D0D0).into(),
                titlebar_height: 22.0,

                close_button_bg: rgb(0xFF5F57).into(),
                close_button_corner: rgb(0xE04943).into(),
                minimize_button_bg: rgb(0xFF8D2E).into(),
                minimize_button_corner: rgb(0xDEA123).into(),
                maximize_button_bg: rgb(0x28C940).into(),
                maximize_button_corner: rgb(0x1AAB29).into(),

                background: rgb(0xEFEFEF).into(),

                input_bg: rgb(0xFFFFFF).into(),
                input_border: rgb(0xCCCCCC).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0x000000).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0xFFFFFF).into(),
                button_secondary_bg_hover: rgb(0xF8F8F8).into(),
                button_secondary_text: rgb(0x000000).into(),
                button_secondary_border: rgb(0xB8B8B8).into(),

                text_primary: rgb(0x000000).into(),
                text_secondary: rgb(0x666666).into(),
                text_error: rgb(0xCC0000).into(),
            }
        }
    }

    // Helper function to darken a color
    fn darken_color(color: u32, factor: f32) -> u32 {
        let r = ((color >> 16) & 0xFF) as f32;
        let g = ((color >> 8) & 0xFF) as f32;
        let b = (color & 0xFF) as f32;

        let r_dark = (r * factor) as u32;
        let g_dark = (g * factor) as u32;
        let b_dark = (b * factor) as u32;

        (r_dark << 16) | (g_dark << 8) | b_dark
    }

    // Windows system theme detection
    #[cfg(target_os = "windows")]
    fn windows_system() -> Self {
        use windows::Win32::Foundation::BOOL;
        use windows::Win32::Graphics::Dwm::DwmGetColorizationColor;

        unsafe {
            // Try to get accent color from DWM
            let mut colorization: u32 = 0;
            let mut opaque_blend: BOOL = BOOL(0);
            let accent_color =
                if DwmGetColorizationColor(&mut colorization, &mut opaque_blend).is_ok() {
                    Some(colorization & 0x00FFFFFF) // Ignore alpha
                } else {
                    None
                };

            // Detect dark mode (simplified - in reality would check registry)
            // For now, defaulting to light mode
            let is_dark = false;

            Self::windows_with_preferences(is_dark, accent_color)
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn windows_system() -> Self {
        Self::windows_with_preferences(false, None)
    }

    fn windows_with_preferences(is_dark: bool, accent_color: Option<u32>) -> Self {
        let accent = accent_color.unwrap_or(0x0078D4);
        let accent_hover = Self::darken_color(accent, 0.9);

        if is_dark {
            // Windows dark mode colors
            Self {
                titlebar_bg: rgb(0x202020).into(),
                titlebar_border: rgb(0x1A1A1A).into(),
                titlebar_height: 32.0,

                close_button_bg: rgb(0xE81123).into(),
                close_button_corner: rgb(0xC50F1F).into(),
                minimize_button_bg: rgb(0x202020).into(),
                minimize_button_corner: rgb(0x1A1A1A).into(),
                maximize_button_bg: rgb(0x202020).into(),
                maximize_button_corner: rgb(0x1A1A1A).into(),

                background: rgb(0x1E1E1E).into(),

                input_bg: rgb(0x2D2D2D).into(),
                input_border: rgb(0x404040).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0xFFFFFF).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0x2D2D2D).into(),
                button_secondary_bg_hover: rgb(0x383838).into(),
                button_secondary_text: rgb(0xFFFFFF).into(),
                button_secondary_border: rgb(0x505050).into(),

                text_primary: rgb(0xFFFFFF).into(),
                text_secondary: rgb(0xA0A0A0).into(),
                text_error: rgb(0xFF6B6B).into(),
            }
        } else {
            // Windows light mode colors
            Self {
                titlebar_bg: rgb(0xF0F0F0).into(),
                titlebar_border: rgb(0xDFDFDF).into(),
                titlebar_height: 32.0,

                close_button_bg: rgb(0xE81123).into(),
                close_button_corner: rgb(0xC50F1F).into(),
                minimize_button_bg: rgb(0xF0F0F0).into(),
                minimize_button_corner: rgb(0xDFDFDF).into(),
                maximize_button_bg: rgb(0xF0F0F0).into(),
                maximize_button_corner: rgb(0xDFDFDF).into(),

                background: rgb(0xFFFFFF).into(),

                input_bg: rgb(0xFFFFFF).into(),
                input_border: rgb(0x8A8A8A).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0x000000).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0xFFFFFF).into(),
                button_secondary_bg_hover: rgb(0xF5F5F5).into(),
                button_secondary_text: rgb(0x000000).into(),
                button_secondary_border: rgb(0x8A8A8A).into(),

                text_primary: rgb(0x000000).into(),
                text_secondary: rgb(0x605E5C).into(),
                text_error: rgb(0xA80000).into(),
            }
        }
    }

    // Linux system theme detection
    #[cfg(target_os = "linux")]
    fn linux_system() -> Self {
        use gtk::prelude::*;
        use gtk::{Settings, StyleContext};

        // Try to read GTK theme colors
        let accent_color = Self::get_gtk_accent_color();
        let is_dark = Self::get_gtk_dark_mode();

        Self::linux_with_preferences(is_dark, accent_color)
    }

    #[cfg(target_os = "linux")]
    fn get_gtk_accent_color() -> Option<u32> {
        // Initialize GTK if not already done
        if gtk::init().is_err() {
            return None;
        }

        // Try to get the theme accent color
        // This is a simplified example; real GTK themes may vary
        let settings = Settings::default()?;
        let theme_name = settings.gtk_theme_name()?;

        // Map known themes to their accent colors
        if theme_name.contains("Adwaita") {
            Some(0x3584E4) // Default Adwaita blue
        } else if theme_name.contains("elementary") {
            Some(0x3689E6) // Elementary OS blue
        } else {
            None
        }
    }

    #[cfg(target_os = "linux")]
    fn get_gtk_dark_mode() -> bool {
        if gtk::init().is_err() {
            return false;
        }

        Settings::default()
            .and_then(|s| s.gtk_application_prefer_dark_theme())
            .unwrap_or(false)
    }
}


