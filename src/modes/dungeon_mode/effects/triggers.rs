use super::*;

pub fn item_trigger(creator: Option<Entity>, item: Entity, targets: &Targets, ecs: &mut World) {
    // Use the item via the generic system
    let did_something = event_trigger(creator, item, targets, ecs);

    // If it was a consumable, then it gets deleted
    if did_something && ecs.read_storage::<Consumable>().get(item).is_some() {
        ecs.entities().delete(item).expect("Delete Failed");
    }
}

// pub fn trigger(creator: Option<Entity>, trigger: Entity, targets: &Targets, ecs: &mut World) {
//     // The triggering item is no longer hidden
//     ecs.write_storage::<Hidden>().remove(trigger);

//     // Use the item via the generic system
//     let did_something = event_trigger(creator, trigger, targets, ecs);

//     // If it was a single activation, then it gets deleted
//     if did_something && ecs.read_storage::<SingleActivation>().get(trigger).is_some() {
//         ecs.entities().delete(trigger).expect("Delete Failed");
//     }
// }

fn event_trigger(creator: Option<Entity>, entity: Entity, targets: &Targets, ecs: &mut World) -> bool {
    let mut did_something = false;

    // Providing food
    if ecs.read_storage::<ProvidesFood>().get(entity).is_some() {
        did_something = true;
        let names = ecs.read_storage::<Name>();

        add_effect(creator, EffectType::WellFed, targets.clone());
        bo_logging::Logger::new().append("You eat the").item_name(&names.get(entity).unwrap().0).log();
    }

    // Healing
    if let Some(heal) = ecs.read_storage::<ProvidesHealing>().get(entity) {
        add_effect(creator, EffectType::Healing { amount: heal.heal_amount }, targets.clone());
        did_something = true;
    }

    // Damage
    if let Some(damage) = ecs.read_storage::<InflictsDamage>().get(entity) {
        add_effect(creator, EffectType::Damage { amount: damage.damage }, targets.clone());
        did_something = true;
    }

    // Confusion
    if let Some(confusion) = ecs.read_storage::<Confusion>().get(entity) {
        add_effect(creator, EffectType::Confusion { turns: confusion.turns }, targets.clone());
        did_something = true;
    }

    // Magic mapper
    if ecs.read_storage::<MagicMapper>().get(entity).is_some() {
        bo_logging::Logger::new().append("The map is revealed to you!").log();
        let mut runstate = ecs.fetch_mut::<TurnState>();
        *runstate = TurnState::MagicMapReveal(0);
        did_something = true;
    }

    did_something
}
