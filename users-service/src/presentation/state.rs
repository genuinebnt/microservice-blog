use crate::infrastructure::database::factory::RepoProvider;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repos: RepoProvider,
}

impl AppState {
    pub fn new(repos: RepoProvider) -> Self {
        Self { repos }
    }
}
