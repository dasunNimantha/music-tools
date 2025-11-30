use crate::message::Message;
use crate::model::{AppState, Screen};
use crate::theme::{
    get_colors, CardStyle, DangerButtonStyle, FileItemStyle, PrimaryButtonStyle,
    ProcessingButtonStyle, SecondaryButtonStyle, TextInputStyle, ThemeMode, ToggleStyle,
    TransparentButtonStyle, WarningButtonStyle,
};
use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, Column, Space,
};
use iced::{Alignment, Color, Element, Length, Theme};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};

pub fn build_view(state: &AppState, theme_mode: ThemeMode) -> Element<'_, Message> {
    let colors = get_colors(theme_mode);

    let content: Element<Message> = match state.current_screen {
        Screen::Home => build_home_screen(theme_mode),
        Screen::MetadataEditor => build_metadata_editor(state, theme_mode),
        Screen::MusicDownloader => build_music_downloader(state, theme_mode),
        Screen::AudioConverter => build_audio_converter(state, theme_mode),
    };

    let bg_primary = colors.bg_primary;
    let text_primary = colors.text_primary;
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(
            move |_theme: &Theme| iced::widget::container::Appearance {
                text_color: Some(text_primary),
                background: Some(iced::Background::Color(bg_primary)),
                border: iced::Border::default(),
                shadow: Default::default(),
            },
        )))
        .into()
}

// ============== HOME SCREEN ==============

fn build_home_screen(theme_mode: ThemeMode) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);

    let top_bar = container(
        row![
            Space::with_width(Length::Fill),
            row![
                text(if theme_mode == ThemeMode::Dark {
                    "Dark"
                } else {
                    "Light"
                })
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary)),
                Space::with_width(6),
                checkbox("", theme_mode == ThemeMode::Light)
                    .on_toggle(|_| Message::ToggleTheme)
                    .style(iced::theme::Checkbox::Custom(Box::new(ToggleStyle {
                        mode: theme_mode
                    }))),
            ]
            .spacing(0)
            .align_items(Alignment::Center),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([12, 20, 0, 20]);

    let header = container(
        column![
            row![
                icon_to_text(Bootstrap::MusicNoteList)
                    .size(32.0)
                    .style(iced::theme::Text::Color(colors.cosmic_accent)),
                Space::with_width(12),
                text("Music Tools")
                    .size(28)
                    .style(iced::theme::Text::Color(colors.text_primary)),
            ]
            .spacing(0)
            .align_items(Alignment::Center),
            Space::with_height(6),
            text("Your all-in-one music utility suite")
                .size(14)
                .style(iced::theme::Text::Color(colors.text_secondary)),
        ]
        .spacing(0)
        .align_items(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([20, 20, 30, 20]);

    let utility_cards = row![
        build_utility_card(
            Bootstrap::TagsFill,
            "Metadata Editor",
            "Edit artist, album, genre and cover art for your music files",
            colors.cosmic_accent,
            Screen::MetadataEditor,
            theme_mode,
        ),
        Space::with_width(20),
        build_utility_card(
            Bootstrap::CloudArrowDown,
            "Music Downloader",
            "Download music from various online sources",
            Color::from_rgb(0.4, 0.8, 0.4),
            Screen::MusicDownloader,
            theme_mode,
        ),
        Space::with_width(20),
        build_utility_card(
            Bootstrap::ArrowRepeat,
            "Audio Converter",
            "Convert audio files between different formats",
            Color::from_rgb(0.9, 0.6, 0.2),
            Screen::AudioConverter,
            theme_mode,
        ),
    ]
    .spacing(0)
    .align_items(Alignment::Center);

    let hint_text = text("Select a tool to get started")
        .size(12)
        .style(iced::theme::Text::Color(colors.text_disabled))
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .width(Length::Fill);

    column![
        top_bar,
        header,
        container(utility_cards)
            .width(Length::Fill)
            .center_x()
            .padding([0, 40, 0, 40]),
        Space::with_height(20),
        hint_text,
        Space::with_height(Length::Fill),
    ]
    .spacing(0)
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn build_utility_card(
    icon: Bootstrap,
    title: &str,
    description: &str,
    accent_color: Color,
    target_screen: Screen,
    theme_mode: ThemeMode,
) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);
    let title_owned = title.to_string();
    let description_owned = description.to_string();

    button(
        container(
            column![
                container(
                    icon_to_text(icon)
                        .size(48.0)
                        .style(iced::theme::Text::Color(accent_color))
                )
                .width(Length::Fixed(90.0))
                .height(Length::Fixed(90.0))
                .center_x()
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(16),
                text(title_owned)
                    .size(16)
                    .style(iced::theme::Text::Color(colors.text_primary))
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
                Space::with_height(8),
                text(description_owned)
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_secondary))
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
            .width(Length::Fill),
        )
        .width(Length::Fixed(220.0))
        .padding([24, 20, 24, 20])
        .style(iced::theme::Container::Custom(Box::new(CardStyle {
            mode: theme_mode,
        }))),
    )
    .style(iced::theme::Button::Custom(Box::new(
        UtilityCardButtonStyle { mode: theme_mode },
    )))
    .on_press(Message::NavigateTo(target_screen))
    .padding(0)
    .into()
}

struct UtilityCardButtonStyle {
    mode: ThemeMode,
}

