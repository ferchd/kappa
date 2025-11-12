pub mod core;
pub mod features;
pub mod state;
pub mod input;
pub mod ui;
pub mod utils;

pub use core::buffer::RopeBuffer;
pub use core::cursor::CursorPosition;
pub use core::document::Document;
pub use state::{EditorMode, EditorState};
pub use input::{InputAction, InputHandler};