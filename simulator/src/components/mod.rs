// Import statements - bring external types into scope
use nalgebra::Vector2;                    // 2D vector math from nalgebra crate
use serde::{Deserialize, Serialize};      // For converting to/from JSON, binary, etc.

/// 2D position component
/// 
/// This represents where an entity is located in 2D space
/// Components in ECS are just data - no behavior/methods for game logic
/// 
/// The #[derive(...)] is a "derive macro" that automatically generates code:
/// - Debug: Lets you print the struct with {:?} 
/// - Clone: Lets you make copies with .clone()
/// - Copy: Lets you copy with just assignment (very cheap)
/// - PartialEq: Lets you compare with == and !=
/// - Serialize/Deserialize: Lets you save/load to files or send over network
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate in world space
    /// 'pub' means other modules can read/write this field directly
    /// f32 is a 32-bit floating point number (like float in C/Java)
    pub x: f32,
    
    /// Y coordinate in world space  
    /// In 2D games, usually +X = right, +Y = up (or down, depending on system)
    pub y: f32,
}

// Implementation block - where we define methods for Position
// This is like defining class methods in other languages
impl Position {
    /// Create a new Position with specific coordinates
    /// 
    /// This is an "associated function" (like a static method)
    /// No 'self' parameter means you call it like: Position::new(1.0, 2.0)
    /// 'Self' is an alias for the current type (Position)
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }   // Struct literal syntax - creates a new Position
    }
    
    /// Create a position at the origin (0, 0)
    /// 
    /// Another associated function - provides a convenient default
    /// This is a common Rust pattern for creating "default" values
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Convert to a nalgebra Vector2 for math operations
    /// 
    /// This is a method (has &self parameter)  
    /// &self means "borrow self immutably" - we can read but not modify
    /// Methods are called like: position.to_vector()
    pub fn to_vector(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)  // Create a nalgebra vector
    }
    
    /// Create a Position from a nalgebra Vector2
    /// 
    /// Associated function for conversion from vector math
    /// This lets us easily convert between our component and math library
    pub fn from_vector(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

/// 2D velocity component
/// 
/// This represents how fast and in what direction an entity is moving
/// Velocity is typically in units per second (e.g., meters/second, pixels/second)
/// Positive X usually means moving right, positive Y means moving up
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    /// Velocity in X direction (horizontal speed)
    pub x: f32,
    
    /// Velocity in Y direction (vertical speed)  
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity with specific X and Y components
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Create a zero velocity (not moving)
    /// 
    /// Useful for stationary objects or when you want to stop something
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Convert to nalgebra Vector2 for vector math operations
    /// 
    /// Vector math is useful for operations like:
    /// - Adding velocities together
    /// - Rotating velocity vectors  
    /// - Normalizing to unit vectors
    pub fn to_vector(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
    
    /// Create velocity from a nalgebra Vector2
    pub fn from_vector(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
    
    /// Calculate the magnitude (speed) of this velocity
    /// 
    /// Uses the Pythagorean theorem: magnitude = sqrt(x² + y²)
    /// This gives you the overall speed regardless of direction
    /// For example: velocity (3, 4) has magnitude 5
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Rotation component (in radians)
/// 
/// Represents how much an entity is rotated from its default orientation
/// Radians are the standard unit for angles in programming and math:
/// - 0 radians = 0 degrees (facing right, typically)
/// - π/2 radians = 90 degrees  
/// - π radians = 180 degrees
/// - 2π radians = 360 degrees (full circle)
/// 
/// Why radians? Math functions (sin, cos, etc.) expect radians, and they
/// make calculations simpler (no need to convert constantly)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rotation {
    /// Angle in radians
    /// Positive typically means counter-clockwise rotation
    pub angle: f32,
}

impl Rotation {
    /// Create a new rotation with a specific angle in radians
    pub fn new(angle: f32) -> Self {
        Self { angle }
    }
    
    /// Create a rotation of 0 (no rotation)
    pub fn zero() -> Self {
        Self { angle: 0.0 }
    }
    
    /// Create a rotation from degrees
    /// 
    /// Since humans think in degrees but computers prefer radians,
    /// this helper function converts for you
    /// Example: Rotation::degrees(90.0) creates a 90-degree rotation
    pub fn degrees(degrees: f32) -> Self {
        Self { 
            angle: degrees.to_radians()  // Built-in conversion method
        }
    }
    
    /// Convert this rotation to degrees
    /// 
    /// Useful for displaying rotation values to users or debugging
    /// Most people understand "90 degrees" better than "1.57 radians"
    pub fn to_degrees(&self) -> f32 {
        self.angle.to_degrees()  // Built-in conversion method
    }
}

/// Mass component for physics calculations
/// 
/// Represents how much matter an entity contains
/// Mass affects:
/// - How much force is needed to accelerate the object (F = ma)
/// - How objects behave in collisions
/// - Gravitational effects
/// - Inertia (resistance to changes in motion)
/// 
/// Units are typically in kilograms, but can be any consistent unit
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Mass {
    /// Mass value in kilograms (or your chosen unit)
    /// Should be positive - negative mass would be very strange physics!
    pub value: f32,
}

impl Mass {
    /// Create a new mass with a specific value
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

/// Name component for debugging and identification
/// 
/// While not essential for physics, names are incredibly useful for:
/// - Debugging (seeing which entity is which in logs)  
/// - Editor tools (showing meaningful names in lists)
/// - Gameplay (displaying character names, object labels)
/// - Save/load systems (identifying specific entities)
/// 
/// String vs &str: String owns the text data, &str just borrows it
/// We use String here because components need to own their data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    /// The actual name text
    /// String is a growable, owned string type (like std::string in C++)
    pub value: String,
}

impl Name {
    /// Create a new name from a string slice (&str)
    /// 
    /// &str is a borrowed string - it references text stored elsewhere
    /// .to_string() converts it to an owned String that this component can keep
    pub fn new(name: &str) -> Self {
        Self { 
            value: name.to_string()  // Convert borrowed &str to owned String
        }
    }
}

// Trait implementations for Name
// 
// Traits in Rust are like interfaces in other languages - they define behavior
// that types can implement. The 'From' trait is for converting between types.
// 
// These implementations let you create Names in different ways:
// - Name::from("hello")           (from &str)
// - Name::from(my_string)         (from String)  
// - "hello".into()                (using Into, which is auto-implemented)

/// Implement From<&str> for Name
/// 
/// This lets you convert a string slice (&str) directly to a Name
/// Example: let name = Name::from("Player");
/// The From trait is part of Rust's conversion system
impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Name::new(name)  // Just call our existing constructor
    }
}

/// Implement From<String> for Name  
/// 
/// This lets you convert an owned String directly to a Name
/// Example: let name = Name::from(my_string);
/// Since we already own the String, we can just move it in
impl From<String> for Name {
    fn from(name: String) -> Self {
        Self { value: name }  // Move the String directly, no copying needed
    }
}

// Note: When you implement From<T> for a type, Rust automatically implements
// Into<YourType> for T. This means these also work:
// - let name: Name = "hello".into();
// - let name: Name = my_string.into();
// 
// This is part of Rust's "coherence" system that prevents conflicting implementations