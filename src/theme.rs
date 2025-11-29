use iced::theme::{self, Theme};
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct ColorScheme {
    // Primary colors
    pub cosmic_blue: Color,
    pub cosmic_purple: Color,
    pub cosmic_accent: Color,
    pub cosmic_accent_dark: Color,
    
    // Background colors
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub bg_hover: Color,
    
    // Surface colors
    pub surface: Color,
    pub surface_hover: Color,
    pub surface_active: Color,
    pub surface_elevated: Color,
    
    // Text colors
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_disabled: Color,
    
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    
    // Border colors
    pub border: Color,
    pub border_light: Color,
    pub border_focus: Color,
}

impl ColorScheme {
    pub fn dark() -> Self {
        Self {
            cosmic_blue: Color::from_rgb(0.25, 0.55, 0.95),
            cosmic_purple: Color::from_rgb(0.6, 0.35, 0.95),
            cosmic_accent: Color::from_rgb(0.45, 0.65, 1.0),
            cosmic_accent_dark: Color::from_rgb(0.35, 0.55, 0.9),
            bg_primary: Color::from_rgb(0.09, 0.09, 0.10),
            bg_secondary: Color::from_rgb(0.13, 0.13, 0.15),
            bg_tertiary: Color::from_rgb(0.17, 0.17, 0.19),
            bg_hover: Color::from_rgb(0.20, 0.20, 0.22),
            surface: Color::from_rgb(0.12, 0.12, 0.14),
            surface_hover: Color::from_rgb(0.16, 0.16, 0.18),
            surface_active: Color::from_rgb(0.20, 0.20, 0.22),
            surface_elevated: Color::from_rgb(0.14, 0.14, 0.16),
            text_primary: Color::from_rgb(0.95, 0.95, 0.97),
            text_secondary: Color::from_rgb(0.7, 0.7, 0.75),
            text_disabled: Color::from_rgb(0.5, 0.5, 0.55),
            success: Color::from_rgb(0.2, 0.8, 0.4),
            warning: Color::from_rgb(1.0, 0.7, 0.2),
            error: Color::from_rgb(0.95, 0.3, 0.3),
            info: Color::from_rgb(0.3, 0.6, 0.95),
            border: Color::from_rgb(0.25, 0.25, 0.30),
            border_light: Color::from_rgb(0.20, 0.20, 0.25),
            border_focus: Color::from_rgb(0.45, 0.65, 1.0),
        }
    }
    
    pub fn light() -> Self {
        Self {
            cosmic_blue: Color::from_rgb(0.2, 0.5, 0.9),
            cosmic_purple: Color::from_rgb(0.55, 0.3, 0.9),
            cosmic_accent: Color::from_rgb(0.4, 0.6, 1.0),
            cosmic_accent_dark: Color::from_rgb(0.3, 0.5, 0.85),
            bg_primary: Color::from_rgb(0.98, 0.98, 0.99),
            bg_secondary: Color::from_rgb(0.95, 0.95, 0.97),
            bg_tertiary: Color::from_rgb(0.92, 0.92, 0.94),
            bg_hover: Color::from_rgb(0.90, 0.90, 0.92),
            surface: Color::from_rgb(1.0, 1.0, 1.0),
            surface_hover: Color::from_rgb(0.98, 0.98, 1.0),
            surface_active: Color::from_rgb(0.96, 0.96, 0.98),
            surface_elevated: Color::from_rgb(1.0, 1.0, 1.0),
            text_primary: Color::from_rgb(0.1, 0.1, 0.12),
            text_secondary: Color::from_rgb(0.35, 0.35, 0.4),
            text_disabled: Color::from_rgb(0.6, 0.6, 0.65),
            success: Color::from_rgb(0.15, 0.75, 0.35),
            warning: Color::from_rgb(0.95, 0.65, 0.15),
            error: Color::from_rgb(0.9, 0.25, 0.25),
            info: Color::from_rgb(0.25, 0.55, 0.9),
            border: Color::from_rgb(0.85, 0.85, 0.9),
            border_light: Color::from_rgb(0.9, 0.9, 0.95),
            border_focus: Color::from_rgb(0.4, 0.6, 1.0),
        }
    }
}

