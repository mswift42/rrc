fn is_valid(r: f64, g: f64, b: f64) -> bool {
    0.0 <= r && r <= 1.0 && 0.0 <= g && g <= 1.0 && 0.0 <= b && b <= 1.0
}

#[cfg(test)]
mod test {
    #[test]
    fn is_valid() {
        assert_eq!(super::is_valid(0.0, 0.0, 0.0), true);
        assert_eq!(super::is_valid(1.0, 1.0, 1.0), true);
        assert_eq!(super::is_valid(1.1, 0.0, 0.0), false);
        assert_eq!(super::is_valid(1.0, 1.1, 1.0), false);
        assert_eq!(super::is_valid(1.0, 1.0, 0.0), true);
        assert_eq!(super::is_valid(1.0, 0.0, 1.0), true);
        assert_eq!(super::is_valid(1.0, 1.0, 1.1), false);
    }
}