impl iced::widget::button::StyleSheet for UtilityCardButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 12.0.into(),
            },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: Color::TRANSPARENT,
                offset: iced::Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        // Use cosmic blue for hover (same as button hover)
        let shadow_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.25, 0.55, 0.95, 0.5)
        } else {
            Color::from_rgba(0.25, 0.55, 0.95, 0.4)
        };
        let hover_bg = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.25, 0.55, 0.95, 0.08)
        } else {
            Color::from_rgba(0.25, 0.55, 0.95, 0.06)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(hover_bg)),
            border: iced::Border {
                color: colors.cosmic_accent,
                width: 1.5,
                radius: 12.0.into(),
            },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: shadow_color,
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        // Use darker blue for pressed state
        let shadow_color = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.2, 0.45, 0.85, 0.35)
        } else {
            Color::from_rgba(0.2, 0.45, 0.85, 0.25)
        };
        let press_bg = if self.mode == ThemeMode::Dark {
            Color::from_rgba(0.25, 0.55, 0.95, 0.12)
        } else {
            Color::from_rgba(0.25, 0.55, 0.95, 0.1)
        };
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(press_bg)),
            border: iced::Border {
                color: colors.cosmic_blue,
                width: 1.5,
                radius: 12.0.into(),
            },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: shadow_color,
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 6.0,
            },
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }
}

// ============== HEADER WITH BACK BUTTON ==============

fn build_app_header(
    title: &str,
    subtitle: &str,
    theme_mode: ThemeMode,
) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);
    let title_owned = title.to_string();
    let subtitle_owned = subtitle.to_string();

    container(
        row![
            button(
                row![
                    icon_to_text(Bootstrap::ArrowLeft)
                        .size(16.0)
                        .style(iced::theme::Text::Color(colors.text_secondary)),
                    Space::with_width(6),
                    text("Home")
                        .size(12)
                        .style(iced::theme::Text::Color(colors.text_secondary)),
                ]
                .spacing(0)
                .align_items(Alignment::Center)
            )
            .style(iced::theme::Button::Custom(Box::new(
                SecondaryButtonStyle { mode: theme_mode }
            )))
            .on_press(Message::GoHome)
            .padding([6, 12]),
            Space::with_width(20),
            column![
                text(title_owned)
                    .size(18)
                    .style(iced::theme::Text::Color(colors.text_primary)),
                text(subtitle_owned)
                    .size(11)
                    .style(iced::theme::Text::Color(colors.text_secondary)),
            ]
            .spacing(2)
            .width(Length::Fill),
            row![
                text(if theme_mode == ThemeMode::Dark {
                    "Dark"
                } else {
                    "Light"
                })
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary)),
                Space::with_width(6),
                checkbox("", theme_mode == ThemeMode::Light)
                    .on_toggle(|_| Message::ToggleTheme)
                    .style(iced::theme::Checkbox::Custom(Box::new(ToggleStyle {
                        mode: theme_mode
                    }))),
            ]
            .spacing(0)
            .align_items(Alignment::Center),
        ]
        .spacing(0)
        .align_items(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 16, 10, 16])
    .into()
}

// ============== METADATA EDITOR ==============

