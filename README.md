# ECS-Based Aerodynamic Simulator

A high-performance, custom Entity Component System (ECS) framework built in Rust, designed as the foundation for an aerodynamic flight simulator with reinforcement learning capabilities.

## 🎯 Project Overview

This project demonstrates advanced systems programming, simulation architecture, and AI integration by building a modular flight dynamics simulator from the ground up. The custom ECS engine provides a performant, memory-safe foundation for complex physics simulations and reinforcement learning experiments.

**Key Features:**
- ✅ Custom ECS framework built entirely from scratch in Rust
- ✅ Type-safe component management with runtime flexibility
- ✅ Frame-rate independent physics simulation loop
- ✅ Modular workspace architecture for easy extension
- ✅ Comprehensive error handling and memory safety
- ✅ Full test coverage with automated validation

## 📊 Current Status: Phase 1 Complete

### ✅ Completed: Core ECS Framework
- [x] Entity management with SlotMap for efficient allocation
- [x] Component system with type-safe storage and querying
- [x] System dispatcher for organized game logic
- [x] World management with comprehensive error handling
- [x] Basic movement and debug systems
- [x] Complete test suite (100% pass rate)

### 🚧 In Progress: Phase 2 - Physics & Aerodynamics
- [ ] Rigid body physics implementation
- [ ] Force and torque integration
- [ ] Aerodynamic lift and drag calculations
- [ ] Wind and turbulence modeling

### 📋 Planned: Future Phases
- **Phase 3**: Simulation environment with RL API
- **Phase 4**: Reinforcement learning agent integration
- **Phase 5**: Visualization and performance analysis
- **Phase 6**: Advanced features (3D flight, multi-agent, etc.)

## 🏗️ Architecture

### Workspace Structure

```
aerodynamic_simulator/
├── ecs/                    # Core ECS framework (reusable)
│   ├── component.rs       # Component trait definition
│   ├── entity.rs          # Entity type implementation
│   ├── system.rs          # System trait & dispatcher
│   └── world.rs           # Central ECS management
├── physics/               # Physics simulation (Phase 2)
├── aerodynamics/          # Aerodynamic modeling (Phase 2)
├── rl_interface/          # RL integration (Phase 4)
└── simulator/             # Main application
    ├── components/        # Simulation-specific components
    ├── systems/           # Simulation systems
    └── world/             # Simulation world management
```

### Design Philosophy

The project follows **Entity Component System** architectural principles:

- **Entities**: Unique identifiers for game objects
- **Components**: Pure data containers with no behavior
- **Systems**: Logic that operates on entities with specific components

This provides:
- **Composition over inheritance**: Flexible entity definitions
- **Data locality**: Cache-friendly component storage
- **Parallel processing potential**: Independent system execution
- **Runtime flexibility**: Dynamic component management

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd aerodynamic_simulator

# Build the project
cargo build --release

# Run tests
cargo test --all

# Run the simulator
cargo run --release --bin simulator
```

### Running the Demo

The current Phase 1 demo creates a simple simulation with moving entities:

```bash
cargo run --release --bin simulator
```

**Expected Output:**
```
🚀 Aerodynamic Simulator Starting...
=====================================
Initial simulation state:
  Entities: 3
  Systems: 2
  Time Step: 0.0167s

Running simulation...
=== Debug Info ===
Entities: 3
Entity EntityId { ... }: Pos(5.00, 2.50) Vel(10.00, 5.00)
Entity EntityId { ... }: Pos(50.00, 30.00) Vel(0.00, 0.00)
Entity EntityId { ... }: Pos(-27.50, 14.00) Vel(-15.00, 8.00)
==================

Simulation completed!
  Total time: 5.00s
  Final entity count: 3
=====================================
✅ ECS-based Simulator Demo Complete
```

## 🧪 Testing

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific crate tests
cargo test -p ecs
cargo test -p simulator

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --all --out Html
```

**Current Test Coverage:**
- 6 ECS core tests (entity management, components, world)
- 2 simulator integration tests
- 0 failures, 100% pass rate

## 📚 Documentation

### Building Documentation

```bash
# Generate and open documentation
cargo doc --all --open

# Generate documentation without dependencies
cargo doc --no-deps --open
```

### Key Documentation Files

- **[Technical Implementation Document](ECS-Based_Aerodynamic_Simulator__Technical_Implementation_Document.md)**: Comprehensive technical details, architecture decisions, and implementation notes
- **[Project Specification](ECS_Aerodynamics_RL_Project_Spec.pdf)**: Phase-by-phase development plan and deliverables

