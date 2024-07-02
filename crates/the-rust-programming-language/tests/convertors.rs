#[cfg(test)]
mod prim {
    #[test]
    fn test_slice() {
        let x32 = 0_u32.to_be_bytes();
        println!("{}", x32.len());
        let slicen: &[u8; 36] = x;

        println!("{slicen}")
    }
}
