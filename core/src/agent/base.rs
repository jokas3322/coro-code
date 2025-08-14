//! Base agent trait and structures

use super::config::AgentConfig;
use crate::error::Result;
use crate::trajectory::TrajectoryRecorder;
use async_trait::async_trait;

use super::execution::AgentExecution;

/// Result type for agent operations
pub type AgentResult<T> = Result<T>;

/// Base trait for all agents
#[async_trait]
pub trait Agent: Send + Sync {
    /// Execute a task
    async fn execute_task(&mut self, task: &str) -> AgentResult<AgentExecution>;

    /// Get the agent's configuration
    fn config(&self) -> &AgentConfig;

    /// Get the agent's name/type
    fn agent_type(&self) -> &str;

    /// Set the trajectory recorder
    fn set_trajectory_recorder(&mut self, recorder: TrajectoryRecorder);

    /// Get the trajectory recorder
    fn trajectory_recorder(&self) -> Option<&TrajectoryRecorder>;
}

// TODO: AgentFactory needs to be updated for new config system
// /// Factory for creating agents
// pub struct AgentFactory;
