use crate::core::cursor::CursorPosition;
use crate::core::document::Document;
use crate::core::viewport::Viewport;
use crate::features::command_palette::{Command, CommandRegistry};
use crate::state::{EditorMode, MessageState};

pub struct EditorState {
    document: Document,
    cursor: CursorPosition,
    viewport: Viewport,
    mode: EditorMode,
    message: MessageState,
    command_input: String,
    command_registry: CommandRegistry,
    filtered_commands: Vec<Command>,
}

impl EditorState {
    pub fn new(document: Document) -> Self {
        let command_registry = CommandRegistry::new();
        Self {
            document,
            cursor: CursorPosition::new(),
            viewport: Viewport::new(20),
            mode: EditorMode::Normal,
            message: MessageState::new("Ctrl+P: Command Palette | Ctrl+S: Save | Ctrl+Q: Quit"),
            command_input: String::new(),
            command_registry,
            filtered_commands: Vec::new(),
        }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    pub fn cursor(&self) -> &CursorPosition {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut CursorPosition {
        &mut self.cursor
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn message(&self) -> &MessageState {
        &self.message
    }

    pub fn message_mut(&mut self) -> &mut MessageState {
        &mut self.message
    }

    pub fn command_input(&self) -> &str {
        &self.command_input
    }

    pub fn set_command_input(&mut self, input: String) {
        self.command_input = input;
    }

    pub fn push_command_input(&mut self, ch: char) {
        self.command_input.push(ch);
    }

    pub fn pop_command_input(&mut self) {
        self.command_input.pop();
    }

    pub fn clear_command_input(&mut self) {
        self.command_input.clear();
    }

    pub fn filtered_commands(&self) -> &[Command] {
        &self.filtered_commands
    }

    pub fn set_filtered_commands(&mut self, commands: Vec<Command>) {
        self.filtered_commands = commands;
    }

    pub fn command_registry(&self) -> &CommandRegistry {
        &self.command_registry
    }

    pub fn replace_document(&mut self, document: Document) {
        self.document = document;
        self.cursor = CursorPosition::new();
        self.viewport = Viewport::new(self.viewport.height());
    }

    pub fn buffer_and_cursor_mut(&mut self) -> (&mut crate::core::buffer::RopeBuffer, &mut CursorPosition) {
        (self.document.buffer_mut(), &mut self.cursor)
    }

    pub fn buffer_cursor_and_viewport_mut(&mut self) -> (&mut crate::core::buffer::RopeBuffer, &mut CursorPosition, &mut Viewport) {
        (self.document.buffer_mut(), &mut self.cursor, &mut self.viewport)
    }

    pub fn cursor_and_viewport_mut(&mut self) -> (&mut CursorPosition, &mut Viewport) {
        (&mut self.cursor, &mut self.viewport)
    }

    pub fn buffer_cursor_viewport(&mut self) -> (&crate::core::buffer::RopeBuffer, &mut CursorPosition, &mut Viewport) {
        (self.document.buffer(), &mut self.cursor, &mut self.viewport)
    }

    pub fn buffer_cursor(&mut self) -> (&crate::core::buffer::RopeBuffer, &mut CursorPosition) {
        (self.document.buffer(), &mut self.cursor)
    }
}