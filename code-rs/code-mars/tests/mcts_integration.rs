//! Integration tests for MCTS implementation

use code_mars::{
    config::MarsConfig, mcts::*, Result, LLMProvider,
};

/// Mock LLM provider for MCTS testing
struct MockMCTSProvider;

#[async_trait::async_trait]
impl LLMProvider for MockMCTSProvider {
    async fn complete(
        &self,
        _prompt: &str,
        _system: Option<&str>,
    ) -> Result<String> {
        // Return a mock response
        Ok("This is a mock MCTS response".to_string())
    }

    async fn stream(
        &self,
        _prompt: &str,
        _system: Option<&str>,
    ) -> Result<code_mars::model_router::ModelStream> {
        unimplemented!("Stream not used in MCTS tests")
    }

    fn provider_name(&self) -> &str {
        "mock-mcts"
    }

    fn model_name(&self) -> &str {
        "mock-model"
    }
}

#[test]
fn test_dialogue_state_creation() {
    let state = DialogueState::new(
        "You are helpful".to_string(),
        vec![],
        "What is 2+2?".to_string(),
    );
    assert_eq!(state.system_prompt, "You are helpful");
    assert_eq!(state.current_query, "What is 2+2?");
    assert!(state.conversation_history.is_empty());
}

#[test]
fn test_message_creation() {
    let msg = Message::new("user", "Hello");
    assert_eq!(msg.role, "user");
    assert_eq!(msg.content, "Hello");
}

#[test]
fn test_mcts_node_creation() {
    let state = DialogueState::new("prompt".to_string(), vec![], "query".to_string());
    let node = MCTSNode::new(state, None);
    assert_eq!(node.visits, 0);
    assert_eq!(node.value, 0.0);
    assert!(node.children.is_empty());
    assert!(node.parent.is_none());
}

#[test]
fn test_mcts_node_with_parent() {
    let state = DialogueState::new("prompt".to_string(), vec![], "query".to_string());
    let node = MCTSNode::new(state, Some(0));
    assert_eq!(node.parent, Some(0));
}

#[test]
fn test_mcts_config_defaults() {
    let config = MCTSConfig::default();
    assert_eq!(config.simulation_depth, 1);
    assert_eq!(config.exploration_weight, 0.2);
    assert_eq!(config.num_simulations, 2);
    assert_eq!(config.num_actions, 3);
}

#[test]
fn test_mcts_creation() {
    let config = MCTSConfig::default();
    let mcts = MCTS::new(config);
    assert_eq!(mcts.completion_tokens, 0);
}

#[test]
fn test_is_terminal_not_terminal() {
    let config = MCTSConfig::default();
    let mcts = MCTS::new(config);
    let state = DialogueState::new("".to_string(), vec![], "hello".to_string());
    assert!(!mcts.is_terminal(&state));
}

#[test]
fn test_is_terminal_with_goodbye() {
    let config = MCTSConfig::default();
    let mcts = MCTS::new(config);
    let state = DialogueState::new("".to_string(), vec![], "goodbye".to_string());
    assert!(mcts.is_terminal(&state));
}

#[test]
fn test_is_terminal_max_history() {
    let config = MCTSConfig::default();
    let mcts = MCTS::new(config);

    // Create history with more than max_history_length messages
    let mut history = Vec::new();
    for i in 0..15 {
        history.push(Message::new(
            if i % 2 == 0 { "user" } else { "assistant" },
            format!("Message {}", i),
        ));
    }

    let state = DialogueState::new("".to_string(), history, "hello".to_string());
    assert!(mcts.is_terminal(&state));
}

#[test]
fn test_mars_config_mcts_defaults() {
    let config = MarsConfig::default();
    assert_eq!(config.mcts_simulation_depth, 1);
    assert_eq!(config.mcts_exploration_weight, 0.2);
    assert_eq!(config.mcts_num_simulations, 2);
    assert_eq!(config.mcts_num_actions, 3);
}

#[test]
fn test_mars_config_get_mcts_config() {
    let config = MarsConfig::default();
    let mcts_config = config.get_mcts_config();

    assert_eq!(mcts_config.simulation_depth, 1);
    assert_eq!(mcts_config.exploration_weight, 0.2);
    assert_eq!(mcts_config.num_simulations, 2);
    assert_eq!(mcts_config.num_actions, 3);
    assert_eq!(mcts_config.generation_temperature, 1.0);
    assert_eq!(mcts_config.evaluation_temperature, 0.1);
}

#[test]
fn test_mars_config_custom_mcts() {
    let config = MarsConfig::new()
        .with_mcts_simulation_depth(3)
        .with_mcts_exploration_weight(0.5)
        .with_mcts_num_simulations(5)
        .with_mcts_num_actions(4);

    assert_eq!(config.mcts_simulation_depth, 3);
    assert_eq!(config.mcts_exploration_weight, 0.5);
    assert_eq!(config.mcts_num_simulations, 5);
    assert_eq!(config.mcts_num_actions, 4);
}

#[tokio::test]
async fn test_mcts_search_basic() {
    let config = MCTSConfig {
        num_simulations: 1,
        simulation_depth: 0,
        ..Default::default()
    };

    let mut mcts = MCTS::new(config);
    let initial_state = DialogueState::new(
        "You are helpful".to_string(),
        vec![],
        "Test query".to_string(),
    );

    let provider = MockMCTSProvider;
    let result = mcts.search(initial_state, &provider).await;

    assert!(result.is_ok());
}

#[test]
fn test_aggregation_method_mcts() {
    use code_mars::types::AggregationMethod;

    let method = AggregationMethod::MonteCarloTreeSearch;
    match method {
        AggregationMethod::MonteCarloTreeSearch => {
            // Test passes if we match the MCTS variant
        }
        _ => {
            panic!("Expected MonteCarloTreeSearch variant");
        }
    }
}

#[test]
fn test_config_mcts_builder_methods() {
    let config = MarsConfig::new()
        .with_mcts_simulation_depth(2)
        .with_mcts_exploration_weight(0.3)
        .with_mcts_num_simulations(3)
        .with_mcts_num_actions(5);

    assert_eq!(config.mcts_simulation_depth, 2);
    assert_eq!(config.mcts_exploration_weight, 0.3);
    assert_eq!(config.mcts_num_simulations, 3);
    assert_eq!(config.mcts_num_actions, 5);
}

#[test]
fn test_dialogue_state_with_history() {
    let history = vec![
        Message::new("user", "Hello"),
        Message::new("assistant", "Hi there!"),
    ];

    let state = DialogueState::new(
        "You are helpful".to_string(),
        history,
        "How are you?".to_string(),
    );

    assert_eq!(state.conversation_history.len(), 2);
    assert_eq!(state.conversation_history[0].role, "user");
    assert_eq!(state.conversation_history[1].role, "assistant");
}
