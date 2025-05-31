use std::any::Any;

/// Trait that all components must implement
/// Components are pure data with no behavior
pub trait Component: Any + Send + Sync {
    /// Get the type name for debugging
    fn type_name() -> &'static str where Self: Sized {
        std::any::type_name::<Self>()
    }
}

/// Automatically implement Component for types that meet the requirements
impl<T> Component for T where T: Any + Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestComponent {
        value: i32,
    }

    #[test]
    fn test_component_trait() {
        let component = TestComponent { value: 42 };
        assert_eq!(TestComponent::type_name(), "ecs::component::tests::TestComponent");
    }
}