use super::*;

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, Equippable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, map, mut wants_use, aoe, equippable) = data;

        for (entity, useitem, ()) in (&entities, &wants_use, !&equippable).join() {
            // Call the effects system
            add_effect(
                Some(entity),
                EffectType::ItemUse(useitem.item),
                match useitem.target {
                    None => Targets::Single(*player_entity),
                    Some(target) => {
                        if let Some(aoe) = aoe.get(useitem.item) {
                            Targets::Tiles(aoe_tiles(&map, target, aoe.radius))
                        } else {
                            Targets::Tile(map.point2d_to_index(target))
                        }
                    }
                },
            );
        }

        wants_use.clear();
    }
}
