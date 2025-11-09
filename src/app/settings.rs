use gpui::{Context, EventEmitter, ParentElement, Render, Styled, div};
use gpui_component::{
    Icon, TitleBar,
    button::{Button, ButtonVariants},
};

use crate::events::navigation::{NavigationEvent, Screen};

pub struct SettingScreen;

impl EventEmitter<NavigationEvent> for SettingScreen {}

impl SettingScreen {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        return SettingScreen;
    }
}

impl Render for SettingScreen {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        return div().size_full().flex().flex_col().child(
            TitleBar::new().child(div().child("Settings")).child(
                div().flex().items_center().gap_2().child(
                    Button::new("settings")
                        .icon(Icon::new(Icon::empty()).path("icons/x.svg"))
                        .ghost()
                        .on_click(cx.listener(|_this, _event, _window, cx| {
                            cx.emit(NavigationEvent {
                                screen: Screen::Timer,
                            });
                        })),
                ),
            ),
        );
    }
}
