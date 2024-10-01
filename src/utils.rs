use regex::Regex;
use std::{fs, path::PathBuf};

pub fn home_dir_mark(path: &str) -> String {
    path.replace(
        dirs::home_dir()
            .expect("Error: failed to get home directory")
            .to_str()
            .unwrap(),
        "~",
    )
}

pub fn get_dir_items(path: &str) -> Vec<PathBuf> {
    let dir = fs::read_dir(path).unwrap();
    let mut files: Vec<PathBuf> = Vec::new();

    for item in dir.into_iter() {
        files.push(item.unwrap().path());
    }

    files
}

pub struct GitUrl {
    pub domain: String,
    pub user: String,
    pub repo: String,
}

pub fn get_git_url(url: &str) -> Option<GitUrl> {
    let git_re =
        Regex::new(r"git\@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}\:").unwrap();
    let https_re =
        Regex::new(r"^https?://([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}").unwrap();

    let domain_re = Regex::new(r"([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}").unwrap();
    let repo_re = Regex::new(r"(?<user>[a-zA-Z0-9-\.]*)/(?<repo>[a-zA-Z0-9-\.]*)\.git$").unwrap();

    if https_re.is_match(url) || git_re.is_match(url) {
        let domain = domain_re.captures(url).unwrap();
        let domain = domain.get(0).unwrap().as_str();

        let repo = repo_re.captures(url).unwrap();
        let user = repo.name("user").unwrap().as_str();
        let repo = repo.name("repo").unwrap().as_str();
        // println!("domain: {}", domain);
        // println!("repo: {}", repo);

        return Some(GitUrl {
            domain: domain.to_string(),
            user: user.to_string(),
            repo: repo.to_string(),
        });
    }

    return None;
}
