use colored::Colorize;

pub fn print_item(path: &str, is_repo: bool) {
    if !is_repo {
        println!("{}", path.white());
        return;
    }
    println!("{}", path.red());
}

pub fn print_item_with_info(path: &str, is_repo: bool, info: String) {
    if !is_repo {
        println!("{}", path.white());
        return;
    }

    println!("{:<30}{}", path.red(), info);
}
