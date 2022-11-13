use self::attributes::{ERROR_TEXT, ERROR_TITLE, GUIDE_TEXT, GUIDE_TITLE};
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

pub mod attributes {
    pub const GUIDE_TITLE: &str = "GUIDE_TITLE";
    pub const GUIDE_TEXT: &str = "GUID_TEXT";
    pub const ERROR_TITLE: &str = "ERROR_TITLE";
    pub const ERROR_TEXT: &str = "ERROR_TEXT";
}

impl Header {
    pub fn error_text<S>(mut self, error_text: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::ERROR_TEXT),
            AttrValue::String(error_text.as_ref().to_string()),
        );
        self
    }

    pub fn error_title<S>(mut self, error_title: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::ERROR_TITLE),
            AttrValue::String(error_title.as_ref().to_string()),
        );
        self
    }

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

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let alignment = self
            .props
            .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
            .unwrap_alignment();

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
        let foreground = self
            .props
            .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
            .unwrap_color();
        let background = self
            .props
            .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
            .unwrap_color();
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
            area,
        );
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

    fn render_error(&self, frame: &mut Frame, area: Rect) {
        let error_title = self
            .props
            .get(Attribute::Custom(ERROR_TITLE))
            .unwrap()
            .unwrap_string();
        let error_text = self
            .props
            .get(Attribute::Custom(ERROR_TEXT))
            .unwrap()
            .unwrap_string();
        frame.render_widget(
            Paragraph::new(error_text)
                .block(get_block(
                    Borders::default(),
                    Some((error_title, Alignment::Left)),
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
            let header_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Length(82), Constraint::Min(0)].as_ref())
                .split(area);
            let even_info_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(header_chunks[1]);

            self.render_header(frame, header_chunks[0]);
            self.render_guide(frame, even_info_chunks[0]);
            self.render_error(frame, even_info_chunks[1]);
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
