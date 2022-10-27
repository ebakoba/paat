use crate::{localization::fl, messages::Message, ports::ApiEvent};

use super::mocks::Header;
use tuirealm::{Component, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct AppHeader {
    component: Header,
}

impl AppHeader {
    pub fn new() -> Self {
        Self {
            component: Header::default()
                .guide_title(fl!("usage-guide-title"))
                .guide_text(fl!("usage-guide-text")),
        }
    }
}

impl Component<Message, ApiEvent> for AppHeader {
    fn on(&mut self, _: tuirealm::Event<ApiEvent>) -> Option<Message> {
        None
    }
}
