# Aethaum

[简体中文](README.md) | [English](README_en.md)

**Important**: This project is still in early development stage and currently has no available release.



A declarative framework for building interactive worlds for AI agents.

Aethaum enables you to define complex game worlds and AI interaction environments through simple TOML configurations, without writing intricate Rust code.

## 🎯 Core Principles

- **Declarative Configuration**: Define ECS structures using TOML
- **Logic Separation**: Separate configuration from logic with Lua handling runtime behavior
- **Automatic Translation**: TOML configurations automatically convert to high-performance Rust code
- **Flexible Extension**: Generated Rust projects can be directly extended and customized

## 🚀 Features

- 📝 **TOML-Based ECS Configuration** - Clean and readable configuration syntax
- 🐍 **Lua Scripting Logic** - Flexible runtime logic implementation
- 🦀 **Bevy ECS Integration** - Powerful ECS engine support
- 🔥 **Hot Reload Support** - Real-time updates during development
- 🎮 **AI-Friendly Design** - Built specifically for AI agent interactions

## 🏗️ Project Architecture

world/
├── world.toml                 # World configuration file
├── systems/                   # System definitions directory
├── components/                # Component definitions directory
├── events/                    # Event definitions directory
└── entity_protos/             # Entity prototype directory

## 📋 Configuration Details

### 🌍 world.toml - World Configuration

```toml
[world]
name = "MyAIWorld"
version = "0.1.0"
author = "Your Name"

[includes]
systems = [
    "movement.toml",
    "combat.toml"
]
components = [
    "health.toml",
    "position.toml"
]
events = [
    "damage.toml",
    "death.toml"
]
entity_protos = [
    "player.toml",
    "enemy.toml"
]

[build]
output_dir = "generated"

[cargo]
# Standard Cargo configuration
```

### ⚙️ systems/*.toml - System Definitions

```toml
# Only one system can be defined per TOML file
[normal]
name = "HealthSystem"
description = "Handles health value updates for entities"
category = "combat"
priority = 100

# Component query definitions
[[queries]]
name = "living_entities"
components = ["Health", "Position"]
description = "Query all living entities"

[[queries]]
name = "damaged_entities"
components = ["Health", "Damage"]
description = "Query damaged entities"

[update]
interval = 0.1  # Update interval in seconds

# Update condition (Lua)
condition = '''
return entity.health.value > 0
'''

# Update logic (Lua)
logic = '''
entity.health.value = entity.health.value - entity.damage.amount
entity.damage.amount = 0
'''

# Event handlers
[[event_handlers]]
watch_for = "EntityDamaged"
priority = 10
logic = '''
entity.health.value = entity.health.value - event.damage
'''

[[event_handlers]]
watch_for = "EntityHealed"
priority = 20
logic = '''
entity.health.value = math.min(
    entity.health.value + event.amount,
    entity.health.max_value
)
```

### 🧩 components/*.toml - Component Definitions

```toml
# Multiple components can be defined in one TOML file. The 'normal' field is metadata and doesn't participate in translation
[normal]
tags = ["combat", "stats"]
description = "Combat-related components"

# Health component
[[components]]
name = "Health"
description = "Entity health properties"

[[components.fields]]
name = "value"
type = "f32"
default = 100.0
description = "Current health value"

[[components.fields]]
name = "max_value"
type = "f32"
default = 100.0
description = "Maximum health value"

# Position component
[[components]]
name = "Position"
description = "Entity position coordinates"

[[components.fields]]
name = "x"
type = "f32"
default = 0.0
description = "X coordinate"

[[components.fields]]
name = "y"
type = "f32"
default = 0.0
description = "Y coordinate"
```

### ⚡ events/*.toml - Event Definitions

```toml
# Multiple events can be defined in one TOML file. The 'normal' field is metadata and doesn't participate in translation
[normal]
tags = ["combat", "interaction"]
description = "Combat and interaction events"

# Damage event
[[events]]
name = "EntityDamaged"
description = "Entity receives damage"

[[events.fields]]
name = "damage"
type = "f32"
description = "Amount of damage"

[[events.fields]]
name = "attacker"
type = "EntityId"
description = "Attacker entity ID"

# Healing event
[[events]]
name = "EntityHealed"
description = "Entity receives healing"

[[events.fields]]
name = "amount"
type = "f32"
description = "Amount of healing"

[[events.fields]]
name = "healer"
type = "EntityId"
description = "Healer entity ID"
```

### 🏗️ entity_protos/*.toml - Entity Prototypes

```toml
# Multiple entity prototypes can be defined in one TOML file. The 'normal' field is metadata and doesn't participate in translation
[normal]
tags = ["characters", "npcs"]
description = "Character entity prototypes"

# Player prototype
[[entity_protos]]
name = "Player"
components = ["Health", "Position", "PlayerControlled"]
description = "Player character prototype"

# Enemy prototype
[[entity_protos]]
name = "Enemy"
components = ["Health", "Position", "AIControlled"]
description = "Enemy character prototype"

# Item prototype
[[entity_protos]]
name = "HealthPotion"
components = ["Item", "Consumable"]
description = "Health restoration potion"
```

## 🚀 Quick Start

- Currently under development and not yet available

## 🛠️ Development Workflow

1. **Define Components** - Create TOML files in `components/` directory
2. **Create Systems** - Define system logic in `systems/` directory
3. **Design Events** - Create event structures in `events/` directory
4. **Build Prototypes** - Create entity templates in `entity_protos/` directory
5. **Configure World** - Edit `world.toml` to include all definitions
6. **Build and Run** - Use CLI tools for building and execution

## 🎯 Use Cases

- 🤖 **AI Training Environments** - Provides simple interactive worlds for AI agents
- 🤖 **AI Role-Playing** - Enables AI characters to accumulate experiences that can be transformed into memories
- 🎮 **Prototype Development** - Rapidly build small game prototypes
- 🔬 **Simulation Experiments** - Simulate complex system behaviors

## 🤝 Contribution

Feel free to submit Issues and Pull Requests!

## 📄 License

MIT License