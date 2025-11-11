//! Constellation diagram utilities

use num_complex::Complex64;

/// Normalize QPSK constellation points for display
/// 
/// Scales constellation points to unit circle and extracts I/Q components
/// as separate f32 vectors suitable for plotting.
/// 
/// # Arguments
/// * `symbols` - IQ symbols to normalize
/// 
/// # Returns
/// Tuple of (I_values, Q_values) as f32 vectors
pub fn normalize_constellation(symbols: &[Complex64]) -> (Vec<f32>, Vec<f32>) {
    if symbols.is_empty() {
        return (Vec::new(), Vec::new());
    }
    
    let scale = 1.0 / std::f64::consts::SQRT_2;
    let i_vals: Vec<f32> = symbols.iter().map(|c| (c.re * scale) as f32).collect();
    let q_vals: Vec<f32> = symbols.iter().map(|c| (c.im * scale) as f32).collect();
    
    (i_vals, q_vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_constellation() {
        let symbols = vec![
            Complex64::new(0.707, 0.707),
            Complex64::new(-0.707, 0.707),
        ];
        
        let (i_vals, q_vals) = normalize_constellation(&symbols);
        
        assert_eq!(i_vals.len(), 2);
        assert_eq!(q_vals.len(), 2);
        
        // Check scaling is applied
        assert!((i_vals[0] - 0.5).abs() < 0.01);
        assert!((q_vals[0] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_empty_input_returns_empty() {
        let (i_vals, q_vals) = normalize_constellation(&[]);
        
        assert!(i_vals.is_empty());
        assert!(q_vals.is_empty());
    }
}
