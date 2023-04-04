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
