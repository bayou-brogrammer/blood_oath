use super::*;
use bo_ecs::construct_dispatcher;

mod damage_system;
mod end_turn;
mod fov_system;
mod inventory;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod particle_system;
mod render;

pub use damage_system::{DamageSystem, DeleteDeadSystem};
pub use end_turn::EndTurnSystem;
pub use fov_system::FovSystem;
pub use inventory::*;
pub use map_indexing_system::MapIndexingSystem;
pub use melee_combat_system::MeleeCombatSystem;
pub use monster_ai_system::MonsterAISystem;
pub use particle_system::{ParticleSpawnSystem, ParticleUpdateSystem};
pub use render::RenderSystem;

pub fn new_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!(
        (FovSystem, "fov", &[]),
        (MonsterAISystem, "ai_system", &[]),
        (MapIndexingSystem, "map_indexing", &[]),
        (MeleeCombatSystem, "melee_combat", &[]),
        (DamageSystem, "damage", &[]),
        (ItemCollectionSystem, "pickup", &[]),
        (ItemUseSystem, "use", &[]),
        (ItemDropSystem, "drop", &[]),
        (EndTurnSystem, "end_turn", &[]),
        (ParticleSpawnSystem, "particle_spawn", &[]),
        (ParticleUpdateSystem, "particle_update", &[]),
        (DeleteDeadSystem, "delete_dead", &[])
    );

    new_dispatch_with_local(RenderSystem)
}
