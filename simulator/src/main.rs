use anyhow::Result;

mod components;
mod systems;
mod world;

use world::SimWorld;

/// Main entry point for the aerodynamic simulator
fn main() -> Result<()> {
    println!("🚀 Aerodynamic Simulator Starting...");
    println!("=====================================");
    
    // Create and initialize the simulation world
    let mut sim_world = SimWorld::new();
    sim_world.initialize()?;
    
    // Add some test entities to see the ECS in action
    sim_world.populate_with_test_entities()?;
    
    // Print initial state
    let stats = sim_world.stats();
    println!("Initial simulation state:");
    println!("  Entities: {}", stats.entity_count);
    println!("  Systems: {}", stats.system_count);
    println!("  Time Step: {:.4}s", stats.time_step);
    println!();
    
    // Run the simulation for 5 seconds
    println!("Running simulation...");
    sim_world.run_for(5.0)?;
    
    // Print final state
    let final_stats = sim_world.stats();
    println!();
    println!("Simulation completed!");
    println!("  Total time: {:.2}s", final_stats.total_time);
    println!("  Final entity count: {}", final_stats.entity_count);
    
    println!("=====================================");
    println!("✅ ECS-based Simulator Demo Complete");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simulator_basic_functionality() {
        let mut sim_world = SimWorld::new();
        
        // Test initialization
        assert!(sim_world.initialize().is_ok());
        assert_eq!(sim_world.stats().entity_count, 0);
        
        // Test entity creation
        assert!(sim_world.populate_with_test_entities().is_ok());
        assert_eq!(sim_world.stats().entity_count, 3);
        
        // Test simulation step
        assert!(sim_world.step().is_ok());
        assert!(sim_world.stats().total_time > 0.0);
    }
    
    #[test]
    fn test_components_basic_functionality() {
        use components::*;
        
        let pos = Position::new(1.0, 2.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        
        let vel = Velocity::new(3.0, 4.0);
        assert_eq!(vel.magnitude(), 5.0); // 3-4-5 triangle
        
        let name = Name::new("test");
        assert_eq!(name.value, "test");
    }
}