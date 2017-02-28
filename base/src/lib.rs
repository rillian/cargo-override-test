pub fn boom() -> u32 {
    let edge = u32::max_value();
    edge + 1
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn it_works() {
        assert_eq!(0u32, ::boom());
    }
}
