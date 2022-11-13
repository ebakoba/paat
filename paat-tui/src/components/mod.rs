mod departure_date;
mod header;
mod hidden_handler;
mod mocks;
mod select_ferry;
mod select_line;

pub use departure_date::DepartureDate;
pub use header::AppHeader;
pub use hidden_handler::HiddenHandler;
pub use mocks::HeaderAttributes;
pub use select_ferry::SelectFerry;
pub use select_line::SelectLine;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentId {
    Header,
    DepartureDate,
    SelectFerry,
    SelectLine,
    HiddenHandler,
}
