
#[path = "point.rs"]
pub mod point;

#[path = "table.rs"]
pub mod table;

pub use point::Point;
pub use table::Table;

pub mod prelude {
    pub use crate::point::Point;
    pub use crate::table::Table;
}
