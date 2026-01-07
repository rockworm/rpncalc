use rpncalc::*;

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

#[test]
fn test_ln() {
    let mut app = App::new();
    app.stack = vec![std::f64::consts::E];
    app.input = "ln".to_string();
    app.execute_command();
    assert!((app.stack[0] - 1.0).abs() < 1e-10);
}

#[test]
fn test_log() {
    let mut app = App::new();
    app.stack = vec![100.0];
    app.input = "log".to_string();
    app.execute_command();
    assert!((app.stack[0] - 2.0).abs() < 1e-10);
}

#[test]
fn test_exp() {
    let mut app = App::new();
    app.stack = vec![1.0];
    app.input = "exp".to_string();
    app.execute_command();
    assert!((app.stack[0] - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_10x() {
    let mut app = App::new();
    app.stack = vec![2.0];
    app.input = "10x".to_string();
    app.execute_command();
    assert_eq!(app.stack, vec![100.0]);
}

#[test]
fn test_abs() {
    let mut app = App::new();
    app.stack = vec![-5.0];
    app.input = "abs".to_string();
    app.execute_command();
    assert_eq!(app.stack, vec![5.0]);
}

#[test]
fn test_cbrt() {
    let mut app = App::new();
    app.stack = vec![8.0];
    app.input = "cbrt".to_string();
    app.execute_command();
    assert!((app.stack[0] - 2.0).abs() < 1e-10);
}

#[test]
fn test_root() {
    let mut app = App::new();
    app.stack = vec![8.0, 3.0]; // 3rd root of 8
    app.input = "root".to_string();
    app.execute_command();
    assert!((app.stack[0] - 2.0).abs() < 1e-10);
}

#[test]
fn test_root_zero() {
    let mut app = App::new();
    app.stack = vec![8.0, 0.0];
    app.input = "root".to_string();
    app.execute_command();
    assert_eq!(app.stack, vec![8.0, 0.0]);
    assert!(app.message.contains("Cannot take 0th root"));
}

#[test]
fn test_root_insufficient_stack() {
    let mut app = App::new();
    app.stack = vec![8.0];
    app.input = "root".to_string();
    app.execute_command();
    assert_eq!(app.stack, vec![8.0]);
    assert!(app.message.contains("Need 2 numbers"));
}