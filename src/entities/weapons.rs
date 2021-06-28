use crate::structs::parsed::weapons;
use crate::structs::raw::{
    common, equip_affix_excel_config_data, material_excel_config_data,
    weapon_curve_excel_config_data, weapon_excel_config_data, weapon_promote_excel_config_data,
};
use crate::utils::{readable::Readable, remove_xml, texthash::TextHash};

use std::fs;

fn load_data() -> crate::Result<(
    Vec<weapon_excel_config_data::Data>,
    Vec<weapon_curve_excel_config_data::Data>,
    Vec<weapon_promote_excel_config_data::Data>,
    Vec<equip_affix_excel_config_data::Data>,
    Vec<material_excel_config_data::Data>,
)> {
    let weapon_data =
        fs::read_to_string("./data/ExcelBinOutput/WeaponExcelConfigData.json").unwrap();
    let weapon_configs: Vec<weapon_excel_config_data::Data> =
        serde_json::from_str(&weapon_data).unwrap();

    let weapon_curve_data =
        fs::read_to_string("./data/ExcelBinOutput/WeaponCurveExcelConfigData.json").unwrap();
    let weapon_curves: Vec<weapon_curve_excel_config_data::Data> =
        serde_json::from_str(&weapon_curve_data).unwrap();

    let weapon_promote_data =
        fs::read_to_string("./data/ExcelBinOutput/WeaponPromoteExcelConfigData.json").unwrap();
    let weapon_promotes: Vec<weapon_promote_excel_config_data::Data> =
        serde_json::from_str(&weapon_promote_data).unwrap();

    let equip_data =
        fs::read_to_string("./data/ExcelBinOutput/EquipAffixExcelConfigData.json").unwrap();
    let equip_affixs: Vec<equip_affix_excel_config_data::Data> =
        serde_json::from_str(&equip_data).unwrap();

    let material_data =
        fs::read_to_string("./data/ExcelBinOutput/MaterialExcelConfigData.json").unwrap();
    let materials: Vec<material_excel_config_data::Data> =
        serde_json::from_str(&material_data).unwrap();
    Ok((
        weapon_configs,
        weapon_curves,
        weapon_promotes,
        equip_affixs,
        materials,
    ))
}

fn parse_ascensions(
    promotes: Vec<&weapon_promote_excel_config_data::Data>,
    materials: &Vec<material_excel_config_data::Data>,
    texthash: &TextHash,
) -> Vec<weapons::Ascension> {
    let mut ascensions: Vec<weapons::Ascension> = vec![];

    for promote in promotes.iter() {
        let mut recipe: Vec<weapons::AscensionRecipe> = vec![];

        for item in promote.cost_items.iter() {
            if item.id.is_none() {
                continue;
            }
            if item.count.is_none() {
                continue;
            }

            let item_id = match item.id {
                Some(id) => id,
                None => 0,
            };

            let item_count = match item.count {
                Some(count) => count,
                None => 0,
            };
            let recipe_item = weapons::AscensionRecipe {
                name: match materials.iter().find(|material| material.id == item_id) {
                    Some(material) => texthash.get_match(&material.name_text_map_hash).unwrap(),
                    None => "".to_owned(),
                },
                count: item_count,
                game_id: item_id,
            };

            recipe.push(recipe_item);
        }

        let ascension = weapons::Ascension {
            level: promote.promote_level.unwrap_or_default(),
            recipe: recipe,
            cost: promote.coin_cost.unwrap_or_default(),
            max_level: promote.unlock_max_level,
            required_player_level: promote.required_player_level.unwrap_or_default(),
        };

        ascensions.push(ascension);
    }

    ascensions
}

fn parse_passive(
    equip_affixs: Vec<&equip_affix_excel_config_data::Data>,
    texthash: &TextHash,
) -> Option<weapons::Passive> {
    if equip_affixs.len() == 0 {
        return None;
    }

    let name = texthash
        .get_match(&equip_affixs[0].name_text_map_hash)
        .unwrap();
    let levels = equip_affixs
        .iter()
        .map(|affix| weapons::PassiveLevel {
            description: remove_xml::remove_xml(
                texthash.get_match(&affix.desc_text_map_hash).unwrap(),
            )
            .unwrap(),
            additional_properties: affix
                .add_props
                .iter()
                .filter(|prop| prop.prop_type.is_some())
                .cloned()
                .collect(),
            params: affix.param_list.clone(),
            level: match affix.level {
                Some(level) => level + 1,
                None => 1,
            },
            extra_config: affix.open_config.clone(),
            extra_id: affix.id,
        })
        .collect();
    Some(weapons::Passive { name, levels })
}

