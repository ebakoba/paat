mod header;
mod mocks;

pub use header::AppHeader;

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentId {
    Header,
}
