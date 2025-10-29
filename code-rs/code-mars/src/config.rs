/// Configuration for MARS (Multi-Agent Reasoning System).
use serde::{Deserialize, Serialize};

/// Configuration for MARS execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarsConfig {
    /// Number of agents to spawn (default: 3)
    pub num_agents: usize,

    /// Temperature values for agents to explore different solution paths
    /// Default: [0.3, 0.6, 1.0] for low, medium, high exploration
    pub temperatures: Vec<f32>,

    /// Number of verification passes required before marking solution as verified
    /// Default: 2 (must pass 2 consecutive verifications with no failures)
    pub consensus_threshold: usize,

    /// Enable RSA-inspired solution aggregation and refinement
    /// Default: false (phase 2a)
    pub enable_aggregation: bool,

    /// Enable cross-agent strategy network for shared insights
    /// Default: false (phase 2b)
    pub enable_strategy_network: bool,

    /// Maximum number of improvement iterations
    /// Default: 5
    pub max_iterations: usize,

    /// Whether to wrap reasoning in <think></think> tags
    /// Default: true
    pub use_thinking_tags: bool,

    /// Token budget for complex reasoning
    /// Default: 64000
    pub token_budget_reasoning: usize,

    /// Token budget for lightweight tasks (e.g., coding)
    /// Default: 4000
    pub token_budget_lightweight: usize,

    /// Automatically activate lightweight mode when task max_tokens <= 4000
    /// Default: true
    pub auto_lightweight_mode: bool,

    /// Size of solution population to maintain for aggregation
    /// Default: 6
    pub aggregation_population_size: usize,

    /// Number of solutions to select for aggregation per loop
    /// Default: 3
    pub aggregation_selection_size: usize,

    /// Number of aggregation loops to run
    /// Default: 3
    pub aggregation_loops: usize,

    /// Method to use for aggregation
    /// Default: RSA
    pub aggregation_method: crate::types::AggregationMethod,

    /// Number of completions to generate in MOA phase 1
    /// Default: 3
    pub moa_num_completions: usize,

    /// Enable fallback for MOA when n parameter not supported
    /// Default: true
    pub moa_fallback_enabled: bool,

    /// Provider configuration for multi-model support
    /// Default: None (use default ModelClient)
    pub provider_routing: Option<crate::provider_config::ProviderRoutingConfig>,

    /// Enable multi-provider agent routing
    /// Default: false
    pub enable_multi_provider: bool,

    /// Request timeout in seconds
    /// Default: 300
    pub timeout_seconds: u64,

    /// MCTS simulation depth
    /// Default: 1
    pub mcts_simulation_depth: usize,

    /// MCTS exploration weight (UCB coefficient)
    /// Default: 0.2
    pub mcts_exploration_weight: f32,

    /// MCTS number of simulations
    /// Default: 2
    pub mcts_num_simulations: usize,

    /// MCTS number of actions to generate
    /// Default: 3
    pub mcts_num_actions: usize,

    /// Enable debug logging
    /// Default: false
    pub debug: bool,
}

impl Default for MarsConfig {
    fn default() -> Self {
        Self {
            num_agents: 3,
            temperatures: vec![0.3, 0.6, 1.0],
            consensus_threshold: 2,
            enable_aggregation: false,
            enable_strategy_network: false,
            max_iterations: 5,
            use_thinking_tags: true,
            token_budget_reasoning: 64000,
            token_budget_lightweight: 4000,
            auto_lightweight_mode: true,
            aggregation_population_size: 6,
            aggregation_selection_size: 3,
            aggregation_loops: 3,
            aggregation_method: crate::types::AggregationMethod::RSA,
            moa_num_completions: 3,
            moa_fallback_enabled: true,
            provider_routing: None,
            enable_multi_provider: false,
            timeout_seconds: 300,
            mcts_simulation_depth: 1,
            mcts_exploration_weight: 0.2,
            mcts_num_simulations: 2,
            mcts_num_actions: 3,
            debug: false,
        }
    }
}

impl MarsConfig {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable all advanced features (aggregation and strategy network)
    pub fn with_advanced_features(mut self) -> Self {
        self.enable_aggregation = true;
        self.enable_strategy_network = true;
        self
    }

    /// Lightweight mode for simple tasks (fewer agents, faster processing)
    pub fn lightweight(mut self) -> Self {
        self.num_agents = 2;
        self.max_iterations = 2;
        self.enable_aggregation = false;
        self.enable_strategy_network = false;
        self
    }

