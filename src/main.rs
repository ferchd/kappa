use crossterm::{
    event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kappa::{EditorState, InputAction, InputHandler};
use kappa::core::document::{Document, FileIO};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args: Vec<String> = env::args().collect();
    let document = if let Some(filename) = args.get(1) {
        FileIO::load_from_file(filename)?
    } else {
        Document::new()
    };

    let mut state = EditorState::new(document);
    state.viewport_mut().set_height(terminal.size()?.height as usize - 2);

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut EditorState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| kappa::ui::render(f, state))?;

        let event = event::read()?;
        if let Some(action) = InputHandler::handle(event, state)? {
            match action {
                InputAction::Quit => break,
                InputAction::Save => {}
                InputAction::Continue => {}
            }
        }
    }
    Ok(())
}