// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
impl std::ops::Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, other: WrappingU32) -> Self::Output {
        Self { value: self.value.wrapping_add(other.value) }
    }
}

impl Clone for WrappingU32 {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl Copy for WrappingU32 {}

impl PartialEq for WrappingU32 {
    fn eq(&self, other: &WrappingU32) -> bool {
        self.value == other.value
    }
}

#[derive(Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
