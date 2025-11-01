use gpui::{
    Context, Div, FontWeight, InteractiveElement, IntoElement, ParentElement, Render, Styled,
    Window, div, prelude::FluentBuilder, px, rgb, svg, white,
};

pub struct TimerApp {}

impl TimerApp {
    pub fn new() -> Self {
        return Self {};
    }

    fn button(&self, fill: bool) -> Div {
        return div()
            .border(px(1.))
            .border_color(rgb(0x5F5F5F))
            .rounded_full()
            .p_3()
            .when(fill, |el| el.bg(rgb(0x242424)))
            .hover(|e| e.bg(rgb(0x141414)));
    }

    fn focus_sessions_indicator(&self, session_count: u8, currently_at: u8) -> Div {
        return div()
            .flex_row()
            .flex()
            .gap_1()
            .children((1..session_count + 1).into_iter().map(|index| {
                div()
                    .h(px(9.))
                    .w(px(3.))
                    .bg(rgb(if currently_at >= index {
                        0xE93131
                    } else {
                        0x424242
                    }))
                    .rounded_full()
            }));
    }

    fn timer_widget(&self) -> Div {
        return div()
            .flex()
            .flex_col()
            .gap_4()
            .justify_center()
            .items_center()
            // state icon
            .child(svg().path("svg/eye.svg").size_8().text_color(white()))
            // timer count
            .child(
                div()
                    .child("00:00")
                    .text_3xl()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(white()),
            )
            // focus session count
            .child(self.focus_sessions_indicator(4, 2))
            // state text
            .child(
                div()
                    .child("FOCUS")
                    .text_color(rgb(0x4F4F4F))
                    .text_size(px(10.)),
            )
            .h(px(300.))
            .w(px(300.))
            .border_4()
            .border_color(rgb(0x3A3A3A))
            .rounded_full();
    }

    fn button_row(&self) -> Div {
        return div()
            .flex()
            .flex_row()
            .gap_4()
            .child(
                self.button(false).child(
                    svg()
                        .path("svg/return.svg")
                        .size_8()
                        .text_color(rgb(0x545454)),
                ),
            )
            .child(
                self.button(true)
                    .child("START")
                    .text_color(white())
                    .flex_grow()
                    .text_center(),
            )
            .child(
                self.button(false).child(
                    svg()
                        .path("svg/settings.svg")
                        .size_8()
                        .text_color(rgb(0x545454)),
                ),
            );
    }
}

impl Render for TimerApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .bg(rgb(0x090706))
            .size_full()
            .p_10()
            .flex()
            .flex_col()
            .gap_4()
            .justify_around()
            .items_center()
            .children([self.timer_widget(), self.button_row().w_full()]);
    }
}
