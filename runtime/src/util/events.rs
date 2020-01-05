use amethyst::core::ecs::*;
use amethyst::core::shrev::{EventChannel, ReaderId};
use amethyst::core::EventReader;
use amethyst::ui::UiEvent;
use amethyst::winit::Event;

#[derive(Clone, EventReader)]
#[reader(AllEventsReader)]
pub enum AllEvents {
    Window(Event),
    Ui(UiEvent),
    Custom(CustomStateEvent),
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Copy)]
pub enum CustomStateEvent {
    // Actually a redirect to MapSelectState in this case.
    // TODO: Remove once TransQueue hits live
    GotoMainMenu,
    MapFinished,
    Retry,
}