fn build_metadata_editor(state: &AppState, theme_mode: ThemeMode) -> Element<'_, Message> {
    let header = build_app_header(
        "Metadata Editor",
        "Edit artist, album, and cover art for your music files",
        theme_mode,
    );
    let file_panel = build_file_panel(state, theme_mode);
    let metadata_panel = build_metadata_panel(state, theme_mode);
    let edit_panel = build_edit_panel(state, theme_mode);

    let main_content = row![
        container(file_panel)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding([0, 6, 0, 12]),
        container(metadata_panel)
            .width(Length::FillPortion(4))
            .height(Length::Fill)
            .padding([0, 6, 0, 6]),
        container(edit_panel)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding([0, 12, 0, 6]),
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill);

    column![
        header,
        Space::with_height(8),
        main_content,
        Space::with_height(12),
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn build_file_panel(state: &AppState, theme_mode: ThemeMode) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);

    let file_list: Element<Message> = if state.loading_files {
        // Pulsing/shining effect
        let pulse = ((state.loading_rotation * 3.0).sin() + 1.0) / 2.0;
        let icon_color = Color::from_rgb(0.3 + pulse * 0.4, 0.5 + pulse * 0.3, 0.85 + pulse * 0.15);

        container(
            column![
                container(
                    icon_to_text(Bootstrap::ArrowClockwise)
                        .size(40.0)
                        .style(iced::theme::Text::Color(icon_color))
                )
                .width(Length::Fixed(70.0))
                .height(Length::Fixed(60.0))
                .center_x()
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(12),
                text("Loading files...")
                    .size(14)
                    .style(iced::theme::Text::Color(colors.text_secondary))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                text("Please wait")
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_disabled))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            ]
            .spacing(4)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    } else if state.files.is_empty() {
        container(
            column![
                container(
                    icon_to_text(Bootstrap::FolderPlus)
                        .size(36.0)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.95, 0.75, 0.3)))
                )
                .width(Length::Fixed(70.0))
                .height(Length::Fixed(60.0))
                .center_x()
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(12),
                text("No files loaded")
                    .size(14)
                    .style(iced::theme::Text::Color(colors.text_secondary))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                text("Select files or a folder")
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_disabled))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            ]
            .spacing(4)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    } else {
        let mut file_column = Column::new().spacing(3).width(Length::Fill);

        for (index, file) in state.files.iter().enumerate() {
            let file_name = file
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let max_length = 32;
            let display_name = if file_name.len() > max_length {
                format!("{}...", &file_name[..max_length.saturating_sub(3)])
            } else {
                file_name
            };

            let is_selected = state.selected_file_index == Some(index);
            let item_bg = if is_selected {
                if theme_mode == ThemeMode::Dark {
                    Color::from_rgba(0.45, 0.65, 1.0, 0.18)
                } else {
                    Color::from_rgba(0.4, 0.6, 1.0, 0.18)
                }
            } else {
                colors.bg_secondary
            };

            let file_item = container(
                row![
                    button(
                        container(
                            text(display_name)
                                .size(13)
                                .style(iced::theme::Text::Color(if is_selected {
                                    colors.cosmic_accent
                                } else {
                                    colors.text_primary
                                }))
                                .width(Length::Fill)
                                .shaping(iced::widget::text::Shaping::Advanced)
                        )
                        .width(Length::Fill)
                        .padding([6, 10, 6, 10])
                        .clip(true)
                    )
                    .style(iced::theme::Button::Custom(Box::new(
                        TransparentButtonStyle {
                            mode: theme_mode,
                            is_selected
                        }
                    )))
                    .on_press(Message::FileSelected(index))
                    .width(Length::Fill),
                    button(
                        container(text("×").size(16).width(Length::Shrink))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y()
                    )
                    .style(iced::theme::Button::Custom(Box::new(WarningButtonStyle {
                        mode: theme_mode
                    })))
                    .on_press(Message::RemoveFile(index))
                    .padding(0)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0)),
                ]
                .spacing(4)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .width(Length::Fill)
            .padding([0, 4, 0, 0])
            .style(iced::theme::Container::Custom(Box::new(
                move |_theme: &Theme| iced::widget::container::Appearance {
                    text_color: Some(colors.text_secondary),
                    background: Some(iced::Background::Color(item_bg)),
                    border: iced::Border {
                        color: if is_selected {
                            colors.cosmic_accent
                        } else {
                            Color::TRANSPARENT
                        },
                        width: if is_selected { 1.0 } else { 0.0 },
                        radius: 4.0.into(),
                    },
                    shadow: Default::default(),
                },
            )));

            file_column = file_column.push(file_item);
        }

        scrollable(
            container(file_column)
                .width(Length::Fill)
                .padding([4, 16, 4, 8]),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };

    container(
        column![
            row![
                text("Files")
                    .size(14)
                    .style(iced::theme::Text::Color(colors.text_primary)),
                Space::with_width(Length::Fill),
                container(
                    text(format!("{}", state.files.len()))
                        .size(11)
                        .style(iced::theme::Text::Color(colors.text_secondary))
                )
                .padding([3, 10])
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
            ]
            .spacing(6)
            .align_items(Alignment::Center)
            .width(Length::Fill),
            Space::with_height(10),
            row![
                button("Select Files")
                    .style(iced::theme::Button::Custom(Box::new(
                        SecondaryButtonStyle { mode: theme_mode }
                    )))
                    .on_press(Message::SelectFiles)
                    .padding([8, 12])
                    .width(Length::Fill),
                Space::with_width(8),
                button("Select Folder")
                    .style(iced::theme::Button::Custom(Box::new(
                        SecondaryButtonStyle { mode: theme_mode }
                    )))
                    .on_press(Message::SelectFolder)
                    .padding([8, 12])
                    .width(Length::Fill),
            ]
            .spacing(0)
            .width(Length::Fill),
            Space::with_height(10),
            container(file_list)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
            Space::with_height(10),
            button("Clear All")
                .style(iced::theme::Button::Custom(Box::new(DangerButtonStyle {
                    mode: theme_mode
                })))
                .on_press_maybe(if state.files.is_empty() {
                    None
                } else {
                    Some(Message::ClearAllFiles)
                })
                .padding([8, 12])
                .width(Length::Fill),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([12, 14, 12, 14])
    .style(iced::theme::Container::Custom(Box::new(CardStyle {
        mode: theme_mode,
    })))
    .into()
}

fn build_metadata_panel(state: &AppState, theme_mode: ThemeMode) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);

    let content: Element<Message> = if let Some(selected_idx) = state.selected_file_index {
        if let Some(metadata) = state.file_metadata.get(&selected_idx) {
            let file_name_full = state.files[selected_idx]
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            // Truncate long file names with ellipsis
            let file_name = if file_name_full.len() > 45 {
                format!("{}...", &file_name_full[..42])
            } else {
                file_name_full
            };

            let duration_str = if let Some(dur) = metadata.duration {
                let mins = dur / 60;
                let secs = dur % 60;
                format!("{}:{:02}", mins, secs)
            } else {
                "—".to_string()
            };

            let audio_info = format!(
                "{} • {} kbps • {} Hz",
                metadata.format,
                metadata.bitrate.unwrap_or(0),
                metadata.sample_rate.unwrap_or(0)
            );

            column![
                row![
                    text("Current File")
                        .size(14)
                        .style(iced::theme::Text::Color(colors.text_primary)),
                    Space::with_width(Length::Fill),
                    container(
                        text(&duration_str)
                            .size(12)
                            .style(iced::theme::Text::Color(colors.text_secondary))
                    )
                    .padding([4, 10])
                    .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                        mode: theme_mode
                    }))),
                ]
                .align_items(Alignment::Center)
                .width(Length::Fill),
                Space::with_height(8),
                container(
                    text(&file_name)
                        .size(13)
                        .style(iced::theme::Text::Color(colors.cosmic_accent))
                        .width(Length::Fill)
                        .shaping(iced::widget::text::Shaping::Advanced)
                )
                .width(Length::Fill)
                .padding([10, 12])
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(8),
                text(&audio_info)
                    .size(11)
                    .style(iced::theme::Text::Color(colors.text_disabled))
                    .width(Length::Fill),
                Space::with_height(18),
                text("Metadata")
                    .size(14)
                    .style(iced::theme::Text::Color(colors.text_primary))
                    .width(Length::Fill),
                Space::with_height(12),
                build_metadata_row(
                    "Title",
                    if metadata.title.is_empty() {
                        "—"
                    } else {
                        &metadata.title
                    },
                    theme_mode
                ),
                Space::with_height(10),
                build_metadata_row(
                    "Artist",
                    if metadata.artist.is_empty() {
                        "—"
                    } else {
                        &metadata.artist
                    },
                    theme_mode
                ),
                Space::with_height(10),
                build_metadata_row(
                    "Album",
                    if metadata.album.is_empty() {
                        "—"
                    } else {
                        &metadata.album
                    },
                    theme_mode
                ),
                Space::with_height(10),
                row![
                    build_metadata_field(
                        "Genre",
                        if metadata.genre.is_empty() {
                            "—"
                        } else {
                            &metadata.genre
                        },
                        theme_mode
                    ),
                    Space::with_width(14),
                    build_metadata_field(
                        "Year",
                        &metadata
                            .year
                            .map(|y| y.to_string())
                            .unwrap_or("—".to_string()),
                        theme_mode
                    ),
                    Space::with_width(14),
                    build_metadata_field(
                        "Track",
                        &metadata
                            .track
                            .map(|t| t.to_string())
                            .unwrap_or("—".to_string()),
                        theme_mode
                    ),
                ]
                .spacing(0)
                .width(Length::Fill),
            ]
            .spacing(0)
            .width(Length::Fill)
            .into()
        } else {
            container(
                column![
                    icon_to_text(Bootstrap::HourglassSplit)
                        .size(28.0)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.7, 0.95))),
                    Space::with_height(10),
                    text("Loading metadata...")
                        .size(13)
                        .style(iced::theme::Text::Color(colors.text_secondary))
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ]
                .spacing(0)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        }
    } else {
        container(
            column![
                container(
                    icon_to_text(Bootstrap::MusicNoteBeamed)
                        .size(40.0)
                        .style(iced::theme::Text::Color(colors.cosmic_accent))
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(70.0))
                .center_x()
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(14),
                text("Select a file to view metadata")
                    .size(14)
                    .style(iced::theme::Text::Color(colors.text_secondary))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                Space::with_height(4),
                text("Click on any file from the list")
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_disabled))
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            ]
            .spacing(0)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    };

    // Build status/log section
    let status_content: Element<Message> = if !state.error_logs.is_empty() {
        // Show scrollable error logs
        let mut log_column = Column::new().spacing(4);
        for error in &state.error_logs {
            let error_text = error.clone();
            log_column = log_column.push(
                text(error_text)
                    .size(11)
                    .style(iced::theme::Text::Color(colors.error)),
            );
        }

        container(
            scrollable(container(log_column).width(Length::Fill).padding([8, 10]))
                .height(Length::Fixed(120.0)),
        )
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
            mode: theme_mode,
        })))
        .into()
    } else {
        // Show single status line with icon
        let (icon, text_color, status_text) =
            if state.status.starts_with("Error") || state.status.contains("error") {
                (Some(Bootstrap::XCircle), colors.error, state.status.clone())
            } else if state.status.starts_with("✓") || state.status.contains("Success") {
                // Remove the checkmark character from the text since we're showing an icon
                let cleaned = state
                    .status
                    .strip_prefix("✓ ")
                    .unwrap_or(&state.status)
                    .to_string();
                (Some(Bootstrap::CheckCircle), colors.success, cleaned)
            } else if state.status.contains("Processing") {
                (
                    Some(Bootstrap::ArrowClockwise),
                    colors.info,
                    state.status.clone(),
                )
            } else {
                (None, colors.text_secondary, state.status.clone())
            };

        let status_row = if let Some(icon_type) = icon {
            row![
                icon_to_text(icon_type)
                    .size(14.0)
                    .style(iced::theme::Text::Color(text_color)),
                Space::with_width(8),
                text(&status_text)
                    .size(12)
                    .style(iced::theme::Text::Color(text_color))
                    .width(Length::Fill),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        } else {
            row![text(&status_text)
                .size(12)
                .style(iced::theme::Text::Color(text_color))
                .width(Length::Fill),]
        };

        container(status_row)
            .width(Length::Fill)
            .padding([10, 12])
            .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                mode: theme_mode,
            })))
            .into()
    };

    container(
        column![content, Space::with_height(Length::Fill), status_content,]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([12, 14, 12, 14])
    .style(iced::theme::Container::Custom(Box::new(CardStyle {
        mode: theme_mode,
    })))
    .into()
}

