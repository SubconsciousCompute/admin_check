#[cfg(windows)]
mod win;

#[cfg(unix)]
mod unix;

pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        win::is_elevated()
    }
    #[cfg(unix)]
    {
        unix::is_elevated()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_should_not_be_elevated() {
        assert_eq!(crate::is_elevated(), false)
    }
}
