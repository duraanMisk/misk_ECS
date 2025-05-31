// Physics module - placeholder for Phase 2 implementation
// This will contain rigid body physics, force integration, etc.

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

/// Force component for entities
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Force {
    pub x: f32,
    pub y: f32,
}

impl Force {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    pub fn to_vector(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}

/// Placeholder for physics constants
pub mod constants {
    pub const GRAVITY: f32 = 9.81; // m/s²
    pub const AIR_DENSITY: f32 = 1.225; // kg/m³ at sea level
}

/// Placeholder for physics systems (to be implemented in Phase 2)
pub mod systems {
    // Will contain integration systems, collision detection, etc.
}