fn build_metadata_row(
    label: &str,
    value: &str,
    theme_mode: ThemeMode,
) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);
    let value_owned = value.to_string();
    row![
        text(label)
            .size(12)
            .style(iced::theme::Text::Color(colors.text_secondary))
            .width(Length::Fixed(60.0)),
        container(
            text(value_owned)
                .size(13)
                .style(iced::theme::Text::Color(colors.text_primary))
                .width(Length::Fill)
        )
        .width(Length::Fill)
        .padding([8, 12])
        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
            mode: theme_mode
        }))),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn build_metadata_field(
    label: &str,
    value: &str,
    theme_mode: ThemeMode,
) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);
    let value_owned = value.to_string();
    column![
        text(label)
            .size(11)
            .style(iced::theme::Text::Color(colors.text_secondary))
            .width(Length::Fill),
        Space::with_height(5),
        container(
            text(value_owned)
                .size(13)
                .style(iced::theme::Text::Color(colors.text_primary))
                .width(Length::Fill)
        )
        .width(Length::Fill)
        .padding([8, 12])
        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
            mode: theme_mode
        }))),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn build_edit_panel(state: &AppState, theme_mode: ThemeMode) -> Element<'static, Message> {
    let colors = get_colors(theme_mode);

    container(
        column![
            text("Edit Metadata")
                .size(14)
                .style(iced::theme::Text::Color(colors.text_primary))
                .width(Length::Fill),
            Space::with_height(14),
            text("Artist")
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary))
                .width(Length::Fill),
            Space::with_height(5),
            text_input("Enter artist name", &state.artist)
                .on_input(Message::ArtistChanged)
                .width(Length::Fill)
                .padding(10)
                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                    mode: theme_mode
                }))),
            Space::with_height(12),
            text("Album")
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary))
                .width(Length::Fill),
            Space::with_height(5),
            text_input("Enter album name", &state.album)
                .on_input(Message::AlbumChanged)
                .width(Length::Fill)
                .padding(10)
                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                    mode: theme_mode
                }))),
            Space::with_height(12),
            row![
                column![
                    text("Genre")
                        .size(11)
                        .style(iced::theme::Text::Color(colors.text_secondary))
                        .width(Length::Fill),
                    Space::with_height(5),
                    text_input("Genre", &state.genre)
                        .on_input(Message::GenreChanged)
                        .width(Length::Fill)
                        .padding(10)
                        .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                            mode: theme_mode
                        }))),
                ]
                .spacing(0)
                .width(Length::FillPortion(2)),
                Space::with_width(10),
                column![
                    text("Year")
                        .size(11)
                        .style(iced::theme::Text::Color(colors.text_secondary))
                        .width(Length::Fill),
                    Space::with_height(5),
                    text_input("Year", &state.year)
                        .on_input(Message::YearChanged)
                        .width(Length::Fill)
                        .padding(10)
                        .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                            mode: theme_mode
                        }))),
                ]
                .spacing(0)
                .width(Length::FillPortion(1)),
            ]
            .spacing(0)
            .width(Length::Fill),
            Space::with_height(14),
            text("Album Art")
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary))
                .width(Length::Fill),
            Space::with_height(5),
            row![button(if state.album_art_path.is_some() {
                "Change Image"
            } else {
                "Select Image"
            })
            .style(iced::theme::Button::Custom(Box::new(
                SecondaryButtonStyle { mode: theme_mode }
            )))
            .on_press(Message::SelectImage)
            .padding([8, 14])
            .width(Length::Fill),]
            .spacing(0)
            .width(Length::Fill),
            Space::with_height(6),
            container(
                text(if let Some(ref path) = state.album_art_path {
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Image selected")
                } else {
                    "No image selected"
                })
                .size(11)
                .style(iced::theme::Text::Color(
                    if state.album_art_path.is_some() {
                        colors.success
                    } else {
                        colors.text_disabled
                    }
                ))
                .width(Length::Fill)
            )
            .width(Length::Fill)
            .padding([8, 10])
            .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                mode: theme_mode
            }))),
            Space::with_height(Length::Fill),
            button(if state.processing {
                // Pulsing icon during processing
                let pulse = ((state.loading_rotation * 3.0).sin() + 1.0) / 2.0;
                let icon_color = Color::from_rgb(1.0, 1.0, 0.8 + pulse * 0.2);
                row![
                    icon_to_text(Bootstrap::ArrowClockwise)
                        .size(14.0)
                        .style(iced::theme::Text::Color(icon_color)),
                    Space::with_width(8),
                    text("Processing...").size(14),
                ]
                .spacing(0)
                .align_items(Alignment::Center)
                .width(Length::Fill)
            } else {
                row![text("Apply to All Files")
                    .size(14)
                    .width(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),]
                .width(Length::Fill)
            })
            .style(iced::theme::Button::Custom(Box::new(
                ProcessingButtonStyle {
                    mode: theme_mode,
                    rotation: if state.processing {
                        state.loading_rotation
                    } else {
                        0.0
                    }
                }
            )))
            .on_press_maybe(if state.processing || state.files.is_empty() {
                None
            } else {
                Some(Message::ProcessFiles)
            })
            .width(Length::Fill)
            .padding([12, 16]),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([12, 14, 12, 14])
    .style(iced::theme::Container::Custom(Box::new(CardStyle {
        mode: theme_mode,
    })))
    .into()
}

