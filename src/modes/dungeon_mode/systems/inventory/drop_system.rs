use super::*;

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, entities, mut wants_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let dropped_pos = positions.get(entity).unwrap();

            positions.insert(to_drop.item, *dropped_pos).expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                let item_name = names.get(to_drop.item).unwrap().0.clone();
                bo_logging::Logger::new().append("You drop the").item_name(item_name).log();
            }
        }

        wants_drop.clear();
    }
}
