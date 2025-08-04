#[macro_export]
/// If the floats are essentially integers, make them the exact integers.
///
/// # Examples
///
/// ```
/// use rotchess_core::floating_drift::floating_drift_adjust;
/// let result = floating_drift_adjust!(1.000001, 2.5);
/// assert_eq!(result, (1.0, 2.5));
/// ```
macro_rules! floating_drift_adjust {
    ($($x:expr),+ $(,)?) => {{
        const EPSILON: f32 = 1e-4;

        // Helper function to round a value if it's close to an integer
        fn round_if_close_to_integer(val: f32) -> f32 {
            if (val - val.round()).abs() < EPSILON {
                val.round()
            } else {
                val
            }
        }

        // Return tuple with each value individually checked and possibly rounded
        ($(round_if_close_to_integer($x as f32)),+)
    }};
}

pub use floating_drift_adjust;

#[cfg(test)]
mod tests {
    #[test]
    fn values_close_to_integers() {
        let result = floating_drift_adjust!(1.000001, 2.999999, 3.0);
        assert_eq!(result, (1.0, 3.0, 3.0));
    }

    #[test]
    fn values_not_close_to_integers() {
        let result = floating_drift_adjust!(1.5, 2.7, 3.2);
        assert_eq!(result, (1.5, 2.7, 3.2));
    }

    #[test]
    fn mixed_close_and_not_close() {
        let result = floating_drift_adjust!(1.000001, 2.5, 3.999999);
        assert_eq!(result, (1.0, 2.5, 4.0));
    }

    #[test]
    fn mixed_integer_and_float_types() {
        let result = floating_drift_adjust!(1, 2.000001, 3.999999);
        assert_eq!(result, (1.0, 2.0, 4.0));
    }

    #[test]
    fn single_value_close_to_integer() {
        let result = floating_drift_adjust!(4.000001);
        assert_eq!(result, (4.0));
    }

    #[test]
    fn single_value_not_close_to_integer() {
        let result = floating_drift_adjust!(4.5);
        assert_eq!(result, (4.5));
    }

    #[test]
    fn many_values_all_close() {
        let result = floating_drift_adjust!(1.0, 2.000001, 3.999999, 4.0, 5.000001);
        assert_eq!(result, (1.0, 2.0, 4.0, 4.0, 5.0));
    }

    #[test]
    fn many_values_mixed() {
        let result = floating_drift_adjust!(1.000001, 2.5, 3.0, 4.7, 5.999999);
        assert_eq!(result, (1.0, 2.5, 3.0, 4.7, 6.0));
    }

    #[test]
    fn negative_values() {
        let result = floating_drift_adjust!(-1.000001, -2.999999);
        assert_eq!(result, (-1.0, -3.0));
    }

    #[test]
    fn zero_values() {
        let result = floating_drift_adjust!(0.000001, -0.000001);
        assert_eq!(result, (0.0, 0.0));
    }

    #[test]
    fn mixed_negative_and_positive() {
        let result = floating_drift_adjust!(-1.000001, 2.5, 3.999999);
        assert_eq!(result, (-1.0, 2.5, 4.0));
    }
}
