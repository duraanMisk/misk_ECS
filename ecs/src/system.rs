use crate::{World, EcsResult};

/// Trait for systems that operate on the ECS world
pub trait System {
    /// System name for debugging and identification
    fn name(&self) -> &str;
    
    /// Run the system for one update cycle
    fn run(&mut self, world: &mut World, delta_time: f32) -> EcsResult<()>;
    
    /// Called when the system is first added to the world
    fn initialize(&mut self, _world: &mut World) -> EcsResult<()> {
        Ok(())
    }
    
    /// Called when the system is removed from the world
    fn cleanup(&mut self, _world: &mut World) -> EcsResult<()> {
        Ok(())
    }
}

/// System dispatcher manages and runs systems in order
pub struct SystemDispatcher {
    systems: Vec<Box<dyn System>>,
}

impl SystemDispatcher {
    /// Create a new system dispatcher
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    
    /// Add a system to the dispatcher
    pub fn add_system<S: System + 'static>(&mut self, mut system: S, world: &mut World) -> EcsResult<()> {
        system.initialize(world)?;
        self.systems.push(Box::new(system));
        Ok(())
    }
    
    /// Run all systems in order
    pub fn run_systems(&mut self, world: &mut World, delta_time: f32) -> EcsResult<()> {
        for system in &mut self.systems {
            system.run(world, delta_time)?;
        }
        Ok(())
    }
    
    /// Get the number of registered systems
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }
}

impl Default for SystemDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::World;

    struct TestSystem {
        name: String,
        run_count: usize,
    }

    impl TestSystem {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                run_count: 0,
            }
        }
    }

    impl System for TestSystem {
        fn name(&self) -> &str {
            &self.name
        }

        fn run(&mut self, _world: &mut World, _delta_time: f32) -> EcsResult<()> {
            self.run_count += 1;
            Ok(())
        }
    }

    #[test]
    fn test_system_dispatcher() {
        let mut world = World::new();
        let mut dispatcher = SystemDispatcher::new();
        
        let system = TestSystem::new("test_system");
        dispatcher.add_system(system, &mut world).unwrap();
        
        assert_eq!(dispatcher.system_count(), 1);
        
        dispatcher.run_systems(&mut world, 0.016).unwrap();
    }
}