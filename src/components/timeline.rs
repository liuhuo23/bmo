use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use gpui::{
    ParentElement, Render, SharedString, Styled, div, percentage, prelude::FluentBuilder, px,
    relative, rems, rgb,
};
use gpui_component::ActiveTheme;

use crate::session::TimerPreset;

pub struct TimeLineSegment {
    duration: Duration,
    title: SharedString,
    color: u32,
}

pub struct TimeLine {
    pub active_index: usize,
    pub total_duration: Duration,
    pub segments: Vec<TimeLineSegment>,
}

impl TimeLine {
    pub fn new() -> Self {
        return Self {
            active_index: 0,
            total_duration: Duration::ZERO,
            segments: vec![],
        };
    }

    /// Generate a random color that complements dark UI themes
    /// Uses a hash of the title to ensure consistent colors for the same segment
    fn generate_color_for_segment(title: &str, index: usize) -> u32 {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        index.hash(&mut hasher);
        let hash = hasher.finish();

        // Generate muted colors that work well with dark backgrounds
        // Keep values in range 30-150 for good contrast without being too bright
        let r = ((hash >> 0) & 0xFF) as u8;
        let g = ((hash >> 8) & 0xFF) as u8;
        let b = ((hash >> 16) & 0xFF) as u8;

        // Map to a darker, more muted range (30-150)
        let r = 30 + (r as u32 * 120 / 255) as u8;
        let g = 30 + (g as u32 * 120 / 255) as u8;
        let b = 30 + (b as u32 * 120 / 255) as u8;

        // Pack into u32 as 0xRRGGBB
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn update_segments(&mut self, preset: &TimerPreset) {
        self.total_duration = preset.total_duration();
        self.segments = preset
            .sessions
            .iter()
            .enumerate()
            .map(|(index, session)| TimeLineSegment {
                duration: session.duration.clone(),
                title: session.title.clone(),
                color: Self::generate_color_for_segment(&session.title, index),
            })
            .collect();
    }
}

impl Render for TimeLine {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let segments = &self.segments;

        return div()
            .flex_grow()
            .h_16()
            .border_1()
            .border_color(cx.theme().border)
            .rounded_xl()
            .relative()
            .child(
                // stroke container
                div()
                    .p_1()
                    .size_full()
                    .absolute()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(
                        // stroke
                        div().w_full().h(px(1.)).bg(cx.theme().border),
                    ),
            )
            // segments
            .child(
                div()
                    .absolute()
                    .px_2()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .size_full()
                    .children(segments.iter().enumerate().map(|(index, seg)| {
                        div()
                            .min_w(rems(2.))
                            .when_else(
                                index == self.active_index,
                                |e| {
                                    e.flex_grow()
                                        .child(div().child(seg.title.clone()))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .h(rems(2.8))
                                },
                                |e| e.h(rems(1.6)),
                            )
                            .bg(rgb(seg.color))
                            .rounded_lg()
                    })),
            );
    }
}
