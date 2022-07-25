use super::*;
use bo_ecs::construct_dispatcher;

mod damage_system;
mod end_turn;
mod fov_system;
mod inventory;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;

use damage_system::{DamageSystem, DeleteDeadSystem};
use end_turn::EndTurnSystem;
use fov_system::FovSystem;
use inventory::*;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use monster_ai_system::MonsterAISystem;

pub fn new_ticking_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!(
        (FovSystem, "fov", &[]),
        (MonsterAISystem, "ai_system", &[]),
        (MapIndexingSystem, "map_indexing", &[]),
        (MeleeCombatSystem, "melee_combat", &[]),
        (DamageSystem, "damage", &[]),
        (ItemCollectionSystem, "pickup", &[]),
        (ItemUseSystem, "use", &[]),
        (ItemDropSystem, "drop", &[]),
        (DeleteDeadSystem, "delete_dead", &[]),
        (EndTurnSystem, "end_turn", &[])
    );
}
