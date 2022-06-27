use crate::{localization::fl, messages::Message};

use super::mocks::Calendar;
use tuirealm::{Component, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct DepartureDate {
    component: Calendar,
}

impl DepartureDate {
    pub fn new() -> Self {
        Self {
            component: Calendar::default().calendar_title(fl!("departure-date")),
        }
    }
}

impl Component<Message, NoUserEvent> for DepartureDate {
    fn on(&mut self, _: tuirealm::Event<NoUserEvent>) -> Option<Message> {
        None
    }
}