// ============== MUSIC DOWNLOADER (PLACEHOLDER) ==============

fn build_music_downloader(state: &AppState, theme_mode: ThemeMode) -> Element<'_, Message> {
    let colors = get_colors(theme_mode);
    let header = build_app_header(
        "Music Downloader",
        "Download music from songhub.lk",
        theme_mode,
    );

    let downloader = &state.downloader_state;

    // Artist search section
    let search_section = container(
        column![
            text("Search Artists")
                .size(15)
                .style(iced::theme::Text::Color(colors.text_primary))
                .width(Length::Fill),
            Space::with_height(10),
            text_input("Type a letter (A-Z) to load artists or type to search...", &downloader.artist_search_query)
                .on_input(Message::DownloaderArtistSearchChanged)
                .width(Length::Fill)
                .padding(12)
                .style(iced::theme::TextInput::Custom(Box::new(TextInputStyle {
                    mode: theme_mode
                }))),
            Space::with_height(8),
            row![
                icon_to_text(Bootstrap::InfoCircle)
                    .size(12.0)
                    .style(iced::theme::Text::Color(colors.text_disabled)),
                Space::with_width(6),
                text("Tip: Type a single letter (e.g., 'g') to load all artists starting with that letter")
                    .size(11)
                    .style(iced::theme::Text::Color(colors.text_disabled))
                    .width(Length::Fill),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(0)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([14, 16, 14, 16])
    .style(iced::theme::Container::Custom(Box::new(CardStyle { mode: theme_mode })));

    // Main content: Artists list or Songs list
    let main_content: Element<Message> =
        if let Some(ref selected_artist) = downloader.selected_artist {
            // Show songs for selected artist
            if downloader.loading_songs {
                // Pulsing/shining effect like in metadata editor
                let pulse = ((state.loading_rotation * 3.0).sin() + 1.0) / 2.0;
                let icon_color =
                    Color::from_rgb(0.3 + pulse * 0.4, 0.5 + pulse * 0.3, 0.85 + pulse * 0.15);

                container(
                    column![
                        container(
                            icon_to_text(Bootstrap::ArrowClockwise)
                                .size(40.0)
                                .style(iced::theme::Text::Color(icon_color))
                        )
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                        .center_x()
                        .center_y()
                        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                            mode: theme_mode
                        }))),
                        Space::with_height(12),
                        text(format!("Loading songs for {}...", selected_artist.name))
                            .size(14)
                            .style(iced::theme::Text::Color(colors.text_secondary))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text("Please wait")
                            .size(12)
                            .style(iced::theme::Text::Color(colors.text_disabled))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            } else if downloader.search_results.is_empty() {
                container(
                    column![
                        container(
                            icon_to_text(Bootstrap::MusicNoteBeamed)
                                .size(40.0)
                                .style(iced::theme::Text::Color(Color::from_rgb(0.4, 0.8, 0.4)))
                        )
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                        .center_x()
                        .center_y()
                        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                            mode: theme_mode
                        }))),
                        Space::with_height(12),
                        text(format!("No songs found for {}", selected_artist.name))
                            .size(14)
                            .style(iced::theme::Text::Color(colors.text_secondary))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            } else {
                // Show songs list
                let mut songs_column = Column::new().spacing(4).width(Length::Fill);

                for (index, song) in downloader.search_results.iter().enumerate() {
                    let is_selected = downloader.selected_songs.contains(&index);
                    let song_item = container(
                        row![
                            checkbox("", is_selected)
                                .on_toggle(move |_| Message::ToggleSongSelection(index))
                                .style(iced::theme::Checkbox::Custom(Box::new(ToggleStyle {
                                    mode: theme_mode
                                }))),
                            Space::with_width(10),
                            text(&song.title)
                                .size(13)
                                .style(iced::theme::Text::Color(colors.text_primary))
                                .width(Length::Fill),
                        ]
                        .spacing(0)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                    )
                    .width(Length::Fill)
                    .padding([8, 12])
                    .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                        mode: theme_mode,
                    })));

                    songs_column = songs_column.push(song_item);
                }

                container(
                    scrollable(container(songs_column).width(Length::Fill).padding([8, 10]))
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(CardStyle {
                    mode: theme_mode,
                })))
                .into()
            }
        } else {
            // Show artists list
            if downloader.loading_artists {
                // Pulsing/shining effect like in metadata editor
                let pulse = ((state.loading_rotation * 3.0).sin() + 1.0) / 2.0;
                let icon_color =
                    Color::from_rgb(0.3 + pulse * 0.4, 0.5 + pulse * 0.3, 0.85 + pulse * 0.15);

                container(
                    column![
                        container(
                            icon_to_text(Bootstrap::ArrowClockwise)
                                .size(40.0)
                                .style(iced::theme::Text::Color(icon_color))
                        )
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                        .center_x()
                        .center_y()
                        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                            mode: theme_mode
                        }))),
                        Space::with_height(12),
                        text(if downloader.artist_search_query.trim().len() == 1 {
                            format!(
                                "Loading artists starting with '{}'...",
                                downloader.artist_search_query.trim().to_uppercase()
                            )
                        } else {
                            "Loading artists...".to_string()
                        })
                        .size(14)
                        .style(iced::theme::Text::Color(colors.text_secondary))
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text("Please wait")
                            .size(12)
                            .style(iced::theme::Text::Color(colors.text_disabled))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            } else if downloader.filtered_artists.is_empty() {
                let (title, subtitle, show_button) =
                    if downloader.all_artists.is_empty() && !downloader.loading_artists {
                        (
                            "No artists loaded",
                            "Click the button below to load artists",
                            true,
                        )
                    } else if downloader.loading_artists {
                        ("Loading artists...", "Please wait", false)
                    } else if !downloader.artist_search_query.trim().is_empty() {
                        ("No artists found", "Try a different search term", false)
                    } else {
                        (
                            "No artists available",
                            "Please wait for artists to load",
                            false,
                        )
                    };

                let mut content =
                    column![
                        container(
                            icon_to_text(if downloader.loading_artists {
                                Bootstrap::ArrowClockwise
                            } else {
                                Bootstrap::CloudArrowDown
                            })
                            .size(40.0)
                            .style(iced::theme::Text::Color(if downloader.loading_artists {
                                colors.info
                            } else {
                                Color::from_rgb(0.4, 0.8, 0.4)
                            }))
                        )
                        .width(Length::Fixed(70.0))
                        .height(Length::Fixed(60.0))
                        .center_x()
                        .center_y()
                        .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                            mode: theme_mode
                        }))),
                        Space::with_height(12),
                        text(title)
                            .size(14)
                            .style(iced::theme::Text::Color(colors.text_secondary))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text(subtitle)
                            .size(12)
                            .style(iced::theme::Text::Color(colors.text_disabled))
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center);

                if show_button {
                    content = content.push(Space::with_height(16));
                    content = content.push(
                        button("Load Artists")
                            .style(iced::theme::Button::Custom(Box::new(PrimaryButtonStyle {
                                mode: theme_mode,
                            })))
                            .on_press(Message::LoadArtists)
                            .padding([10, 20]),
                    );
                }

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            } else {
                // Show artists list
                let mut artists_column = Column::new().spacing(4).width(Length::Fill);

                for (index, artist) in downloader.filtered_artists.iter().enumerate() {
                    let is_selected = downloader
                        .selected_artist
                        .as_ref()
                        .map(|a| a.slug == artist.slug)
                        .unwrap_or(false);
                    let artist_item = container(
                        button(
                            row![
                                icon_to_text(Bootstrap::Person).size(14.0).style(
                                    iced::theme::Text::Color(if is_selected {
                                        Color::WHITE
                                    } else {
                                        colors.cosmic_accent
                                    })
                                ),
                                Space::with_width(10),
                                text(&artist.name)
                                    .size(14)
                                    .style(iced::theme::Text::Color(if is_selected {
                                        Color::WHITE
                                    } else {
                                        colors.text_primary
                                    }))
                                    .width(Length::Fill),
                            ]
                            .spacing(0)
                            .align_items(Alignment::Center)
                            .width(Length::Fill),
                        )
                        .style(iced::theme::Button::Custom(Box::new(
                            TransparentButtonStyle {
                                mode: theme_mode,
                                is_selected,
                            },
                        )))
                        .on_press(Message::SelectArtist(index))
                        .width(Length::Fill)
                        .padding([10, 14]),
                    )
                    .width(Length::Fill)
                    .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                        mode: theme_mode,
                    })));

                    artists_column = artists_column.push(artist_item);
                }

                container(
                    scrollable(
                        container(artists_column)
                            .width(Length::Fill)
                            .padding([8, 10]),
                    )
                    .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(CardStyle {
                    mode: theme_mode,
                })))
                .into()
            }
        };

    // Actions section
    let actions_section = if downloader.selected_artist.is_some() {
        // Show back button and download controls when artist is selected
        container(
            column![
                row![
                    button(
                        row![
                            icon_to_text(Bootstrap::ArrowLeft)
                                .size(14.0)
                                .style(iced::theme::Text::Color(Color::WHITE)),
                            Space::with_width(8),
                            text("Back to Artists").size(14),
                        ]
                        .spacing(0)
                        .align_items(Alignment::Center)
                    )
                    .style(iced::theme::Button::Custom(Box::new(
                        SecondaryButtonStyle { mode: theme_mode }
                    )))
                    .on_press(Message::SelectArtist(usize::MAX)) // Clear selection
                    .padding([10, 16])
                    .width(Length::Fill),
                    Space::with_width(10),
                    button("Select Download Directory")
                        .style(iced::theme::Button::Custom(Box::new(
                            SecondaryButtonStyle { mode: theme_mode }
                        )))
                        .on_press(Message::SelectDownloadDirectory)
                        .padding([10, 16])
                        .width(Length::Fill),
                    Space::with_width(10),
                    button(if downloader.downloading {
                        row![
                            icon_to_text(Bootstrap::ArrowClockwise)
                                .size(14.0)
                                .style(iced::theme::Text::Color(Color::WHITE)),
                            Space::with_width(8),
                            text("Downloading...").size(14),
                        ]
                        .spacing(0)
                        .align_items(Alignment::Center)
                    } else {
                        row![
                            icon_to_text(Bootstrap::CloudArrowDown)
                                .size(14.0)
                                .style(iced::theme::Text::Color(Color::WHITE)),
                            Space::with_width(8),
                            text(format!("Download ({})", downloader.selected_songs.len()))
                                .size(14),
                        ]
                        .spacing(0)
                        .align_items(Alignment::Center)
                    })
                    .style(iced::theme::Button::Custom(Box::new(PrimaryButtonStyle {
                        mode: theme_mode
                    })))
                    .on_press_maybe(
                        if downloader.downloading || downloader.selected_songs.is_empty() {
                            None
                        } else {
                            Some(Message::DownloadSelectedSongs)
                        }
                    )
                    .padding([10, 16])
                    .width(Length::Fill),
                ]
                .spacing(0)
                .width(Length::Fill),
                Space::with_height(8),
                container(
                    text(if let Some(ref path) = downloader.download_path {
                        format!("Download to: {}", path.display())
                    } else {
                        "No download directory selected".to_string()
                    })
                    .size(11)
                    .style(iced::theme::Text::Color(
                        if downloader.download_path.is_some() {
                            colors.success
                        } else {
                            colors.text_disabled
                        }
                    ))
                    .width(Length::Fill)
                )
                .width(Length::Fill)
                .padding([8, 10])
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
                Space::with_height(8),
                container(
                    text(&downloader.status)
                        .size(12)
                        .style(iced::theme::Text::Color(
                            if downloader.status.contains("Error")
                                || downloader.status.contains("Failed")
                            {
                                colors.error
                            } else if downloader.status.contains("Success") {
                                colors.success
                            } else {
                                colors.text_secondary
                            }
                        ))
                        .width(Length::Fill)
                )
                .width(Length::Fill)
                .padding([10, 12])
                .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                    mode: theme_mode
                }))),
            ]
            .spacing(0)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding([12, 14, 12, 14])
        .style(iced::theme::Container::Custom(Box::new(CardStyle {
            mode: theme_mode,
        })))
    } else {
        // Show simple status when no artist selected
        container(
            container(
                text(&downloader.status)
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_secondary))
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .padding([10, 12])
            .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                mode: theme_mode,
            }))),
        )
        .width(Length::Fill)
        .padding([12, 14, 12, 14])
        .style(iced::theme::Container::Custom(Box::new(CardStyle {
            mode: theme_mode,
        })))
    };

    column![
        header,
        Space::with_height(8),
        container(search_section)
            .width(Length::Fill)
            .padding([0, 12, 0, 12]),
        Space::with_height(12),
        row![container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0, 6, 0, 12]),]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill),
        Space::with_height(12),
        container(actions_section)
            .width(Length::Fill)
            .padding([0, 12, 12, 12]),
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// ============== AUDIO CONVERTER (PLACEHOLDER) ==============

