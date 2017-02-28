pub fn boom() -> u32 {
    let edge = u32::max_value();
    edge + 1
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_checking() {
        let v = ::boom();
        assert_eq!(0u32, v);
    }
}
