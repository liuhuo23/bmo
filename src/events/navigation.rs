#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Timer,
    Settings,
    // PresetEdit,
}

#[derive(Debug, Clone, Copy)]
pub struct NavigationEvent {
    pub screen: Screen,
}
