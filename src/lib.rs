#[cfg(windows)]
mod win;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::is_elevated;

#[cfg(windows)]
pub use win::is_elevated;

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(unix)]
    fn test_should_not_be_elevated() {
        assert_eq!(crate::is_elevated(), false)
    }
}
