// Import statements - bring types and functions from other modules into scope
use ecs::{System, World, EcsResult};      // Our ECS types from the ecs crate
use crate::components::{Position, Velocity}; // Component types from our components module

/// Simple movement system that updates positions based on velocities
/// 
/// This is a "System" in ECS terminology - a piece of logic that operates on entities
/// with specific components. This system finds all entities that have BOTH Position 
/// and Velocity components and moves them according to basic physics:
/// 
/// new_position = old_position + (velocity * time)
/// 
/// This is called "Euler integration" - the simplest way to simulate movement
pub struct MovementSystem {
    /// Name for debugging and identification
    /// All systems need a name so we can track them and debug issues
    name: String,
}

// Implementation block for MovementSystem
impl MovementSystem {
    /// Constructor function - creates a new MovementSystem
    /// 
    /// This is an associated function (no 'self' parameter)
    /// Called like: MovementSystem::new()
    pub fn new() -> Self {
        Self {
            name: "MovementSystem".to_string(),  // Convert &str to owned String
        }
    }
}

// Implement the System trait for MovementSystem
// 
// This is how we tell the ECS that MovementSystem is a system that can be run
// The System trait defines the interface that all systems must follow
impl System for MovementSystem {
    /// Returns the name of this system for debugging
    /// 
    /// &str is a string slice - a borrowed reference to string data
    /// We return a reference to our owned String's data
    fn name(&self) -> &str {
        &self.name  // Borrow a reference to the String's contents
    }
    
    /// The main system logic - called every frame
    /// 
    /// Parameters:
    /// - &mut self: Mutable reference to this system (we might change internal state)
    /// - world: Mutable reference to the ECS world (we'll modify entity positions)
    /// - delta_time: Time since last frame in seconds (for frame-rate independent movement)
    /// 
    /// Returns: EcsResult<()> which is Result<(), EcsError>
    /// - Ok(()) means the system ran successfully
    /// - Err(error) means something went wrong
    fn run(&mut self, world: &mut World, delta_time: f32) -> EcsResult<()> {
        // Step 1: Get all entities that exist in the world
        // 
        // world.entities() returns an iterator over all entities
        // .collect() converts the iterator into a Vec<Entity>
        // 
        // Why collect first? Because we need to avoid "borrowing conflicts":
        // - We're about to borrow world mutably to modify components
        // - If we kept the iterator, we'd have both mutable and immutable borrows
        // - Rust prevents this to avoid data races
        let entities: Vec<_> = world.entities().collect();
        
        // Step 2: Process each entity
        // 
        // for loop iterates over each entity in our collected vector
        for entity in entities {
            // Step 3: Check if entity has both components first
            // 
            // We need to check existence separately to avoid borrowing conflicts
            // Rust doesn't allow borrowing world both mutably and immutably at the same time
            let has_both_components = world.has_component::<Velocity>(entity) 
                && world.has_component::<Position>(entity);
            
            if has_both_components {
                // Step 4: Get velocity first (immutable borrow)
                let velocity = *world.get_component::<Velocity>(entity).unwrap();
                
                // Step 5: Get position mutably (mutable borrow)
                // The immutable borrow above is finished, so this is safe
                if let Some(position) = world.get_component_mut::<Position>(entity) {
                    // Step 6: Apply movement physics
                    // 
                    // Basic Euler integration: position += velocity * time
                    // This simulates movement at the given velocity over the time period
                    // 
                    // delta_time makes movement frame-rate independent:
                    // - At 60 FPS: delta_time ≈ 0.0167 seconds
                    // - At 30 FPS: delta_time ≈ 0.0333 seconds  
                    // - Same velocity will move the same distance per second regardless of framerate
                    position.x += velocity.x * delta_time;
                    position.y += velocity.y * delta_time;
                }
            }
            // If the entity doesn't have both Position and Velocity, we simply skip it
            // This is the power of ECS: entities can have any combination of components
        }
        
        // Step 5: Return success
        // () is the "unit type" - like void in C, but it's an actual value in Rust
        Ok(())
    }
    
    /// Called when the system is first added to the world
    /// 
    /// This is a setup/initialization method. We can use it to:
    /// - Register component types the system needs
    /// - Set up initial state
    /// - Print debug information
    /// - Validate that the world is ready for this system
    /// 
    /// The default implementation in the trait does nothing, but we override it here
    fn initialize(&mut self, world: &mut World) -> EcsResult<()> {
        // Register the components this system uses
        // 
        // This ensures the world knows about these component types and has
        // storage allocated for them. If we don't do this, trying to add
        // these components later might cause problems.
        world.register_component::<Position>();
        world.register_component::<Velocity>();
        
        // Print a message so we know the system started up
        // println! is Rust's print macro - similar to printf in C
        println!("MovementSystem initialized");
        
        Ok(())
    }
}

