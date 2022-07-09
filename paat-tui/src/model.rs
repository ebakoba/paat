use crate::{
    components::{AppHeader, ComponentId, DepartureDate},
    messages::Message,
};
use std::time::Duration;
use tuirealm::{
    event::NoUserEvent,
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, EventListenerCfg, Update,
};

pub struct Model {
    pub app: Application<ComponentId, Message, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        let app = &mut self.app;
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(14), Constraint::Length(14)].as_ref())
                    .split(f.size());
                let input_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Ratio(1, 3),
                            Constraint::Ratio(1, 3),
                            Constraint::Ratio(1, 3),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
                app.view(&ComponentId::Header, f, chunks[0]);
                app.view(&ComponentId::DepartureDate, f, input_chunks[0]);
            })
            .is_ok());
    }

    fn init_app() -> Application<ComponentId, Message, NoUserEvent> {
        let mut app: Application<ComponentId, Message, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        assert!(app
            .mount(ComponentId::Header, Box::new(AppHeader::new()), vec![])
            .is_ok());
        assert!(app
            .mount(
                ComponentId::DepartureDate,
                Box::new(DepartureDate::new()),
                vec![]
            )
            .is_ok());

        assert!(app.active(&ComponentId::DepartureDate).is_ok());
        app
    }
}

impl Update<Message> for Model {
    fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Message::AppClose => {
                    self.quit = true;
                    None
                }
            }
        } else {
            None
        }
    }
}
