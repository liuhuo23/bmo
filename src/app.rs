use std::time::Duration;

use gpui::{
    AppContext, Context, Div, Entity, FontWeight, InteractiveElement, IntoElement, MouseButton,
    MouseUpEvent, ParentElement, Render, Styled, Task, Window, div, prelude::FluentBuilder, px,
    rgb, svg, white,
};

use crate::utils::format_time;

// pomorodo session info
pub struct PomorodoSession {
    session_count: u8,
    break_duration: u32,
    focus_duration: u32,
}

impl Default for PomorodoSession {
    fn default() -> Self {
        return Self {
            session_count: 4,
            break_duration: 60 * 10, // ten minutes
            focus_duration: 60 * 60, // one hour
        };
    }
}

pub struct TimerApp {
    remaining_seconds: u32,                   // count down
    session_progress: u8,                     // how many sessions have we passed
    current_session: Entity<PomorodoSession>, // active session
    timer_task: Option<Task<()>>,             // active timer
    is_running: bool,
    is_paused: bool,
    is_break: bool,
}

impl TimerApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        return Self {
            remaining_seconds: 0,
            session_progress: 0,
            current_session: cx.new(|_| PomorodoSession::default()),
            timer_task: None,
            is_paused: false,
            is_running: false,
            is_break: false,
        };
    }

    // moves on to the next session
    // returns if there was a new session to go to
    fn roll_over(&mut self, cx: &Context<Self>) -> bool {
        let current_session = self.current_session.read(cx);
        if self.session_progress == current_session.session_count {
            return false;
        }

        if !self.is_break {
            self.session_progress += 1;
        }

        self.remaining_seconds = if self.is_break {
            current_session.break_duration
        } else {
            current_session.focus_duration
        };

        return true;
    }

    fn start_timer(&mut self, cx: &mut Context<Self>) {
        if self.is_running {
            // do smth else
            return;
        }

        // set initials
        self.is_running = true;
        self.is_break = false;
        self.roll_over(cx);

        // spawn timer task
        self.timer_task = Some(cx.spawn(async move |entity, cx| {
            loop {
                // wait one sec
                cx.background_executor().timer(Duration::from_secs(1)).await;

                // process
                let running = entity.update(cx, |entity, cx| {
                    if !entity.is_running || entity.is_paused {
                        cx.notify();
                        return false;
                    }

                    if entity.remaining_seconds == 0 {
                        entity.is_break = !entity.is_break;
                        if !entity.roll_over(cx) {
                            // reset internals
                            entity.is_running = false;
                            entity.is_paused = false;
                            cx.notify();
                            return false;
                        };
                    } else {
                        entity.remaining_seconds -= 1;
                    }

                    cx.notify();
                    return true;
                });

                if !running.unwrap_or(false) {
                    break;
                }
            }
        }));
    }

    fn handle_action_button(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_timer(cx);
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
                    .child(format_time(self.remaining_seconds))
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

    fn bottom_button_row(&self, cx: &mut Context<Self>) -> Div {
        return div()
            .flex()
            .flex_row()
            .gap_4()
            .child(
                self.button(false).child(
                    svg()
                        .path("svg/plus.svg")
                        .size_8()
                        .text_color(rgb(0x545454)),
                ),
            )
            .child(
                self.button(true)
                    .child("START")
                    .text_color(white())
                    .flex_grow()
                    .text_center()
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_action_button)),
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .bg(rgb(0x090706))
            .size_full()
            .p_10()
            .flex()
            .flex_col()
            .gap_4()
            .justify_around()
            .items_center()
            .children([self.timer_widget(), self.bottom_button_row(cx).w_full()]);
    }
}
