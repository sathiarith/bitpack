#[cfg(test)]
mod tests {
    use crate::bitpack::fitsu;
    use crate::bitpack::fitss;
    use rand::Rng;
    
    #[test]
    fn fitss_twobits() {
        assert_eq!(fitss(-2,2), true);
        assert_eq!(fitss(-1,2), true);
        assert_eq!(fitss(0,2), true);
        assert_eq!(fitss(1,2), true);
        assert_eq!(fitss(2,2), false);
        assert_eq!(fitss(3,2), false);
    }
    #[test]
    fn fitsu_twobits() {
        assert_eq!(fitsu(0,2), true);
        assert_eq!(fitsu(1,2), true);
        assert_eq!(fitsu(2,2), true);
        assert_eq!(fitsu(3,2), true);
        assert_eq!(fitsu(4,2), false);
        assert_eq!(fitsu(5,2), false);
    }
}