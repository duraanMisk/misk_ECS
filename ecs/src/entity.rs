use crate::EntityId;

/// Entity is just a unique identifier
/// All entity logic is handled by the World
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: EntityId,
}

impl Entity {
    /// Create a new Entity with the given ID
    pub fn new(id: EntityId) -> Self {
        Self { id }
    }

    /// Get the internal ID
    pub fn id(&self) -> EntityId {
        self.id
    }
}

impl From<EntityId> for Entity {
    fn from(id: EntityId) -> Self {
        Entity::new(id)
    }
}

impl From<Entity> for EntityId {
    fn from(entity: Entity) -> Self {
        entity.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    #[test]
    fn test_entity_creation() {
        let mut slot_map: SlotMap<EntityId, ()> = SlotMap::new();
        let id = slot_map.insert(());
        let entity = Entity::new(id);
        
        assert_eq!(entity.id(), id);
        assert_eq!(EntityId::from(entity), id);
    }
}