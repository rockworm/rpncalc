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

struct App {
    stack: Vec<f64>,
    input: String,
    message: String,
    history: Vec<Vec<f64>>,
}

impl App {
    fn new() -> App {
        App {
            stack: Vec::new(),
            input: String::new(),
            message: "Type numbers or commands (help for list), Enter to execute, q to quit".to_string(),
            history: Vec::new(),
        }
    }

    fn execute_single_char(&mut self, c: char) {
        if !self.input.is_empty() {
            self.execute_command();
        }
        
        self.history.push(self.stack.clone());
        
        match c {
            '+' => self.binary_op(|a, b| a + b, "+"),
            '-' => self.binary_op(|a, b| a - b, "-"),
            '*' => self.binary_op(|a, b| a * b, "*"),
            '/' => self.divide(),
            '^' => self.binary_op(|a, b| a.powf(b), "^"),
            '%' => self.binary_op(|a, b| a % b, "%"),
            '!' => self.factorial(),
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        if self.input.is_empty() {
            return;
        }
        
        if let Ok(num) = self.input.parse::<f64>() {
            self.history.push(self.stack.clone());
            self.stack.push(num);
            self.message = format!("Pushed {}", num);
        } else {
            match self.input.as_str() {
                "undo" => {
                    if let Some(prev_stack) = self.history.pop() {
                        self.stack = prev_stack;
                        self.message = "Undid last operation".to_string();
                    } else {
                        self.message = "Nothing to undo".to_string();
                    }
                },
                _ => {
                    self.history.push(self.stack.clone());
                    match self.input.as_str() {
                        "+" => self.binary_op(|a, b| a + b, "+"),
                        "-" => self.binary_op(|a, b| a - b, "-"),
                        "*" => self.binary_op(|a, b| a * b, "*"),
                        "/" => self.divide(),
                        "^" | "pow" => self.binary_op(|a, b| a.powf(b), "^"),
                        "%" | "mod" => self.binary_op(|a, b| a % b, "%"),
                        "sin" => self.unary_op(|a| a.to_radians().sin(), "sin"),
                        "cos" => self.unary_op(|a| a.to_radians().cos(), "cos"),
                        "tan" => self.unary_op(|a| a.to_radians().tan(), "tan"),
                        "!" | "fact" => self.factorial(),
                        "swap" => self.swap(),
                        "clear" => self.clear(),
                        "help" => self.help(),
                        _ => self.message = "Unknown command (type 'help' for list)".to_string(),
                    }
                }
            }
        }
        
        self.input.clear();
    }

    fn binary_op<F>(&mut self, op: F, name: &str)
    where
        F: Fn(f64, f64) -> f64,
    {
        if self.stack.len() < 2 {
            self.message = format!("Need 2 numbers for {}", name);
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = op(a, b);
        self.stack.push(result);
        self.message = format!("{} {} {} = {}", a, name, b, result);
    }
    
    fn unary_op<F>(&mut self, op: F, name: &str)
    where
        F: Fn(f64) -> f64,
    {
        if let Some(a) = self.stack.pop() {
            let result = op(a);
            self.stack.push(result);
            self.message = format!("{}({}) = {}", name, a, result);
        } else {
            self.message = format!("Need 1 number for {}", name);
        }
    }
    
    fn divide(&mut self) {
        if self.stack.len() < 2 {
            self.message = "Need 2 numbers for division".to_string();
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        if b == 0.0 {
            self.stack.push(a);
            self.stack.push(b);
            self.message = "Division by zero".to_string();
        } else {
            self.stack.push(a / b);
            self.message = format!("{} / {} = {}", a, b, a / b);
        }
    }
    
    fn factorial(&mut self) {
        if let Some(a) = self.stack.pop() {
            if a < 0.0 || a.fract() != 0.0 {
                self.stack.push(a);
                self.message = "Factorial needs non-negative integer".to_string();
            } else {
                let n = a as u64;
                let result = (1..=n).product::<u64>() as f64;
                self.stack.push(result);
                self.message = format!("{}! = {}", n, result);
            }
        } else {
            self.message = "Need 1 number for factorial".to_string();
        }
    }
    
    fn swap(&mut self) {
        if self.stack.len() < 2 {
            self.message = "Need 2 numbers to swap".to_string();
        } else {
            let len = self.stack.len();
            self.stack.swap(len - 1, len - 2);
            self.message = "Swapped top 2 values".to_string();
        }
    }
    
    fn undo(&mut self) {
        if let Some(prev_stack) = self.history.pop() {
            self.stack = prev_stack;
            self.message = "Undid last operation".to_string();
        } else {
            self.message = "Nothing to undo".to_string();
        }
    }
    
    fn clear(&mut self) {
        self.stack.clear();
        self.message = "Stack cleared".to_string();
    }
    
    fn help(&mut self) {
        self.message = "Commands: +,-,*,/,^,% sin,cos,tan,fact,swap,undo,clear,help".to_string();
    }
}

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
            match key.code {
                KeyCode::Char('q') => return Ok(()),
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("RPN Calculator")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

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
    f.render_widget(stack, chunks[1]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(input, chunks[2]);

    let message = Paragraph::new(app.message.as_str())
        .block(Block::default().borders(Borders::ALL).title("Message"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(message, chunks[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_number() {
        let mut app = App::new();
        app.input = "42".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![42.0]);
    }

    #[test]
    fn test_addition() {
        let mut app = App::new();
        app.stack = vec![5.0, 3.0];
        app.execute_single_char('+');
        assert_eq!(app.stack, vec![8.0]);
    }

    #[test]
    fn test_sin() {
        let mut app = App::new();
        app.stack = vec![90.0];
        app.input = "sin".to_string();
        app.execute_command();
        assert!((app.stack[0] - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_factorial() {
        let mut app = App::new();
        app.stack = vec![5.0];
        app.execute_single_char('!');
        assert_eq!(app.stack, vec![120.0]);
    }

    #[test]
    fn test_swap() {
        let mut app = App::new();
        app.stack = vec![1.0, 2.0];
        app.input = "swap".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![2.0, 1.0]);
    }

    #[test]
    fn test_undo() {
        let mut app = App::new();
        app.stack = vec![5.0, 3.0];
        app.execute_single_char('+');
        assert_eq!(app.stack, vec![8.0]);
        
        app.input = "undo".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![5.0, 3.0]);
    }
}