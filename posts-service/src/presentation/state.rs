use crate::infrastructure::database::RepoProvider;

pub struct AppState {
    pub repos: RepoProvider,
}

impl AppState {
    pub fn new(repo_provider: RepoProvider) -> Self {
        Self {
            repos: repo_provider,
        }
    }
}
