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
    calc_history: Vec<String>,
    show_help: bool,
}

impl App {
    fn new() -> App {
        App {
            stack: Vec::new(),
            input: String::new(),
            message: "Type numbers or commands (help for list), Enter to execute, q to quit".to_string(),
            history: Vec::new(),
            calc_history: Vec::new(),
            show_help: false,
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
                "help" => {
                    self.show_help = true;
                    self.message = "Help shown (press any key to close)".to_string();
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
                        "asin" => self.unary_op(|a| a.asin().to_degrees(), "asin"),
                        "acos" => self.unary_op(|a| a.acos().to_degrees(), "acos"),
                        "atan" => self.unary_op(|a| a.atan().to_degrees(), "atan"),
                        "sqrt" => self.unary_op(|a| a.sqrt(), "sqrt"),
                        "inv" => self.reciprocal(),
                        "!" | "fact" => self.factorial(),
                        "swap" => self.swap(),
                        "clear" | "clr" => {
                            self.stack.clear();
                            self.message = "Stack cleared".to_string();
                        },
                        "drop" => {
                            if let Some(val) = self.stack.pop() {
                                self.message = format!("Dropped {}", val);
                            } else {
                                self.message = "Stack is empty".to_string();
                            }
                        },
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
        let calc = format!("{} {} {} = {}", a, name, b, result);
        self.message = calc.clone();
        self.calc_history.push(calc);
        if self.calc_history.len() > 10 {
            self.calc_history.remove(0);
        }
    }
    
    fn unary_op<F>(&mut self, op: F, name: &str)
    where
        F: Fn(f64) -> f64,
    {
        if let Some(a) = self.stack.pop() {
            let result = op(a);
            self.stack.push(result);
            let calc = format!("{}({}) = {}", name, a, result);
            self.message = calc.clone();
            self.calc_history.push(calc);
            if self.calc_history.len() > 10 {
                self.calc_history.remove(0);
            }
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
            let calc = format!("{} / {} = {}", a, b, a / b);
            self.message = calc.clone();
            self.calc_history.push(calc);
            if self.calc_history.len() > 10 {
                self.calc_history.remove(0);
            }
        }
    }
    
    fn reciprocal(&mut self) {
        if let Some(a) = self.stack.pop() {
            if a == 0.0 {
                self.stack.push(a);
                self.message = "Cannot take reciprocal of zero".to_string();
            } else {
                let result = 1.0 / a;
                self.stack.push(result);
                let calc = format!("1/{} = {}", a, result);
                self.message = calc.clone();
                self.calc_history.push(calc);
                if self.calc_history.len() > 10 {
                    self.calc_history.remove(0);
                }
            }
        } else {
            self.message = "Need 1 number for reciprocal".to_string();
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
                let calc = format!("{}! = {}", n, result);
                self.message = calc.clone();
                self.calc_history.push(calc);
                if self.calc_history.len() > 10 {
                    self.calc_history.remove(0);
                }
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
    
    fn clear(&mut self) {
        self.stack.clear();
        self.message = "Stack cleared".to_string();
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

    let title = Paragraph::new("RPN Calculator")
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
        let help_text = vec![
            "RPN Calculator Help",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_number() {
        let mut app = App::new();
        app.input = "42.5".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![42.5]);
    }

    #[test]
    fn test_addition() {
        let mut app = App::new();
        app.stack = vec![3.0, 4.0];
        app.input = "+".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![7.0]);
    }

    #[test]
    fn test_subtraction() {
        let mut app = App::new();
        app.stack = vec![10.0, 3.0];
        app.input = "-".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![7.0]);
    }

    #[test]
    fn test_multiplication() {
        let mut app = App::new();
        app.stack = vec![3.0, 4.0];
        app.input = "*".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![12.0]);
    }

    #[test]
    fn test_division() {
        let mut app = App::new();
        app.stack = vec![12.0, 3.0];
        app.input = "/".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![4.0]);
    }

    #[test]
    fn test_division_by_zero() {
        let mut app = App::new();
        app.stack = vec![5.0, 0.0];
        app.input = "/".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![5.0, 0.0]);
        assert!(app.message.contains("Division by zero"));
    }

    #[test]
    fn test_power() {
        let mut app = App::new();
        app.stack = vec![2.0, 3.0];
        app.input = "^".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![8.0]);
    }

    #[test]
    fn test_modulo() {
        let mut app = App::new();
        app.stack = vec![10.0, 3.0];
        app.input = "%".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0]);
    }

    #[test]
    fn test_sqrt() {
        let mut app = App::new();
        app.stack = vec![16.0];
        app.input = "sqrt".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![4.0]);
    }

    #[test]
    fn test_reciprocal() {
        let mut app = App::new();
        app.stack = vec![4.0];
        app.input = "inv".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![0.25]);
    }

    #[test]
    fn test_reciprocal_zero() {
        let mut app = App::new();
        app.stack = vec![0.0];
        app.input = "inv".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![0.0]);
        assert!(app.message.contains("Cannot take reciprocal of zero"));
    }

    #[test]
    fn test_factorial() {
        let mut app = App::new();
        app.stack = vec![5.0];
        app.input = "!".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![120.0]);
    }

    #[test]
    fn test_factorial_negative() {
        let mut app = App::new();
        app.stack = vec![-1.0];
        app.input = "!".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![-1.0]);
        assert!(app.message.contains("non-negative integer"));
    }

    #[test]
    fn test_sin() {
        let mut app = App::new();
        app.stack = vec![90.0];
        app.input = "sin".to_string();
        app.execute_command();
        assert!((app.stack[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        let mut app = App::new();
        app.stack = vec![0.0];
        app.input = "cos".to_string();
        app.execute_command();
        assert!((app.stack[0] - 1.0).abs() < 1e-10);
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
    fn test_swap_insufficient() {
        let mut app = App::new();
        app.stack = vec![1.0];
        app.input = "swap".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0]);
        assert!(app.message.contains("Need 2 numbers"));
    }

    #[test]
    fn test_drop() {
        let mut app = App::new();
        app.stack = vec![1.0, 2.0, 3.0];
        app.input = "drop".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0, 2.0]);
    }

    #[test]
    fn test_drop_empty() {
        let mut app = App::new();
        app.input = "drop".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![]);
        assert!(app.message.contains("Stack is empty"));
    }

    #[test]
    fn test_clear() {
        let mut app = App::new();
        app.stack = vec![1.0, 2.0, 3.0];
        app.input = "clear".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![]);
    }

    #[test]
    fn test_undo() {
        let mut app = App::new();
        app.stack = vec![1.0, 2.0];
        app.history.push(vec![1.0]);
        app.input = "undo".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0]);
    }

    #[test]
    fn test_undo_empty_history() {
        let mut app = App::new();
        app.stack = vec![1.0];
        app.input = "undo".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0]);
        assert!(app.message.contains("Nothing to undo"));
    }

    #[test]
    fn test_binary_op_insufficient_stack() {
        let mut app = App::new();
        app.stack = vec![1.0];
        app.input = "+".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![1.0]);
        assert!(app.message.contains("Need 2 numbers"));
    }

    #[test]
    fn test_unary_op_empty_stack() {
        let mut app = App::new();
        app.input = "sqrt".to_string();
        app.execute_command();
        assert_eq!(app.stack, vec![]);
        assert!(app.message.contains("Need 1 number"));
    }

    #[test]
    fn test_unknown_command() {
        let mut app = App::new();
        app.input = "unknown".to_string();
        app.execute_command();
        assert!(app.message.contains("Unknown command"));
    }
}