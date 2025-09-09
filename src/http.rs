use reqwest::Client;
use std::env;

/// GitHub HTTP client with authentication and standard headers
pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    /// Create a new GitHub client with authentication from GITHUB_TOKEN environment variable
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token =
            env::var("GITHUB_TOKEN").map_err(|_| "GITHUB_TOKEN environment variable not set")?;
        Ok(Self {
            client: Client::new(),
            token,
        })
    }

    /// Create a GET request with standard GitHub headers
    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::GET, url)
    }

    /// Create a POST request with standard GitHub headers
    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::POST, url)
    }

    /// Create a request with the specified method and standard GitHub headers
    pub fn request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "ghai")
    }
}

/// URL builder utility for constructing URLs with query parameters
pub struct UrlBuilder {
    base: String,
    params: Vec<(String, String)>,
}

impl UrlBuilder {
    /// Create a new URL builder with the base URL
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            params: Vec::new(),
        }
    }

    /// Add a query parameter if the value is Some
    pub fn param(mut self, key: &str, value: Option<impl std::fmt::Display>) -> Self {
        if let Some(val) = value {
            self.params.push((key.to_string(), val.to_string()));
        }
        self
    }

    /// Add a required query parameter
    pub fn required_param(mut self, key: &str, value: impl std::fmt::Display) -> Self {
        self.params.push((key.to_string(), value.to_string()));
        self
    }

    /// Build the final URL with query parameters
    pub fn build(self) -> String {
        if self.params.is_empty() {
            return self.base;
        }

        let params: Vec<String> = self
            .params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v)))
            .collect();
        format!("{}?{}", self.base, params.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_builder_no_params() {
        let url = UrlBuilder::new("https://api.github.com/repos").build();
        assert_eq!(url, "https://api.github.com/repos");
    }

    #[test]
    fn url_builder_with_params() {
        let url = UrlBuilder::new("https://api.github.com/repos")
            .required_param("state", "open")
            .param("labels", Some("bug"))
            .param("assignee", None::<&str>)
            .build();
        assert_eq!(url, "https://api.github.com/repos?state=open&labels=bug");
    }

    #[test]
    fn url_builder_encodes_values() {
        let url = UrlBuilder::new("https://api.github.com/search")
            .required_param("q", "repo:user/repo is:issue")
            .build();
        assert_eq!(
            url,
            "https://api.github.com/search?q=repo%3Auser%2Frepo%20is%3Aissue"
        );
    }
}
