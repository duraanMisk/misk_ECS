// Import statements - these bring types and functions from other modules into scope
use std::any::TypeId;           // Rust's runtime type identification
use std::collections::HashMap;  // Hash table for key-value storage
use slotmap::SlotMap;          // Efficient sparse array for entities

// Import our own types from other files in this crate
use crate::{
    Component, ComponentStorage, Entity, EntityId, EcsError, EcsResult,
    TypedComponentStorage,
};

/// The World manages all entities and their components
/// 
/// In Rust, 'pub' means this struct is public (visible outside this module)
/// The World is the central data structure that stores all game objects (entities)
/// and their data (components) in an efficient way
pub struct World {
    /// Entity storage using SlotMap for efficient allocation/deallocation
    /// SlotMap<K, V> is like Vec<V> but allows gaps and reuses indices
    /// This prevents the "dangling pointer" problem when entities are deleted
    entities: SlotMap<EntityId, Entity>,
    
    /// Component storages indexed by TypeId
    /// HashMap<K, V> is Rust's hash table - like a dictionary in Python
    /// TypeId is Rust's way to identify types at runtime
    /// Box<dyn ComponentStorage> is a "trait object" - it can hold any type
    /// that implements ComponentStorage. 'dyn' means "dynamic dispatch"
    component_storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
    
    /// Track which entities have which component types (for queries)
    /// This lets us quickly find "all entities with Position AND Velocity"
    entity_component_masks: HashMap<EntityId, Vec<TypeId>>,
}

// Implementation block - this is where we define methods for the World struct
// 'impl' is like defining class methods in other languages
impl World {
    /// Create a new empty world
    /// 
    /// In Rust, 'Self' refers to the current type (World)
    /// This is a "constructor" - it creates and returns a new World instance
    /// All the collections start empty
    pub fn new() -> Self {
        Self {
            entities: SlotMap::new(),                    // Empty entity storage
            component_storages: HashMap::new(),          // Empty component storage
            entity_component_masks: HashMap::new(),      // Empty component masks
        }
    }
    
    /// Create a new entity
    /// 
    /// '&mut self' means this method needs mutable access to the World
    /// In Rust, you can only have ONE mutable reference at a time (prevents data races)
    /// This method modifies the World by adding a new entity
    pub fn create_entity(&mut self) -> Entity {
        // insert_with_key is a SlotMap method that gives us the key (ID) when inserting
        // The closure |id| Entity::new(id) creates an Entity with the generated ID
        // Closures in Rust are like lambda functions in other languages
        let id = self.entities.insert_with_key(|id| Entity::new(id));
        
        // Initialize empty component mask for this entity
        // .insert() returns Option<T> of the old value, but we ignore it here
        self.entity_component_masks.insert(id, Vec::new());
        
        // Return the new entity
        Entity::new(id)
    }
    
    /// Remove an entity and all its components
    /// 
    /// This method returns EcsResult<()> which is Result<(), EcsError>
    /// Result<T, E> is Rust's way of handling errors (no exceptions!)
    /// Ok(()) means success with no return value
    /// Err(error) means something went wrong
    pub fn remove_entity(&mut self, entity: Entity) -> EcsResult<()> {
        let id = entity.id();  // Get the internal ID from the entity
        
        // Check if entity exists - contains_key() returns bool
        if !self.entities.contains_key(id) {
            // Return an error if entity doesn't exist
            // The '?' operator would propagate this error to the caller
            return Err(EcsError::EntityNotFound(entity));
        }
        
        // Remove all components for this entity
        // .values_mut() gives us mutable references to all values in the HashMap
        // 'for' loops in Rust automatically handle borrowing
        for storage in self.component_storages.values_mut() {
            storage.remove(id);  // ComponentStorage trait method
        }
        
        // Remove entity from SlotMap - this frees up the ID for reuse
        self.entities.remove(id);
        // Remove component mask
        self.entity_component_masks.remove(&id);
        
        // Return success (the () is called "unit type" - like void in C)
        Ok(())
    }
    
