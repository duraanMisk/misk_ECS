use ecs::{World, SystemDispatcher, EcsResult};
use crate::components::{Position, Velocity, Name, Mass, Rotation};
use crate::systems::{MovementSystem, DebugSystem};

/// SimWorld wraps the ECS World and manages the simulation loop
pub struct SimWorld {
    pub world: World,
    pub dispatcher: SystemDispatcher,
    pub time_step: f32,
    pub total_time: f32,
}

impl SimWorld {
    /// Create a new simulation world
    pub fn new() -> Self {
        Self {
            world: World::new(),
            dispatcher: SystemDispatcher::new(),
            time_step: 1.0 / 60.0, // 60 FPS
            total_time: 0.0,
        }
    }
    
    /// Initialize the simulation with default systems
    pub fn initialize(&mut self) -> EcsResult<()> {
        // Add core systems
        self.dispatcher.add_system(MovementSystem::new(), &mut self.world)?;
        self.dispatcher.add_system(DebugSystem::new(2.0), &mut self.world)?;
        
        println!("SimWorld initialized with {} systems", self.dispatcher.system_count());
        Ok(())
    }
    
    /// Add some sample entities for testing
    pub fn populate_with_test_entities(&mut self) -> EcsResult<()> {
        // Create a moving entity
        let entity1 = self.world.create_entity();
        self.world.add_component(entity1, Name::new("Moving Object"))?;
        self.world.add_component(entity1, Position::new(0.0, 0.0))?;
        self.world.add_component(entity1, Velocity::new(10.0, 5.0))?;
        self.world.add_component(entity1, Mass::new(1.0))?;
        self.world.add_component(entity1, Rotation::zero())?;
        
        // Create a stationary entity
        let entity2 = self.world.create_entity();
        self.world.add_component(entity2, Name::new("Stationary Object"))?;
        self.world.add_component(entity2, Position::new(50.0, 30.0))?;
        self.world.add_component(entity2, Velocity::zero())?;
        self.world.add_component(entity2, Mass::new(2.5))?;
        
        // Create another moving entity
        let entity3 = self.world.create_entity();
        self.world.add_component(entity3, Name::new("Fast Object"))?;
        self.world.add_component(entity3, Position::new(-20.0, 10.0))?;
        self.world.add_component(entity3, Velocity::new(-15.0, 8.0))?;
        self.world.add_component(entity3, Mass::new(0.5))?;
        
        println!("Created {} test entities", self.world.entity_count());
        Ok(())
    }
    
    /// Step the simulation forward by one time step
    pub fn step(&mut self) -> EcsResult<()> {
        self.dispatcher.run_systems(&mut self.world, self.time_step)?;
        self.total_time += self.time_step;
        Ok(())
    }
    
    /// Run the simulation for a specified duration
    pub fn run_for(&mut self, duration: f32) -> EcsResult<()> {
        let steps = (duration / self.time_step) as usize;
        println!("Running simulation for {:.2}s ({} steps)", duration, steps);
        
        for step in 0..steps {
            self.step()?;
            
            // Print progress every 60 steps (1 second at 60 FPS)
            if step % 60 == 0 {
                println!("Step {}/{} (Time: {:.2}s)", step + 1, steps, self.total_time);
            }
        }
        
        Ok(())
    }
    
    /// Get simulation statistics
    pub fn stats(&self) -> SimStats {
        SimStats {
            entity_count: self.world.entity_count(),
            system_count: self.dispatcher.system_count(),
            total_time: self.total_time,
            time_step: self.time_step,
        }
    }
}

impl Default for SimWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulation statistics
#[derive(Debug)]
pub struct SimStats {
    pub entity_count: usize,
    pub system_count: usize,
    pub total_time: f32,
    pub time_step: f32,
}