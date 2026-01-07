pub struct App {
    pub stack: Vec<f64>,
    pub input: String,
    pub message: String,
    pub history: Vec<Vec<f64>>,
    pub calc_history: Vec<String>,
    pub show_help: bool,
}

impl App {
    pub fn new() -> App {
        App {
            stack: Vec::new(),
            input: String::new(),
            message: "Type numbers or commands (help for list), Enter to execute, q to quit".to_string(),
            history: Vec::new(),
            calc_history: Vec::new(),
            show_help: false,
        }
    }

    pub fn execute_command(&mut self) {
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
                        "ln" => self.unary_op(|a| a.ln(), "ln"),
                        "log" => self.unary_op(|a| a.log10(), "log"),
                        "exp" => self.unary_op(|a| a.exp(), "exp"),
                        "10x" => self.unary_op(|a| 10.0_f64.powf(a), "10x"),
                        "abs" => self.unary_op(|a| a.abs(), "abs"),
                        "cbrt" => self.unary_op(|a| a.cbrt(), "cbrt"),
                        "root" => self.root(),
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

    pub fn binary_op<F>(&mut self, op: F, name: &str)
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
    
    pub fn unary_op<F>(&mut self, op: F, name: &str)
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
    
    pub fn divide(&mut self) {
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
    
    pub fn reciprocal(&mut self) {
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
    
    pub fn factorial(&mut self) {
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
    
    pub fn swap(&mut self) {
        if self.stack.len() < 2 {
            self.message = "Need 2 numbers to swap".to_string();
        } else {
            let len = self.stack.len();
            self.stack.swap(len - 1, len - 2);
            self.message = "Swapped top 2 values".to_string();
        }
    }

    pub fn execute_single_char(&mut self, c: char) {
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

    pub fn clear(&mut self) {
        self.stack.clear();
        self.message = "Stack cleared".to_string();
    }

    fn root(&mut self) {
        if self.stack.len() < 2 {
            self.message = "Need 2 numbers for root (y root x = x^(1/y))".to_string();
            return;
        }
        let y = self.stack.pop().unwrap(); // root index
        let x = self.stack.pop().unwrap(); // base
        if y == 0.0 {
            self.stack.push(x);
            self.stack.push(y);
            self.message = "Cannot take 0th root".to_string();
        } else {
            let result = x.powf(1.0 / y);
            self.stack.push(result);
            let calc = format!("{} root {} = {}", y, x, result);
            self.message = calc.clone();
            self.calc_history.push(calc);
            if self.calc_history.len() > 10 {
                self.calc_history.remove(0);
            }
        }
    }
}