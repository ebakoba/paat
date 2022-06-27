mod departure_date;
mod header;
mod mocks;

pub use departure_date::DepartureDate;
pub use header::AppHeader;

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentId {
    Header,
    Calendar,
}
