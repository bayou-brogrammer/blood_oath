use super::*;

pub struct ItemEquipOnUse {}

impl<'a> System<'a> for ItemEquipOnUse {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player, mut wants_use, names, equippable, mut equipped, mut backpack) = data;

        let mut remove_use: Vec<Entity> = Vec::new();
        for (target, useitem, can_equip) in (&entities, &wants_use)
            .join()
            .filter(|(_, useitem)| equippable.get(useitem.item).is_some())
            .map(|(e, useitem)| (e, useitem, equippable.get(useitem.item).unwrap()))
        {
            let target_slot = can_equip.slot;

            // Remove any items the target has in the item's slot
            let mut to_unequip: Vec<Entity> = Vec::new();
            for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
                if already_equipped.owner == target && already_equipped.slot == target_slot {
                    to_unequip.push(item_entity);
                    if target == *player {
                        bo_logging::Logger::new().append("You unequip").item_name(&name.0).log();
                    }
                }
            }

            for item in to_unequip.iter() {
                equipped.remove(*item);
                backpack
                    .insert(*item, InBackpack { owner: target })
                    .expect("Unable to insert backpack entry");
            }

            // Wield the item
            backpack.remove(useitem.item);
            equipped
                .insert(useitem.item, Equipped { owner: target, slot: target_slot })
                .expect("Unable to insert equipped component");

            if target == *player {
                bo_logging::Logger::new()
                    .append("You equip")
                    .item_name(&names.get(useitem.item).unwrap().0)
                    .log();
            }

            // Done with item
            remove_use.push(target);
        }

        remove_use.iter().for_each(|e| {
            wants_use.remove(*e).expect("Unable to remove");
        });
    }
}