fn build_audio_converter(state: &AppState, theme_mode: ThemeMode) -> Element<'_, Message> {
    let colors = get_colors(theme_mode);
    let header = build_app_header(
        "Audio Converter",
        "Convert audio files between formats",
        theme_mode,
    );

    let content = container(
        column![
            container(
                icon_to_text(Bootstrap::ArrowRepeat)
                    .size(64.0)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.9, 0.6, 0.2)))
            )
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(100.0))
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                mode: theme_mode
            }))),
            Space::with_height(24),
            text("Audio Converter")
                .size(20)
                .style(iced::theme::Text::Color(colors.text_primary)),
            Space::with_height(8),
            text("Convert between MP3, FLAC, WAV, AAC, and more")
                .size(13)
                .style(iced::theme::Text::Color(colors.text_secondary)),
            Space::with_height(24),
            button("Select Files to Convert")
                .style(iced::theme::Button::Custom(Box::new(
                    SecondaryButtonStyle { mode: theme_mode }
                )))
                .on_press(Message::SelectConvertFiles)
                .padding([12, 24])
                .width(Length::Fixed(400.0)),
            Space::with_height(16),
            text("Output Format")
                .size(11)
                .style(iced::theme::Text::Color(colors.text_secondary))
                .width(Length::Fixed(400.0)),
            Space::with_height(6),
            row![
                build_format_button("MP3", &state.convert_format, theme_mode),
                Space::with_width(8),
                build_format_button("FLAC", &state.convert_format, theme_mode),
                Space::with_width(8),
                build_format_button("WAV", &state.convert_format, theme_mode),
                Space::with_width(8),
                build_format_button("AAC", &state.convert_format, theme_mode),
            ]
            .spacing(0)
            .width(Length::Fixed(400.0)),
            Space::with_height(16),
            button("Convert Files")
                .style(iced::theme::Button::Custom(Box::new(PrimaryButtonStyle {
                    mode: theme_mode
                })))
                .on_press(Message::StartConvert)
                .padding([12, 40])
                .width(Length::Fixed(400.0)),
            Space::with_height(16),
            container(
                text(&state.convert_status)
                    .size(12)
                    .style(iced::theme::Text::Color(colors.text_secondary))
            )
            .width(Length::Fixed(400.0))
            .padding([10, 12])
            .style(iced::theme::Container::Custom(Box::new(FileItemStyle {
                mode: theme_mode
            }))),
        ]
        .spacing(0)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y();

    column![header, content,]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn build_format_button(
    format: &str,
    current: &str,
    theme_mode: ThemeMode,
) -> Element<'static, Message> {
    let is_selected = current == format;
    let format_owned = format.to_string();

    button(
        text(format)
            .size(12)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .width(Length::Fill),
    )
    .style(iced::theme::Button::Custom(Box::new(FormatButtonStyle {
        mode: theme_mode,
        is_selected,
    })))
    .on_press(Message::ConvertFormatChanged(format_owned))
    .padding([8, 16])
    .width(Length::Fill)
    .into()
}

struct FormatButtonStyle {
    mode: ThemeMode,
    is_selected: bool,
}

impl iced::widget::button::StyleSheet for FormatButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let colors = get_colors(self.mode);
        if self.is_selected {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(colors.cosmic_accent)),
                border: iced::Border {
                    color: colors.cosmic_accent,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: Color::WHITE,
                shadow: Default::default(),
                shadow_offset: iced::Vector::new(0.0, 0.0),
            }
        } else {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(colors.surface)),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: colors.text_primary,
                shadow: Default::default(),
                shadow_offset: iced::Vector::new(0.0, 0.0),
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        if !self.is_selected {
            let colors = get_colors(self.mode);
            appearance.border.color = colors.cosmic_accent;
        }
        appearance
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.hovered(style)
    }
}
