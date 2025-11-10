use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;
use kappa::{ui, Editor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).cloned();

    let mut editor = Editor::new(filename)?;

    let result = run_app(&mut terminal, &mut editor);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    editor: &mut Editor,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::render(f, editor))?;

        if let Event::Key(key) = event::read()? {
            if let Some(action) = kappa::input::handle_input(Event::Key(key), editor)? {
                match action {
                    kappa::input::Action::Quit => break,
                    kappa::input::Action::Save => {
                        if let Err(e) = editor.save() {
                            editor.set_message(&format!("Save failed: {}", e));
                        }
                    }
                    kappa::input::Action::Continue => {}
                }
            }
        }
    }
    Ok(())
}
