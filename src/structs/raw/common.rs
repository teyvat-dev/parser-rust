use serde::{Deserialize, Serialize};

impl ToString for SubStatPropType {
    fn to_string(&self) -> String {
        match self {
            SubStatPropType::FightPropShieldCostMinusRatio => {
                "ShieldCostMinusRatio".to_owned()
            }
            SubStatPropType::FightPropDefensePercent => {
                "DefensePercent".to_owned()
            }
            SubStatPropType::FightPropHp => {
                "Hp".to_owned()
            }
            SubStatPropType::FightPropHpPercent => {
                "HpPercent".to_owned()
            }
            SubStatPropType::FightPropDefense => {
                "Defence".to_owned()
            }
            SubStatPropType::FightPropHealedAdd => {
                "HealedAdd".to_owned()
            }
            SubStatPropType::FightPropHealAdd => {
                "HealAdd".to_owned()
            }
            SubStatPropType::FightPropBaseAttack => {
                "BaseAttack".to_owned()
            }
            SubStatPropType::FightPropAddHurt => {
                "AddHurt".to_owned()
            }
            SubStatPropType::FightPropAttackPercent => {
                "AttackPercent".to_owned()
            }
            SubStatPropType::FightPropCriticalHurt => {
                "CriticalHurt".to_owned()
            }
            SubStatPropType::FightPropCritical => {
                "Critical".to_owned()
            }
            SubStatPropType::FightPropElementMastery => {
                "ElementMastery".to_owned()
            }
            SubStatPropType::FightPropChargeEfficiency => {
                "ChargeEfficiency".to_owned()
            }
            SubStatPropType::FightPropPhysicalAddHurt => {
                "PhysicalAddHurt".to_owned()
            }
            SubStatPropType::FightPropIceAddHurt => {
                "IceAddHurt".to_owned()
            }
            SubStatPropType::FightPropFireAddHurt => {
                "FireAddHurt".to_owned()
            }
            SubStatPropType::FightPropGrassAddHurt => {
                "GrassAddHurt".to_owned()
            }
            SubStatPropType::FightPropWaterAddHurt => {
                "WaterAddHurt".to_owned()
            }
            SubStatPropType::FightPropElecAddHurt => {
                "ElecAddHurt".to_owned()
            }
            SubStatPropType::FightPropWindAddHurt => {
                "WindAddHurt".to_owned()
            }
            SubStatPropType::FightPropRockAddHurt => {
                "RockAddHurt".to_owned()
            }
            SubStatPropType::FightPropIceSubHurt => {
                "IceSubHurt".to_owned()
            }
            SubStatPropType::FightPropFireSubHurt => {
                "FireSubHurt".to_owned()
            }
            SubStatPropType::FightPropGrassSubHurt => {
                "GrassSubHurt".to_owned()
            }
            SubStatPropType::FightPropWaterSubHurt => {
                "WaterSubHurt".to_owned()
            }
            SubStatPropType::FightPropElecSubHurt => {
                "ElecSubHurt".to_owned()
            }
            SubStatPropType::FightPropWindSubHurt => {
                "WindSubHurt".to_owned()
            }
            SubStatPropType::FightPropRockSubHurt => {
                "RockSubHurt".to_owned()
            }
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubStatPropType {
    FightPropShieldCostMinusRatio,
    FightPropDefensePercent,
    FightPropHp,
    FightPropHpPercent,
    FightPropDefense,
    FightPropHealedAdd,
    FightPropHealAdd,
    FightPropBaseAttack,
    FightPropAddHurt,
    FightPropAttackPercent,
    FightPropCriticalHurt,
    FightPropCritical,
    FightPropElementMastery,
    FightPropChargeEfficiency,
    FightPropPhysicalAddHurt,
    FightPropIceAddHurt,
    FightPropFireAddHurt,
    FightPropGrassAddHurt,
    FightPropWaterAddHurt,
    FightPropElecAddHurt,
    FightPropWindAddHurt,
    FightPropRockAddHurt,
    FightPropIceSubHurt,
    FightPropFireSubHurt,
    FightPropGrassSubHurt,
    FightPropWaterSubHurt,
    FightPropElecSubHurt,
    FightPropWindSubHurt,
    FightPropRockSubHurt,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AddProp {
    pub prop_type: Option<SubStatPropType>,
    pub value: Option<f64>,
}
