use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io};
use rpncalc::App;

const VERSION: &str = git_version::git_version!(fallback = env!("CARGO_PKG_VERSION"));

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.show_help {
                app.show_help = false;
                app.message = "Help closed".to_string();
                continue;
            }
            
            match key.code {
                KeyCode::Char('q') if app.input.is_empty() => return Ok(()),
                KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
                    app.input.push(c);
                }
                KeyCode::Char(c @ ('+' | '-' | '*' | '/' | '^' | '%' | '!')) => {
                    app.execute_single_char(c);
                }
                KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                    app.input.push(c);
                }
                KeyCode::Enter => {
                    app.execute_command();
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.clear();
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(main_chunks[0]);

    let title = Paragraph::new(format!("RPN Calculator {}", VERSION))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, left_chunks[0]);

    let stack_items: Vec<ListItem> = app
        .stack
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            ListItem::new(Line::from(Span::raw(format!("{}: {}", i, val))))
        })
        .collect();

    let stack = List::new(stack_items)
        .block(Block::default().borders(Borders::ALL).title("Stack"))
        .style(Style::default().fg(Color::White));
    f.render_widget(stack, left_chunks[1]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(input, left_chunks[2]);

    let message = Paragraph::new(app.message.as_str())
        .block(Block::default().borders(Borders::ALL).title("Message"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(message, left_chunks[3]);

    let history_items: Vec<ListItem> = app
        .calc_history
        .iter()
        .map(|calc| ListItem::new(Line::from(Span::raw(calc))))
        .collect();

    let history = List::new(history_items)
        .block(Block::default().borders(Borders::ALL).title("History"))
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(history, main_chunks[1]);

    if app.show_help {
        let version_str = format!("Version {}", VERSION);
        let help_text = vec![
            "RPN Calculator Help",
            &version_str,
            "",
            "Basic Operations:",
            "  +, -, *, /, ^, %",
            "",
            "Trigonometry:",
            "  sin, cos, tan",
            "  asin, acos, atan",
            "",
            "Other Math:",
            "  sqrt, inv (1/x), ! (factorial)",
            "",
            "Stack Operations:",
            "  swap, drop, clear/clr, undo",
            "",
            "Press any key to close"
        ];

        let help_lines: Vec<Line> = help_text
            .iter()
            .map(|&text| Line::from(Span::raw(text)))
            .collect();

        let help_paragraph = Paragraph::new(help_lines)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Color::White).bg(Color::Blue));

        let area = f.size();
        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area)[1];

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(popup_area)[1];

        f.render_widget(help_paragraph, popup_area);
    }
}