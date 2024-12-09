pub mod vector;
pub mod vector3;

pub use vector::Vector;
pub use vector3::Vector3;

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_function() {
        let result = 42;
        assert_eq!(result, 42);
    }
}
