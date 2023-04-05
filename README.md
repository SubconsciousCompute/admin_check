# admin_check

Check if your program has admin access. 

# Supported platforms
Windows
OS X
Linux (Not tested)

## Example

```rust
use admin_check::is_elevated;

fn main() {
    if !is_elevated() {
        println!(
            "Warning: the program isn’t running as elevated; some functionality may not work."
        );
    } else {
        println!("The program has admin access");
    }
}
```

## References

- [stackoverflow: How to check if a process has the administrative rights](https://stackoverflow.com/questions/8046097/how-to-check-if-a-process-has-the-administrative-rights)
- [rust-lang: How do I determine if I have admin rights on Windows](https://users.rust-lang.org/t/how-do-i-determine-if-i-have-admin-rights-on-windows/35710)
- [yandexx/is_elevated](https://github.com/yandexx/is_elevated)