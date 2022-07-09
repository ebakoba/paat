use super::{close_event_matcher, mocks::Calendar};
use crate::{localization::fl, messages::Message};
use paat_core::datetime::{get_current_date, naive_date_to_output_string};
use tuirealm::{Component, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct DepartureDate {
    component: Calendar,
}

impl DepartureDate {
    pub fn new() -> Self {
        let current_date = get_current_date();
        Self {
            component: Calendar::default()
                .calendar_date(naive_date_to_output_string(&current_date))
                .calendar_title(fl!("departure-date")),
        }
    }
}

impl Component<Message, NoUserEvent> for DepartureDate {
    fn on(&mut self, event: tuirealm::Event<NoUserEvent>) -> Option<Message> {
        close_event_matcher(event, |_| None)
    }
}
