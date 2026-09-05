pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    sigmoid(x) * (1.0 - sigmoid(x))
}

pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

pub fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        let result = sigmoid(0.0);
        // sigmoid(0) should be exactly 0.5
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_sigmoid_derivative() {
        let result = sigmoid_derivative(0.0);
        // derivative at 0 is 0.5 * (1 - 0.5) = 0.25
        assert_eq!(result, 0.25);
    }

    #[test]
    fn test_relu() {
        assert_eq!(relu(-5.0), 0.0);
        assert_eq!(relu(0.0), 0.0);
        assert_eq!(relu(5.0), 5.0);
    }

    #[test]
    fn test_relu_derivative() {
        assert_eq!(relu_derivative(-5.0), 0.0);
        assert_eq!(relu_derivative(0.0), 0.0);
        assert_eq!(relu_derivative(5.0), 1.0);
    }
}
