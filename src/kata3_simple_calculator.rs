// ============================================
// Kata 3: Simple Calculator
// Focus: Error Handling, Result Type, Custom Errors
// ============================================

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    Overflow,
    InvalidInput(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::Overflow => write!(f, "Arithmetic overflow"),
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for CalculatorError {}

pub struct Calculator;

impl Calculator {
    /// Safe addition operation
    pub fn add(a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a + b;
        if result.is_infinite() {
            Err(CalculatorError::Overflow)
        } else {
            Ok(result)
        }
    }

    /// Safe subtraction operation
    pub fn subtract(a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a - b;
        if result.is_infinite() {
            Err(CalculatorError::Overflow)
        } else {
            Ok(result)
        }
    }

    /// Safe multiplication operation
    pub fn multiply(a: f64, b: f64) -> Result<f64, CalculatorError> {
        let result = a * b;
        if result.is_infinite() {
            Err(CalculatorError::Overflow)
        } else {
            Ok(result)
        }
    }

    /// Safe division operation
    pub fn divide(dividend: f64, divisor: f64) -> Result<f64, CalculatorError> {
        if divisor == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            let result = dividend / divisor;
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else {
                Ok(result)
            }
        }
    }

    /// Parse and compute a simple expression
    pub fn evaluate(expression: &str) -> Result<f64, CalculatorError> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        if parts.len() != 3 {
            return Err(CalculatorError::InvalidInput(
                "Expected format: <number> <operator> <number>".to_string()
            ));
        }

        let a = parts[0].parse::<f64>()
            .map_err(|_| CalculatorError::InvalidInput("Invalid first number".to_string()))?;
        let b = parts[2].parse::<f64>()
            .map_err(|_| CalculatorError::InvalidInput("Invalid second number".to_string()))?;

        match parts[1] {
            "+" => Self::add(a, b),
            "-" => Self::subtract(a, b),
            "*" => Self::multiply(a, b),
            "/" => Self::divide(a, b),
            op => Err(CalculatorError::InvalidInput(format!("Unknown operator: {}", op))),
        }
    }
}

#[cfg(test)]
mod calculator_tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        assert_eq!(Calculator::divide(10.0, 0.0), Err(CalculatorError::DivisionByZero));
    }

    #[test]
    fn test_valid_operations() {
        assert_eq!(Calculator::add(2.0, 3.0), Ok(5.0));
        assert_eq!(Calculator::divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_evaluate_expression() {
        assert_eq!(Calculator::evaluate("10 + 5"), Ok(15.0));
        assert_eq!(Calculator::evaluate("10 / 0"), Err(CalculatorError::DivisionByZero));
    }
}