    /// Check if an entity exists
    /// 
    /// This method only needs to read the World, so it takes '&self' (immutable reference)
    /// Multiple immutable references are allowed simultaneously in Rust
    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity.id())
    }
    
    /// Register a component type
    /// 
    /// Generic function: <T: Component> means T can be any type that implements Component
    /// This is like templates in C++ or generics in Java/C#
    /// The 'Component' after the colon is a "trait bound" - T must implement Component
    pub fn register_component<T: Component>(&mut self) {
        // TypeId::of::<T>() gets a unique identifier for type T at runtime
        // This lets us store different component types in the same HashMap
        let type_id = TypeId::of::<T>();
        
        // Check if this component type is already registered
        if !self.component_storages.contains_key(&type_id) {
            // Create a new storage for this component type
            let storage = TypedComponentStorage::<T>::new();
            
            // Box::new() puts the storage on the heap (dynamic allocation)
            // This is necessary because we're storing different types in the same HashMap
            self.component_storages.insert(type_id, Box::new(storage));
        }
    }
    
    /// Add a component to an entity
    /// 
    /// This is where Rust's ownership system really shines
    /// 'component: T' means we take ownership of the component data
    /// The component is "moved" into this function and can't be used by the caller anymore
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) -> EcsResult<()> {
        let id = entity.id();
        
        // Verify entity exists
        if !self.entities.contains_key(id) {
            return Err(EcsError::EntityNotFound(entity));
        }
        
        let type_id = TypeId::of::<T>();
        
        // Ensure component type is registered
        self.register_component::<T>();
        
        // Get the storage for this component type
        // .get_mut() returns Option<&mut V> - either Some(reference) or None
        // .ok_or_else() converts None to an Error
        let storage = self.component_storages.get_mut(&type_id)
            .ok_or_else(|| EcsError::ComponentNotRegistered(T::type_name().to_string()))?;
        
        // This is called "downcasting" - converting from trait object back to concrete type
        // .as_any_mut() returns &mut dyn Any (the most general trait object)
        // .downcast_mut() tries to convert it back to our specific type
        // If the cast fails, it returns None
        let typed_storage = storage.as_any_mut()
            .downcast_mut::<TypedComponentStorage<T>>()
            .ok_or_else(|| EcsError::ComponentNotRegistered(T::type_name().to_string()))?;
        
        // Actually store the component data
        typed_storage.insert(id, component);
        
        // Update component mask - track that this entity has this component type
        if let Some(mask) = self.entity_component_masks.get_mut(&id) {
            // Only add if not already present (no duplicates)
            if !mask.contains(&type_id) {
                mask.push(type_id);
            }
        }
        
        Ok(())
    }
    
    /// Get a component from an entity
    /// 
    /// Returns Option<&T> - either Some(reference to component) or None
    /// The &T is an immutable reference - you can read but not modify
    /// Option<T> is Rust's way of representing "maybe has a value"
    /// It's much safer than null pointers!
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let id = entity.id();
        let type_id = TypeId::of::<T>();
        
        // Try to get the storage for this component type
        // The ? operator here is different - it converts None to None and continues if Some
        let storage = self.component_storages.get(&type_id)?;
        
        // Downcast from trait object to concrete type (immutable version)
        let typed_storage = storage.as_any()
            .downcast_ref::<TypedComponentStorage<T>>()?;
        
        // Get the component for this specific entity
        typed_storage.get(id)
    }
    
    /// Get a mutable component from an entity
    /// 
    /// Returns Option<&mut T> - mutable reference if found
    /// &mut T means you can both read AND modify the component
    /// Rust ensures only ONE mutable reference exists at a time (no data races!)
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let id = entity.id();
        let type_id = TypeId::of::<T>();
        
        // Same pattern but with mutable references
        let storage = self.component_storages.get_mut(&type_id)?;
        let typed_storage = storage.as_any_mut()
            .downcast_mut::<TypedComponentStorage<T>>()?;
        
        typed_storage.get_mut(id)
    }
    
    /// Remove a component from an entity
    /// 
    /// Returns EcsResult<Option<T>> - nested result types
    /// The outer Result handles entity-not-found errors
    /// The inner Option tells us if the component existed (Some) or not (None)
    /// The T means we give back the component data (ownership transfer)
    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> EcsResult<Option<T>> {
        let id = entity.id();
        let type_id = TypeId::of::<T>();
        
        // Verify entity exists first
        if !self.entities.contains_key(id) {
            return Err(EcsError::EntityNotFound(entity));
        }
        
        // Get mutable storage
        let storage = self.component_storages.get_mut(&type_id)
            .ok_or_else(|| EcsError::ComponentNotRegistered(T::type_name().to_string()))?;
        
        let typed_storage = storage.as_any_mut()
            .downcast_mut::<TypedComponentStorage<T>>()
            .ok_or_else(|| EcsError::ComponentNotRegistered(T::type_name().to_string()))?;
        
        // Remove and get the component data
        let component = typed_storage.remove(id);
        
        // Update component mask - remove this type from the entity's list
        if let Some(mask) = self.entity_component_masks.get_mut(&id) {
            // .retain() keeps only elements that match the condition
            // |&t| means the closure takes a reference to each element
            mask.retain(|&t| t != type_id);
        }
        
        Ok(component)
    }
    
    /// Check if an entity has a specific component
    /// 
    /// Simple boolean check - useful for filtering entities
    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        let id = entity.id();
        let type_id = TypeId::of::<T>();
        
        // Chain of Option operations:
        // 1. Get the component mask for this entity (returns Option)
        // 2. If found, check if it contains the type ID (returns Option<bool>)  
        // 3. If not found, default to false
        self.entity_component_masks
            .get(&id)
            .map(|mask| mask.contains(&type_id))  // .map() transforms Some(mask) to Some(bool)
            .unwrap_or(false)                     // .unwrap_or() converts None to false
    }
    
    /// Query entities that have all specified component types
    /// 
    /// Takes a slice (&[TypeId]) of type IDs to search for
    /// Returns a Vec<Entity> containing all matching entities
    /// This is how we implement queries like "find all entities with Position AND Velocity"
    pub fn query_entities(&self, component_types: &[TypeId]) -> Vec<Entity> {
        self.entity_component_masks
            .iter()                                    // Iterate over all entities and their masks
            .filter(|(_, mask)| {                     // Filter to only entities that match
                // .all() returns true if every component type is in the entity's mask
                component_types.iter().all(|&type_id| mask.contains(&type_id))
            })
            .map(|(&id, _)| Entity::new(id))          // Convert from (EntityId, &Vec<TypeId>) to Entity
            .collect()                                // Collect the iterator into a Vec
    }
    
    /// Get all entities
    /// 
    /// The return type is complex: impl Iterator<Item = Entity> + '_
    /// "impl Iterator" means "some type that implements Iterator"
    /// The + '_ part is a lifetime annotation - it means the iterator
    /// borrows from self and can't outlive this World instance
    /// This is much more efficient than collecting into a Vec!
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        // .values() gets an iterator over all entities in the SlotMap
        // .copied() converts from Iterator<&Entity> to Iterator<Entity>
        // Since Entity is Copy, this is very cheap (just copying a small ID)
        self.entities.values().copied()
    }
    
    /// Get the number of entities
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    
    /// Get component storage for iteration
    /// 
    /// This allows systems to iterate over all components of a specific type
    /// Returns Option<&TypedComponentStorage<T>> - reference to the storage
    /// The storage lets you iterate over all entities that have component T
    pub fn get_component_storage<T: Component>(&self) -> Option<&TypedComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        
        // Get the storage and try to downcast it
        let storage = self.component_storages.get(&type_id)?;
        storage.as_any().downcast_ref::<TypedComponentStorage<T>>()
    }
    
    /// Get mutable component storage for iteration
    /// 
    /// Same as above but allows modification of components during iteration
    /// Systems use this to update component data efficiently
    pub fn get_component_storage_mut<T: Component>(&mut self) -> Option<&mut TypedComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        
        let storage = self.component_storages.get_mut(&type_id)?;
        storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>()
    }
}

