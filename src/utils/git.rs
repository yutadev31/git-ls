use anyhow::{Context, Ok, Result};
use git2::Repository;
use regex::Regex;

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

        return Some(GitUrl {
            domain: domain.to_string(),
            user: user.to_string(),
            repo: repo.to_string(),
        });
    }

    return None;
}

pub fn open_repository(path: &str) -> Result<Repository> {
    let repo = Repository::open(path).context("Failed open repository")?;
    Ok(repo)
}
