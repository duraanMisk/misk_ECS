// RL Interface module - placeholder for Phase 4 implementation
// This will contain the reinforcement learning API and Python bindings

use serde::{Deserialize, Serialize};

/// State observation for RL agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub position_x: f32,
    pub position_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
}

impl Observation {
    pub fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rotation: 0.0,
            angular_velocity: 0.0,
        }
    }
    
    /// Convert to vector for RL frameworks
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position_x,
            self.position_y,
            self.velocity_x,
            self.velocity_y,
            self.rotation,
            self.angular_velocity,
        ]
    }
}

/// Action from RL agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub thrust: f32,
    pub elevator: f32,
    pub rudder: f32,
}

impl Action {
    pub fn neutral() -> Self {
        Self {
            thrust: 0.0,
            elevator: 0.0,
            rudder: 0.0,
        }
    }
    
    pub fn from_vec(values: &[f32]) -> Option<Self> {
        if values.len() >= 3 {
            Some(Self {
                thrust: values[0],
                elevator: values[1],
                rudder: values[2],
            })
        } else {
            None
        }
    }
}

/// RL Environment trait (to be implemented in Phase 4)
pub trait RLEnvironment {
    fn reset(&mut self) -> Observation;
    fn step(&mut self, action: Action) -> (Observation, f32, bool); // obs, reward, done
    fn get_observation(&self) -> Observation;
}

/// Placeholder for RL integration (to be implemented in Phase 4)
#[cfg(feature = "python")]
pub mod python {
    // Will contain PyO3 bindings for Python RL frameworks
}