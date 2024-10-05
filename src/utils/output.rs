use colored::Colorize;

pub trait Output: Clone {
    fn print_dir(self, path: &str) {
        println!("{}", path.white());
    }

    fn print_repo(self, path: &str) {
        println!("{}", path.red());
    }

    fn print_item_with_info(self, path: &str, info: String) {
        println!("{:<30}{}", path.red(), info);
    }
}