pub fn get_colors(mode: ThemeMode) -> ColorScheme {
    match mode {
        ThemeMode::Dark => ColorScheme::dark(),
        ThemeMode::Light => ColorScheme::light(),
    }
}

pub fn cosmic_theme(mode: ThemeMode) -> Theme {
    let colors = get_colors(mode);
    Theme::custom(
        "Cosmic".to_string(),
        theme::Palette {
            background: colors.bg_primary,
            text: colors.text_primary,
            primary: colors.cosmic_accent,
            success: colors.success,
            danger: colors.error,
        },
    )
}

pub struct CardStyle {
    pub mode: ThemeMode,
}

impl iced::widget::container::StyleSheet for CardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::container::Appearance {
            text_color: Some(colors.text_primary),
            background: Some(iced::Background::Color(colors.surface_elevated)),
            border: iced::Border {
                color: colors.border_light,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: iced::Shadow {
                color: if self.mode == ThemeMode::Dark {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.15)
                } else {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.08)
                },
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }
}

pub struct ElevatedCardStyle {
    pub mode: ThemeMode,
}

impl iced::widget::container::StyleSheet for ElevatedCardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::container::Appearance {
            text_color: Some(colors.text_primary),
            background: Some(iced::Background::Color(colors.surface_elevated)),
            border: iced::Border {
                color: colors.border_light,
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: iced::Shadow {
                color: if self.mode == ThemeMode::Dark {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.25)
                } else {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.12)
                },
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
        }
    }
}

pub struct FileItemStyle {
    pub mode: ThemeMode,
}

impl iced::widget::container::StyleSheet for FileItemStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::container::Appearance {
            text_color: Some(colors.text_secondary),
            background: Some(iced::Background::Color(colors.bg_secondary)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Default::default(),
        }
    }
}

pub struct PrimaryButtonStyle {
    pub mode: ThemeMode,
}

impl iced::widget::button::StyleSheet for PrimaryButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(colors.cosmic_accent)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0.into(),
            },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: Color::from_rgba(0.25, 0.55, 0.95, if self.mode == ThemeMode::Dark { 0.3 } else { 0.2 }),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 8.0,
            },
            shadow_offset: iced::Vector::new(0.0, 3.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.cosmic_blue));
        appearance.shadow.offset = iced::Vector::new(0.0, 4.0);
        appearance.shadow.blur_radius = 8.0;
        appearance.shadow_offset = iced::Vector::new(0.0, 4.0);
        appearance
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.cosmic_purple));
        appearance.shadow.offset = iced::Vector::new(0.0, 1.0);
        appearance.shadow.blur_radius = 2.0;
        appearance.shadow_offset = iced::Vector::new(0.0, 1.0);
        appearance
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.bg_tertiary));
        appearance.text_color = colors.text_disabled;
        appearance.shadow = Default::default();
        appearance
    }
}

pub struct ProcessingButtonStyle {
    pub mode: ThemeMode,
    pub rotation: f32,
}

impl iced::widget::button::StyleSheet for ProcessingButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        // Pulsing purple effect during processing
        let pulse = ((self.rotation * 3.0).sin() + 1.0) / 2.0;
        let bg_color = if self.rotation > 0.0 {
            Color::from_rgb(
                0.5 + pulse * 0.15,
                0.3 + pulse * 0.1,
                0.8 + pulse * 0.15,
            )
        } else {
            colors.cosmic_accent
        };
        
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(bg_color)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0.into(),
            },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: Color::from_rgba(0.6, 0.35, 0.95, if self.mode == ThemeMode::Dark { 0.4 } else { 0.3 }),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 10.0,
            },
            shadow_offset: iced::Vector::new(0.0, 3.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }
}

