#[derive(Clone)]
pub struct AppState {
    pub error: Vec<String>,
    pub token: Option<String>,
    pub loading: bool,
    pub api_base_url: String,
}

impl AppState {
    pub fn new(api_base_url: &str) -> Self {
        Self {
            error: Vec::new(),
            loading: false,
            token: None,
            api_base_url: api_base_url.into(),
        }
    }
}
