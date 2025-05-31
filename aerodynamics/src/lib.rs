// Aerodynamics module - placeholder for Phase 2 implementation
// This will contain lift/drag calculations, airfoil modeling, etc.

use serde::{Deserialize, Serialize};

/// Aerodynamic properties component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AeroProperties {
    pub lift_coefficient: f32,
    pub drag_coefficient: f32,
    pub wing_area: f32,
    pub angle_of_attack: f32, // radians
}

impl AeroProperties {
    pub fn new(lift_coeff: f32, drag_coeff: f32, wing_area: f32) -> Self {
        Self {
            lift_coefficient: lift_coeff,
            drag_coefficient: drag_coeff,
            wing_area,
            angle_of_attack: 0.0,
        }
    }
    
    /// Simple aircraft preset
    pub fn simple_aircraft() -> Self {
        Self {
            lift_coefficient: 0.5,
            drag_coefficient: 0.05,
            wing_area: 10.0,
            angle_of_attack: 0.0,
        }
    }
}

/// Wind conditions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub turbulence: f32,
}

impl Wind {
    pub fn calm() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            turbulence: 0.0,
        }
    }
}

/// Placeholder for aerodynamic systems (to be implemented in Phase 2)
pub mod systems {
    // Will contain lift/drag calculation systems, wind effects, etc.
}