## 💡 Usage Examples

### Creating Custom Components

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, maximum: max }
    }
}
```

### Creating Custom Systems

```rust
use ecs::{System, World, EcsResult};
use crate::components::{Health, Position};

pub struct HealthSystem;

impl System for HealthSystem {
    fn name(&self) -> &str {
        "HealthSystem"
    }
    
    fn run(&mut self, world: &mut World, delta_time: f32) -> EcsResult<()> {
        for entity in world.entities().collect::<Vec<_>>() {
            if let Some(health) = world.get_component_mut::<Health>(entity) {
                // Regenerate health over time
                health.current = (health.current + delta_time).min(health.maximum);
            }
        }
        Ok(())
    }
}
```

### Building a Simulation

```rust
use simulator::world::SimWorld;

fn main() -> anyhow::Result<()> {
    let mut sim = SimWorld::new();
    sim.initialize()?;
    
    // Add entities and components
    let aircraft = sim.world.create_entity();
    sim.world.add_component(aircraft, Position::new(0.0, 100.0))?;
    sim.world.add_component(aircraft, Velocity::new(50.0, 0.0))?;
    
    // Run simulation
    sim.run_for(10.0)?;
    
    Ok(())
}
```

## 🛠️ Development

### Project Structure

Each crate serves a specific purpose:

- **`ecs/`**: Core ECS framework (reusable for other projects)
- **`physics/`**: Physics engine (forces, integration, collisions)
- **`aerodynamics/`**: Aerodynamic calculations (lift, drag, wind)
- **`rl_interface/`**: Reinforcement learning API and Python bindings
- **`simulator/`**: Main application tying everything together

### Key Technologies

- **Rust**: Systems programming language with memory safety
- **SlotMap**: Efficient entity storage with generational indices
- **nalgebra**: Linear algebra and vector mathematics
- **serde**: Serialization framework for save/load
- **PyO3**: Python bindings for RL integration (Phase 4)

### Code Style

The project follows standard Rust conventions:

```bash
# Format code
cargo fmt --all

# Check for common mistakes
cargo clippy --all -- -D warnings

# Check compilation without building
cargo check --all
```

## 🎓 Learning Resources

This project demonstrates several advanced Rust concepts:

- **Ownership & Borrowing**: Safe memory management without garbage collection
- **Trait Objects**: Dynamic dispatch for polymorphic systems
- **Type Erasure**: Storing heterogeneous component types
- **Generics**: Type-safe, zero-cost abstractions
- **Error Handling**: Result types and error propagation
- **Testing**: Unit tests, integration tests, and documentation tests

## 📈 Performance

### Current Metrics

- **Build Time**: <5 seconds (incremental)
- **Test Execution**: <1 second (all tests)
- **Memory Safety**: 0 unsafe blocks
- **Entity Operations**: O(1) average case
- **Component Access**: O(1) average case

### Future Optimizations

- Parallel system execution
- Component chunking for cache optimization
- Query indexing for common patterns
- SIMD vectorization for physics calculations

## 🤝 Contributing

This is a portfolio/demonstration project, but suggestions and feedback are welcome:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 Roadmap

### Phase 2: Physics & Aerodynamics (Next)
- Implement force integration system
- Add gravity and air resistance
- Create aerodynamic lift/drag calculations
- Model control surfaces (elevator, rudder, ailerons)

### Phase 3: RL Environment
- Define observation space (state vector)
- Define action space (control inputs)
- Implement reward function
- Create episode termination logic

### Phase 4: AI Integration
- Set up Python RL training pipeline
- Implement PPO or SAC agent
- Create training loop with episode collection
- Export trained policy for inference

### Phase 5: Visualization
- 2D flight visualization with macroquad
- Real-time trajectory plotting
- Performance metrics dashboard
- Episode replay system

## 📄 License

This project is available for educational and portfolio purposes. Please contact for commercial use.

## 🙏 Acknowledgments

- **Rust Community**: For excellent documentation and libraries
- **ECS Architecture**: Inspired by specs, bevy, and legion
- **Physics Simulation**: Based on game physics literature

## 📧 Contact

For questions, suggestions, or collaboration opportunities, please reach out through:
- GitHub Issues
- Email: [your-email@example.com]
- LinkedIn: [your-profile]

---

**Built with ❤️ and Rust** | *Demonstrating systems programming, simulation architecture, and AI integration*
