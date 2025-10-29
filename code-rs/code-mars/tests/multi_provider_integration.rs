//! Integration tests for multi-provider support in MARS
//!
//! Tests the unified LLMProvider trait interface and provider routing capabilities.

use code_mars::{
    config::MarsConfig, provider_config::ProviderSpec,
    LLMProvider, Result,
};

/// Mock LLM provider for testing
struct MockProvider {
    name: String,
}

#[async_trait::async_trait]
impl LLMProvider for MockProvider {
    async fn complete(
        &self,
        prompt: &str,
        system_prompt: Option<&str>,
    ) -> Result<String> {
        let mut response = format!("Mock response to prompt: {}", &prompt[..50.min(prompt.len())]);

        if let Some(system) = system_prompt {
            response = format!("{}System: {}\n{}", response, system, response);
        }

        Ok(response)
    }

    async fn stream(
        &self,
        prompt: &str,
        system_prompt: Option<&str>,
    ) -> Result<code_mars::model_router::ModelStream> {
        let content = self.complete(prompt, system_prompt).await?;
        Ok(code_mars::model_router::ModelStream::new(content))
    }

    fn provider_name(&self) -> &str {
        &self.name
    }

    fn model_name(&self) -> &str {
        "mock-model"
    }
}

#[test]
fn test_mars_config_with_provider_routing() {
    let config = MarsConfig::new().with_multi_provider_enabled(true);
    assert!(config.enable_multi_provider);
}

#[test]
fn test_provider_spec_creation() {
    let spec = ProviderSpec::new("openai", "gpt-4")
        .with_api_key("test-key".to_string());
    assert_eq!(spec.provider, "openai");
    assert_eq!(spec.model, "gpt-4");
    assert_eq!(spec.api_key, "test-key");
}

#[test]
fn test_provider_spec_with_builder_methods() {
    let spec = ProviderSpec::new("anthropic", "claude-3")
        .with_api_key("new-key".to_string())
        .with_priority(10);

    assert_eq!(spec.api_key, "new-key");
    assert_eq!(spec.priority, 10);
}

#[tokio::test]
async fn test_mock_provider_complete() {
    let mock = MockProvider {
        name: "test-provider".to_string(),
    };

    let response = mock.complete("What is 2+2?", None).await;
    assert!(response.is_ok());

    let content = response.unwrap();
    assert!(content.contains("Mock response"));
}

#[tokio::test]
async fn test_mock_provider_with_system_prompt() {
    let mock = MockProvider {
        name: "test-provider".to_string(),
    };

    let response = mock
        .complete("What is 2+2?", Some("You are a math teacher"))
        .await;
    assert!(response.is_ok());

    let content = response.unwrap();
    assert!(content.contains("System:"));
}

#[test]
fn test_mock_provider_metadata() {
    let mock = MockProvider {
        name: "gpt-4-provider".to_string(),
    };

    assert_eq!(mock.provider_name(), "gpt-4-provider");
    assert_eq!(mock.model_name(), "mock-model");
}

#[test]
fn test_mars_config_default_values() {
    let config = MarsConfig::default();

    assert_eq!(config.num_agents, 3);
    assert!(!config.enable_multi_provider);
    assert!(config.provider_routing.is_none());
}

#[test]
fn test_mars_config_advanced_features() {
    let config = MarsConfig::new()
        .with_num_agents(5)
        .with_multi_provider_enabled(true);

    assert_eq!(config.num_agents, 5);
    assert!(config.enable_multi_provider);
}

#[test]
fn test_mars_config_moa_settings() {
    let config = MarsConfig::new()
        .with_moa_num_completions(5)
        .with_moa_fallback_enabled(false);

    assert_eq!(config.moa_num_completions, 5);
    assert!(!config.moa_fallback_enabled);
}

#[test]
fn test_provider_spec_validation() {
    let spec = ProviderSpec::new("openai", "gpt-4")
        .with_api_key("test-key".to_string());
    let result = spec.validate();
    assert!(result.is_ok());
}

#[test]
fn test_provider_spec_with_base_url() {
    let spec = ProviderSpec::new("custom", "model")
        .with_api_key("key".to_string())
        .with_base_url("https://api.custom.com".to_string());

    assert_eq!(
        spec.base_url,
        Some("https://api.custom.com".to_string())
    );
}

#[test]
fn test_mars_config_token_budgets() {
    let config = MarsConfig::default();

    assert_eq!(config.get_token_budget(false), 64000);
    assert_eq!(config.get_token_budget(true), 4000);
}

#[test]
fn test_mars_config_lightweight_mode() {
    let config = MarsConfig::default();

    // Auto lightweight mode should trigger for small token budgets
    assert!(config.should_use_lightweight(Some(2000)));
    assert!(!config.should_use_lightweight(Some(5000)));
}
