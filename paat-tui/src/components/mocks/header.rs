use self::attributes::{GUIDE_TEXT, GUIDE_TITLE};
use crate::ascii::static_art::HEADER_TITLE;
use tui_realm_stdlib::utils::get_block;
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, Borders, Color, Style, TextModifiers},
    tui::{
        layout::{Constraint, Direction, Layout, Rect},
        widgets::Paragraph,
    },
    AttrValue, Attribute, Frame, MockComponent, Props, State,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Header {
    props: Props,
}

mod attributes {
    pub const GUIDE_TITLE: &str = "GUIDE_TITLE";
    pub const GUIDE_TEXT: &str = "GUID_TEXT";
}

impl Header {
    pub fn guide_title<S>(mut self, guide_title: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::GUIDE_TITLE),
            AttrValue::String(guide_title.as_ref().to_string()),
        );
        self
    }

    pub fn guide_text<S>(mut self, guide_text: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::GUIDE_TEXT),
            AttrValue::String(guide_text.as_ref().to_string()),
        );
        self
    }

    fn render_guide(&self, frame: &mut Frame, area: Rect) {
        let guide_title = self
            .props
            .get(Attribute::Custom(GUIDE_TITLE))
            .unwrap()
            .unwrap_string();
        let guide_text = self
            .props
            .get(Attribute::Custom(GUIDE_TEXT))
            .unwrap()
            .unwrap_string();
        frame.render_widget(
            Paragraph::new(guide_text)
                .block(get_block(
                    Borders::default(),
                    Some((guide_title, Alignment::Left)),
                    false,
                    None,
                ))
                .style(Style::default())
                .alignment(Alignment::Left),
            area,
        );
    }
}

impl MockComponent for Header {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();

            let vertical_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Length(82), Constraint::Min(0)].as_ref())
                .split(area);
            frame.render_widget(
                Paragraph::new(HEADER_TITLE)
                    .block(get_block(borders, Some(title), false, None))
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(TextModifiers::BOLD),
                    )
                    .alignment(alignment),
                vertical_chunks[0],
            );
            self.render_guide(frame, vertical_chunks[1]);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}