/// Debug system that prints entity information
/// 
/// This system helps with development and debugging by periodically printing
/// information about entities in the world. It's not essential for gameplay,
/// but very useful for understanding what's happening in your simulation.
/// 
/// This demonstrates a different kind of system - one that reads data but
/// doesn't modify game state (except for its own internal timer)
pub struct DebugSystem {
    /// Name for identification
    name: String,
    
    /// How often to print debug info (in seconds)
    /// For example, 2.0 means print every 2 seconds
    print_interval: f32,
    
    /// Time elapsed since last print (in seconds)
    /// This is internal state that the system maintains between runs
    /// We accumulate delta_time here until we reach print_interval
    elapsed: f32,
}

impl DebugSystem {
    /// Create a new debug system with a specified print interval
    /// 
    /// Parameters:
    /// - print_interval: How often to print debug info in seconds
    /// 
    /// Example: DebugSystem::new(1.0) prints debug info every second
    pub fn new(print_interval: f32) -> Self {
        Self {
            name: "DebugSystem".to_string(),
            print_interval,  // Field init shorthand - same as print_interval: print_interval
            elapsed: 0.0,    // Start with no elapsed time
        }
    }
}

impl System for DebugSystem {
    fn name(&self) -> &str {
        &self.name
    }
    
    /// The main debug system logic
    /// 
    /// This system uses a timer pattern: accumulate time until we reach
    /// our target interval, then do something and reset the timer
    fn run(&mut self, world: &mut World, delta_time: f32) -> EcsResult<()> {
        // Step 1: Update our internal timer
        // 
        // Add the time since last frame to our elapsed time counter
        // delta_time is typically a small number like 0.016 (60 FPS)
        self.elapsed += delta_time;
        
        // Step 2: Check if it's time to print debug info
        // 
        // Only proceed if enough time has passed since our last print
        if self.elapsed >= self.print_interval {
            // Reset the timer for the next cycle
            // We don't set it to 0.0 in case we overshot the interval slightly
            // This keeps the timing more accurate over long periods
            self.elapsed = 0.0;
            
            // Step 3: Print header
            // 
            // === makes it easy to spot debug output in console logs
            println!("=== Debug Info ===");
            println!("Entities: {}", world.entity_count());
            
            // Step 4: Print info about each entity
            // 
            // Iterate over all entities and print their components
            for entity in world.entities() {
                // Start printing this entity's info
                // {:?} is Rust's debug format - it prints the internal structure
                // entity.id() gives us the unique identifier for this entity
                print!("Entity {:?}: ", entity.id());
                
                // Try to get Position component and print it if it exists
                // 
                // if let Some(pos) = ... is pattern matching:
                // - If get_component returns Some(position), extract it as 'pos'
                // - If get_component returns None, skip this block
                if let Some(pos) = world.get_component::<Position>(entity) {
                    // print! (no 'ln') prints without a newline
                    // {:.2} means "format as floating point with 2 decimal places"
                    print!("Pos({:.2}, {:.2}) ", pos.x, pos.y);
                }
                
                // Try to get Velocity component and print it if it exists
                if let Some(vel) = world.get_component::<Velocity>(entity) {
                    print!("Vel({:.2}, {:.2}) ", vel.x, vel.y);
                }
                
                // End the line for this entity
                // println!() prints an empty line (just a newline character)
                println!();
            }
            
            // Print footer to close the debug section
            println!("==================");
        }
        
        // Always return success - debug systems shouldn't fail the simulation
        Ok(())
    }
    
    /// Initialize the debug system
    /// 
    /// This method is called when the system is first added to the world
    /// For the debug system, we just print a startup message
    fn initialize(&mut self, _world: &mut World) -> EcsResult<()> {
        // The underscore prefix (_world) tells Rust "I know this parameter
        // exists but I'm not using it, so don't warn me about it"
        // This is common for trait methods where not all implementations
        // need all parameters
        
        // Print startup message with our configuration
        // {:.2} formats the float with 2 decimal places
        println!("DebugSystem initialized (interval: {:.2}s)", self.print_interval);
        
        Ok(())
    }
}