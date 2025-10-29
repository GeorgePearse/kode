/// Model router for unified access to multiple LLM providers.
///
/// Provides abstraction layer supporting both code_core::ModelClient
/// and litellm-rs for flexible provider selection.

use crate::Result;
use async_trait::async_trait;

/// Stream wrapper for generic model responses
pub struct ModelStream {
    content: String,
    position: usize,
}

impl ModelStream {
    /// Create new model stream from content
    pub fn new(content: String) -> Self {
        Self {
            content,
            position: 0,
        }
    }

    /// Get next chunk of streaming content
    pub fn next_chunk(&mut self) -> Option<String> {
        if self.position >= self.content.len() {
            return None;
        }

        // For now, yield the entire content
        // In production, this would stream incrementally
        let chunk = self.content.clone();
        self.position = self.content.len();
        Some(chunk)
    }
}

/// Generic LLM provider trait for unified provider access
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Complete a prompt and return the full response
    async fn complete(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String>;

    /// Stream a prompt response incrementally
    async fn stream(&self, prompt: &str, system_prompt: Option<&str>) -> Result<ModelStream>;

    /// Get provider name for logging/debugging
    fn provider_name(&self) -> &str;

    /// Get model name for logging/debugging
    fn model_name(&self) -> &str;
}

/// Wrapper around litellm-rs for multi-provider support
pub struct LiteLLMRouter {
    /// Provider name (e.g., "openai", "anthropic")
    provider: String,
    /// Model identifier
    model: String,
    /// API key for authentication (used in full implementation)
    #[allow(dead_code)]
    api_key: String,
}

impl LiteLLMRouter {
    /// Create new litellm router
    pub fn new(provider: String, model: String, api_key: String) -> Self {
        Self {
            provider,
            model,
            api_key,
        }
    }
}

#[async_trait]
impl LLMProvider for LiteLLMRouter {
    async fn complete(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String> {
        // For now, return a placeholder indicating litellm-rs integration would happen here
        // Full litellm-rs integration requires proper API key handling and async support
        // This demonstrates the trait contract while integration details can be completed later

        let prefix = if let Some(system) = system_prompt {
            format!("[SYSTEM: {}] ", system)
        } else {
            String::new()
        };

        // In a full implementation, this would call litellm-rs API
        // litellm_rs::completion(&self.model, messages, options).await
        // For now, we return a structured response that demonstrates routing capability
        Ok(format!(
            "{}[Response from {}/{}] {}",
            prefix, self.provider, self.model, prompt
        ))
    }

    async fn stream(&self, prompt: &str, system_prompt: Option<&str>) -> Result<ModelStream> {
        // For now, fall back to non-streaming completion
        // Full streaming support would require litellm-rs streaming API
        let content = self.complete(prompt, system_prompt).await?;
        Ok(ModelStream::new(content))
    }

    fn provider_name(&self) -> &str {
        &self.provider
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

/// Wrapper around code_core::ModelClient for backward compatibility
pub struct ModelClientRouter {
    client: code_core::ModelClient,
}

impl ModelClientRouter {
    /// Create new ModelClient router
    pub fn new(client: code_core::ModelClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl LLMProvider for ModelClientRouter {
    async fn complete(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String> {
        // Build prompt for ModelClient
        let user_prompt = if let Some(system) = system_prompt {
            format!("{}\n\n{}", system, prompt)
        } else {
            prompt.to_string()
        };

        let mut p = code_core::Prompt::default();
        p.input = vec![code_core::ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![code_core::ContentItem::InputText {
                text: user_prompt,
            }],
        }];

        if let Some(system) = system_prompt {
            p.base_instructions_override = Some(system.to_string());
        }

        p.set_log_tag("model_client_router");

        // Stream to completion
        use futures::StreamExt;
        let mut stream = self.client.stream(&p).await?;
        let mut response = String::new();

        while let Some(event) = stream.next().await {
            match event? {
                code_core::ResponseEvent::OutputTextDelta { delta, .. } => {
                    response.push_str(&delta);
                }
                code_core::ResponseEvent::Completed { .. } => break,
                _ => {}
            }
        }

        Ok(response)
    }

    async fn stream(&self, prompt: &str, system_prompt: Option<&str>) -> Result<ModelStream> {
        // Build prompt for ModelClient
        let user_prompt = if let Some(system) = system_prompt {
            format!("{}\n\n{}", system, prompt)
        } else {
            prompt.to_string()
        };

        let mut p = code_core::Prompt::default();
        p.input = vec![code_core::ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![code_core::ContentItem::InputText {
                text: user_prompt,
            }],
        }];

        if let Some(system) = system_prompt {
            p.base_instructions_override = Some(system.to_string());
        }

        p.set_log_tag("model_client_router_stream");

        // Stream to completion
        use futures::StreamExt;
        let mut stream = self.client.stream(&p).await?;
        let mut response = String::new();

        while let Some(event) = stream.next().await {
            match event? {
                code_core::ResponseEvent::OutputTextDelta { delta, .. } => {
                    response.push_str(&delta);
                }
                code_core::ResponseEvent::Completed { .. } => break,
                _ => {}
            }
        }

        Ok(ModelStream::new(response))
    }

    fn provider_name(&self) -> &str {
        "code-client"
    }

    fn model_name(&self) -> &str {
        "code-model"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_stream_creation() {
        let stream = ModelStream::new("Hello, world!".to_string());
        assert_eq!(stream.content, "Hello, world!");
        assert_eq!(stream.position, 0);
    }

    #[test]
    fn test_litellm_router_creation() {
        let router = LiteLLMRouter::new(
            "openai".to_string(),
            "gpt-4o".to_string(),
            "test-key".to_string(),
        );

        assert_eq!(router.provider_name(), "openai");
        assert_eq!(router.model_name(), "gpt-4o");
    }
}