pub struct SecondaryButtonStyle {
    pub mode: ThemeMode,
}

impl iced::widget::button::StyleSheet for SecondaryButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(colors.surface)),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: colors.text_primary,
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.surface_hover));
        appearance.border.color = colors.cosmic_accent;
        appearance
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.surface_active));
        appearance
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.bg_tertiary));
        appearance.text_color = colors.text_disabled;
        appearance.border.color = colors.border;
        appearance
    }
}

pub struct TextInputStyle {
    pub mode: ThemeMode,
}

impl iced::widget::text_input::StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(colors.bg_secondary),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            icon_color: colors.text_secondary,
        }
    }

    fn focused(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(colors.bg_secondary),
            border: iced::Border {
                color: colors.border_focus,
                width: 2.0,
                radius: 8.0.into(),
            },
            icon_color: colors.cosmic_accent,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        let colors = get_colors(self.mode);
        colors.text_disabled
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        let colors = get_colors(self.mode);
        colors.text_primary
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.4, 0.6, 1.0, 0.3)
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = iced::Background::Color(colors.bg_tertiary);
        appearance
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        let colors = get_colors(self.mode);
        colors.text_disabled
    }
}

pub struct ToggleStyle {
    pub mode: ThemeMode,
}

impl iced::widget::checkbox::StyleSheet for ToggleStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> iced::widget::checkbox::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::checkbox::Appearance {
            background: if is_checked {
                iced::Background::Color(colors.cosmic_accent)
            } else {
                iced::Background::Color(colors.bg_tertiary)
            },
            icon_color: Color::WHITE,
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 12.0.into(),
            },
            text_color: Some(colors.text_primary),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> iced::widget::checkbox::Appearance {
        let mut appearance = self.active(style, is_checked);
        let colors = get_colors(self.mode);
        if is_checked {
            appearance.background = iced::Background::Color(colors.cosmic_blue);
        } else {
            appearance.background = iced::Background::Color(colors.bg_hover);
        }
        appearance
    }
}

pub struct WarningButtonStyle {
    pub mode: ThemeMode,
}

impl iced::widget::button::StyleSheet for WarningButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let _colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
            text_color: if self.mode == ThemeMode::Dark {
                Color::from_rgb(0.75, 0.75, 0.8)
            } else {
                Color::from_rgb(0.4, 0.4, 0.45)
            },
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let error_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.95, 0.25, 0.25, 1.0)
        } else {
            Color::from_rgba(0.9, 0.2, 0.2, 1.0)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(error_color)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let error_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(1.0, 0.2, 0.2, 1.0)
        } else {
            Color::from_rgba(0.95, 0.15, 0.15, 1.0)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(error_color)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.background = Some(iced::Background::Color(colors.bg_tertiary));
        appearance.text_color = colors.text_disabled;
        appearance.border.color = colors.border;
        appearance
    }
}

pub struct DangerButtonStyle {
    pub mode: ThemeMode,
}

impl iced::widget::button::StyleSheet for DangerButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(colors.surface)),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: colors.text_secondary,
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let error_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.85, 0.2, 0.2, 1.0)
        } else {
            Color::from_rgba(0.9, 0.2, 0.2, 1.0)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(error_color)),
            border: iced::Border {
                color: error_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let error_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.95, 0.15, 0.15, 1.0)
        } else {
            Color::from_rgba(0.85, 0.15, 0.15, 1.0)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(error_color)),
            border: iced::Border {
                color: error_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Color::WHITE,
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(colors.bg_tertiary)),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: colors.text_disabled,
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }
}

pub struct TransparentButtonStyle {
    pub mode: ThemeMode,
    pub is_selected: bool,
}

impl iced::widget::button::StyleSheet for TransparentButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: if self.is_selected { colors.cosmic_accent } else { colors.text_primary },
            shadow: Default::default(),
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        let colors = get_colors(self.mode);
        appearance.text_color = colors.text_disabled;
        appearance
    }
}

