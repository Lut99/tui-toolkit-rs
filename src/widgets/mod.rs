//  MOD.rs
//    by Lut99
//
//  Created:
//    21 Jan 2025, 20:25:31
//  Last edited:
//    21 Jan 2025, 20:57:03
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines some commonly used, yet custom widgets for the [`ratatui`]
//!   library.
//

// Declare the widget modules
pub mod scroll_area;

// Use some of it
pub use scroll_area::{ScrollArea, StatefulScrollArea};