struct PromoteLevel {
    max_level: usize,
    add_atk: f64,
}

fn get_promotion_additinal_base_atk(
    promotes: &Vec<weapon_promote_excel_config_data::Data>,
    level: usize,
) -> f64 {
    let promote_levels: Vec<PromoteLevel> = promotes
        .iter()
        .map(|promote| {
            let add_prop = promote.add_props.iter().find(|prop| {
                prop.prop_type.as_ref().unwrap() == &common::SubStatPropType::FightPropBaseAttack
            });
            let max_level = promote.unlock_max_level;
            let add_atk = match add_prop {
                Some(prop) => match prop.value {
                    Some(value) => value,
                    None => 0f64,
                },
                None => 0f64,
            };
            PromoteLevel {
                max_level: max_level,
                add_atk: add_atk,
            }
        })
        .collect();

    if promote_levels.len() >= 1 && level <= promote_levels[0].max_level {
        return promote_levels[0].add_atk;
    }

    if promote_levels.len() >= 2
        && level > promote_levels[0].max_level
        && level <= promote_levels[1].max_level
    {
        return promote_levels[1].add_atk;
    }
    if promote_levels.len() >= 3
        && level > promote_levels[1].max_level
        && level <= promote_levels[2].max_level
    {
        return promote_levels[2].add_atk;
    }
    if promote_levels.len() >= 4
        && level > promote_levels[2].max_level
        && level <= promote_levels[3].max_level
    {
        return promote_levels[3].add_atk;
    }
    if promote_levels.len() >= 5
        && level > promote_levels[3].max_level
        && level <= promote_levels[4].max_level
    {
        return promote_levels[4].add_atk;
    }
    if promote_levels.len() >= 6
        && level > promote_levels[4].max_level
        && level <= promote_levels[5].max_level
    {
        return promote_levels[5].add_atk;
    }
    if promote_levels.len() >= 7
        && level > promote_levels[5].max_level
        && level <= promote_levels[6].max_level
    {
        return promote_levels[6].add_atk;
    }

    0f64
}