    /// Set number of agents
    pub fn with_num_agents(mut self, num: usize) -> Self {
        if num > 0 {
            self.num_agents = num;
            // Ensure we have enough temperatures
            if self.temperatures.len() < num {
                self.temperatures.resize(num, 1.0);
            } else if self.temperatures.len() > num {
                self.temperatures.truncate(num);
            }
        }
        self
    }

    /// Set temperature values
    pub fn with_temperatures(mut self, temps: Vec<f32>) -> Self {
        if !temps.is_empty() {
            self.temperatures = temps;
        }
        self
    }

    /// Enable aggregation
    pub fn with_aggregation(mut self, enabled: bool) -> Self {
        self.enable_aggregation = enabled;
        self
    }

    /// Enable strategy network
    pub fn with_strategy_network(mut self, enabled: bool) -> Self {
        self.enable_strategy_network = enabled;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        if max > 0 {
            self.max_iterations = max;
        }
        self
    }

    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set aggregation method
    pub fn with_aggregation_method(mut self, method: crate::types::AggregationMethod) -> Self {
        self.aggregation_method = method;
        self
    }

    /// Set MOA aggregation method
    pub fn with_moa_aggregation(mut self) -> Self {
        self.aggregation_method = crate::types::AggregationMethod::MixtureOfAgents;
        self
    }

    /// Set number of completions for MOA
    pub fn with_moa_num_completions(mut self, num: usize) -> Self {
        if num > 0 {
            self.moa_num_completions = num;
        }
        self
    }

    /// Set MOA fallback behavior
    pub fn with_moa_fallback_enabled(mut self, enabled: bool) -> Self {
        self.moa_fallback_enabled = enabled;
        self
    }

    /// Set provider routing configuration for multi-model support
    pub fn with_provider_routing(
        mut self,
        routing: crate::provider_config::ProviderRoutingConfig,
    ) -> Self {
        self.provider_routing = Some(routing);
        self
    }

    /// Enable multi-provider agent routing
    pub fn with_multi_provider_enabled(mut self, enabled: bool) -> Self {
        self.enable_multi_provider = enabled;
        self
    }

    /// Get token budget based on mode
    pub fn get_token_budget(&self, is_lightweight: bool) -> usize {
        if is_lightweight {
            self.token_budget_lightweight
        } else {
            self.token_budget_reasoning
        }
    }

    /// Determine if we should use lightweight mode
    pub fn should_use_lightweight(&self, max_tokens: Option<usize>) -> bool {
        if !self.auto_lightweight_mode {
            return false;
        }
        max_tokens.map(|mt| mt <= 4000).unwrap_or(false)
    }

    /// Set MCTS simulation depth
    pub fn with_mcts_simulation_depth(mut self, depth: usize) -> Self {
        self.mcts_simulation_depth = depth;
        self
    }

    /// Set MCTS exploration weight
    pub fn with_mcts_exploration_weight(mut self, weight: f32) -> Self {
        self.mcts_exploration_weight = weight;
        self
    }

    /// Set MCTS number of simulations
    pub fn with_mcts_num_simulations(mut self, num: usize) -> Self {
        self.mcts_num_simulations = num;
        self
    }

    /// Set MCTS number of actions
    pub fn with_mcts_num_actions(mut self, num: usize) -> Self {
        self.mcts_num_actions = num;
        self
    }

    /// Get MCTS configuration from Mars config
    pub fn get_mcts_config(&self) -> crate::mcts::MCTSConfig {
        crate::mcts::MCTSConfig {
            simulation_depth: self.mcts_simulation_depth,
            exploration_weight: self.mcts_exploration_weight,
            num_simulations: self.mcts_num_simulations,
            num_actions: self.mcts_num_actions,
            generation_temperature: 1.0,
            evaluation_temperature: 0.1,
            max_history_length: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MarsConfig::default();
        assert_eq!(config.num_agents, 3);
        assert_eq!(config.temperatures.len(), 3);
        assert!(!config.enable_aggregation);
        assert_eq!(config.max_iterations, 5);
    }

    #[test]
    fn test_lightweight_config() {
        let config = MarsConfig::new().lightweight();
        assert_eq!(config.num_agents, 2);
        assert_eq!(config.max_iterations, 2);
        assert!(!config.enable_aggregation);
    }

    #[test]
    fn test_advanced_features() {
        let config = MarsConfig::new().with_advanced_features();
        assert!(config.enable_aggregation);
        assert!(config.enable_strategy_network);
    }

    #[test]
    fn test_token_budget() {
        let config = MarsConfig::default();
        assert_eq!(config.get_token_budget(false), 64000);
        assert_eq!(config.get_token_budget(true), 4000);
    }
}
