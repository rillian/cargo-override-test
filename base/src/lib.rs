pub fn boom(edge: u32) -> u32 {
    edge + 1
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_checking() {
        let v = ::boom(u32::max_value());
        assert_eq!(0u32, v);
    }
}