fn create_attack_scale(
    weapon_curves: &Vec<weapon_curve_excel_config_data::Data>,
    options: (
        Option<f64>,
        String,
        &Vec<weapon_promote_excel_config_data::Data>,
        usize,
    ),
) -> Vec<weapons::AttackLevel> {
    let mut scale: Vec<weapons::AttackLevel> = vec![];

    let init_value = match options.0 {
        Some(value) => value,
        None => 0f64,
    };
    let curve_type = options.1;
    let promotes = options.2;
    let weapon_id = options.3;

    for curve in weapon_curves.iter() {
        let promote_base_attack = get_promotion_additinal_base_atk(&promotes, curve.level);

        if curve.level > 21 && promote_base_attack == 0f64 {
            continue;
        }

        let value = match curve
            .curve_infos
            .iter()
            .find(|info| info.r#type == curve_type)
        {
            Some(curve) => curve.value * init_value,
            None => 1 as f64,
        };

        scale.push(weapons::AttackLevel {
            level: curve.level,
            value: value,
            upgraded: false,
        })
    }

    for (i, promote) in promotes
        .iter()
        .filter(|promote| promote.weapon_promote_id == weapon_id)
        .enumerate()
    {
        let promote_add_prop = match promotes.len() >= i + 2 {
            true => promotes[i + 1]
                .add_props
                .iter()
                .find(|prop| match prop.prop_type.clone() {
                    Some(prop_type) => prop_type == common::SubStatPropType::FightPropBaseAttack,
                    None => false,
                }),
            false => None,
        };

        if promote_add_prop.is_none() {
            continue;
        }

        let value = match weapon_curves
            .iter()
            .find(|curve| curve.level == promote.unlock_max_level)
        {
            Some(curve) => match curve
                .curve_infos
                .iter()
                .find(|info| info.r#type == curve_type)
            {
                Some(curve) => {
                    let promote_base_attack =
                        get_promotion_additinal_base_atk(&promotes, promote.unlock_max_level);
                    (curve.value * init_value) + promote_base_attack
                }
                None => 1 as f64,
            },
            None => 1 as f64,
        };

        scale.push(weapons::AttackLevel {
            level: promote.unlock_max_level,
            value: value,
            upgraded: true,
        })
    }

    scale
}

fn create_sub_stat_scale(
    weapon_curves: &Vec<weapon_curve_excel_config_data::Data>,
    options: (
        Option<f64>,
        String,
        Option<&weapon_promote_excel_config_data::Data>,
    ),
) -> Vec<weapons::SubStatLevel> {
    let mut scale: Vec<weapons::SubStatLevel> = vec![];

    let init_value = match options.0 {
        Some(value) => value,
        None => 0f64,
    };
    let curve_type = options.1;
    let max_level = match options.2 {
        Some(level) => level.unlock_max_level,
        None => 90,
    };

    for curve in weapon_curves.iter() {
        if curve.level > max_level {
            continue;
        }

        let value = match curve
            .curve_infos
            .iter()
            .find(|info| info.r#type == curve_type)
        {
            Some(curve) => curve.value * init_value,
            None => 1 as f64,
        };

        if value == 0.0f64 {
            continue;
        }

        scale.push(weapons::SubStatLevel {
            level: curve.level,
            value: value,
        })
    }

    scale
}

pub fn parse(texthash: TextHash, readable: Readable) -> crate::Result<Vec<weapons::Data>> {
    let (weapon_configs, weapon_curves, weapon_promotes, equip_affixs, materials) =
        load_data().unwrap();
    let mut weapon_data: Vec<weapons::Data> = vec![];

    for weapon in weapon_configs {
        let weapon_id = weapon.id;
        let weapon_type = &weapon.weapon_type;
        let name = texthash.get_match(&weapon.name_text_map_hash).unwrap();
        let description =
            remove_xml::remove_xml(texthash.get_match(&weapon.desc_text_map_hash).unwrap())
                .unwrap();
        let lore = remove_xml::remove_xml(
            readable
                .get_match(format!("Weapon{}", &weapon.id).as_str())
                .unwrap(),
        )
        .unwrap();
        let weapon_passive: Vec<&equip_affix_excel_config_data::Data> = equip_affixs
            .iter()
            .filter(|affix| affix.id == weapon.skill_affix[0])
            .collect();
        let mut promotes: Vec<&weapon_promote_excel_config_data::Data> = weapon_promotes
            .iter()
            .filter(|promote| promote.weapon_promote_id == weapon.id)
            .collect();
        let passive = parse_passive(weapon_passive, &texthash);
        let rarity = weapon.rank_level;

        let attack_levels = create_attack_scale(
            &weapon_curves,
            (
                weapon.weapon_prop[0].init_value,
                weapon.weapon_prop[0].r#type.clone(),
                &weapon_promotes,
                weapon_id,
            ),
        );
        let sub_stat_levels = match weapon.weapon_prop.len() >= 2 {
            true => create_sub_stat_scale(
                &weapon_curves,
                (
                    weapon.weapon_prop[1].init_value,
                    weapon.weapon_prop[1].r#type.clone(),
                    weapon_promotes.last(),
                ),
            ),
            false => vec![],
        };
        let weapon_sub_stat = match weapon.weapon_prop.len() >= 2 {
            true => match &weapon.weapon_prop[1].prop_type {
                Some(prop_type) => prop_type.to_string(),
                None => "".to_owned(),
            },
            false => "".to_owned(),
        };

        promotes.rotate_right(1);
        let ascensions = parse_ascensions(promotes, &materials, &texthash);

        let weapon = weapons::Data {
            weapon_id: weapon_id,
            weapon: weapons::Weapon {
                r#type: weapon_type.to_string(),
                name: name,
                description: description,
                lore: lore,
                passive: passive,
                rarity: rarity,
                attack_levels: attack_levels,
                sub_stat_levels: sub_stat_levels,
                sub_stat_type: weapon_sub_stat,
            },
            ascensions: ascensions,
        };

        weapon_data.push(weapon);
    }

    Ok(weapon_data)
}