// Implement the Default trait for World
// This is Rust's standard way to provide a "default" constructor
// It lets you write World::default() instead of World::new()
// Many Rust APIs expect types to implement Default
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// Conditional compilation - this code only exists in test builds
// Run tests with: cargo test
#[cfg(test)]
mod tests {
    use super::*;  // Import everything from the parent module

    // Test components - simple structs for testing
    #[derive(Debug, PartialEq)]  // Automatic implementations for debugging and comparison
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    // Test function - #[test] makes this run when you do 'cargo test'
    #[test]
    fn test_world_entity_creation() {
        let mut world = World::new();
        assert_eq!(world.entity_count(), 0);  // assert_eq! panics if values aren't equal
        
        let entity = world.create_entity();
        assert_eq!(world.entity_count(), 1);
        assert!(world.entity_exists(entity));  // assert! panics if condition is false
    }

    #[test]
    fn test_world_component_operations() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        let pos = Position { x: 1.0, y: 2.0 };
        world.add_component(entity, pos).unwrap();  // .unwrap() panics if Result is Err
        
        assert!(world.has_component::<Position>(entity));
        assert!(!world.has_component::<Velocity>(entity));
        
        // .unwrap() here panics if Option is None - we expect Some(component)
        let retrieved_pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(retrieved_pos.x, 1.0);
        assert_eq!(retrieved_pos.y, 2.0);
    }

    #[test]
    fn test_world_entity_removal() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        world.add_component(entity, Position { x: 0.0, y: 0.0 }).unwrap();
        assert!(world.has_component::<Position>(entity));
        
        world.remove_entity(entity).unwrap();
        assert!(!world.entity_exists(entity));
        assert_eq!(world.entity_count(), 0);
    }
}