use super::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Point>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut wants_pickup, mut positions, names, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);

            backpack
                .insert(pickup.item, InBackpack::new(pickup.collected_by))
                .expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                let item_name = names.get(pickup.item).unwrap().0.clone();
                bo_logging::Logger::new().append("You pick up the").item_name(item_name).log();
            }
        }

        wants_pickup.clear();
    }
}
