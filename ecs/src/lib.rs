use std::any::Any;
use std::collections::HashMap;
use slotmap::DefaultKey;
use anyhow::Result;

pub mod component;
pub mod entity;
pub mod system;
pub mod world;

pub use component::Component;
pub use entity::Entity;
pub use system::{System, SystemDispatcher};
pub use world::World;

/// Core ECS error types
#[derive(thiserror::Error, Debug)]
pub enum EcsError {
    #[error("Entity {0:?} not found")]
    EntityNotFound(Entity),
    #[error("Component not registered: {0}")]
    ComponentNotRegistered(String),
    #[error("System error: {0}")]
    SystemError(String),
}

/// Type alias for ECS results
pub type EcsResult<T> = Result<T, EcsError>;

/// Entity ID type using slotmap for efficient storage
pub type EntityId = DefaultKey;

/// Component storage trait for type erasure
pub trait ComponentStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity: EntityId) -> bool;
}

/// Concrete component storage implementation
pub struct TypedComponentStorage<T: Component> {
    components: HashMap<EntityId, T>,
}

impl<T: Component> TypedComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.components.insert(entity, component);
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.components.remove(&entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.components.iter().map(|(id, comp)| (*id, comp))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.components.iter_mut().map(|(id, comp)| (*id, comp))
    }
}

impl<T: Component> ComponentStorage for TypedComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, entity: EntityId) -> bool {
        self.components.remove(&entity).is_some()
    }
}