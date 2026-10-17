//use super::*;
//mod lib;

#[cfg(test)]
mod tests {
    use classes::{Health, SavingThrowView, SkillProficiency, Statistics};

    #[test]
    fn hp_increase_caps_at_max() {
        let mut h = Health {
            armor_class: 0,
            initiative: "+1".to_string(),
            speed: 30,
            current_hp: 9,
            maximum_hp: 10,
            temporary_hp: 0,
            short_rests: 1,
            hit_dice_type: "d12".to_string(),
            total_hit_dice: 6,
            current_hit_dice: 6,
            unconscious: false,
            death_save_saves: 0,
            death_save_fails: 0,
        };
        h.increase();
        h.increase();
        assert_eq!(h.current_hp, 10);
        assert_eq!(h.temporary_hp, 1);
    }

    #[test]
    fn hp_decrease_stops_at_zero() {
        let mut h = Health {
            armor_class: 0,
            initiative: "+1".to_string(),
            speed: 30,
            current_hp: 1,
            maximum_hp: 10,
            temporary_hp: 0,
            short_rests: 1,
            hit_dice_type: "d12".to_string(),
            total_hit_dice: 6,
            current_hit_dice: 6,
            unconscious: false,
            death_save_saves: 0,
            death_save_fails: 0,
        };
        h.decrease();
        h.decrease();
        assert_eq!(h.current_hp, 0);
    }

    #[test]
    fn hp_decrease_temp_hp() {
        let mut h = Health {
            armor_class: 0,
            initiative: "+1".to_string(),
            speed: 30,
            current_hp: 10,
            maximum_hp: 10,
            temporary_hp: 2,
            short_rests: 1,
            hit_dice_type: "d12".to_string(),
            total_hit_dice: 6,
            current_hit_dice: 6,
            unconscious: false,
            death_save_saves: 0,
            death_save_fails: 0,
        };
        h.decrease();
        h.decrease();
        h.decrease();
        assert_eq!(h.temporary_hp, 0);
        assert_eq!(h.current_hp, 9);
    }

    #[test]
    fn skills_proficiency_symbol_none() {
        let skill_prof: SkillProficiency = SkillProficiency::None;
        assert_eq!(skill_prof.symbol(), "○");
    }

    #[test]
    fn skills_proficiency_symbol_proficient() {
        let skill_prof: SkillProficiency = SkillProficiency::Proficient;
        assert_eq!(skill_prof.symbol(), "●");
    }

    #[test]
    fn skills_proficiency_symbol_expertise() {
        let skill_prof: SkillProficiency = SkillProficiency::Expertise;
        assert_eq!(skill_prof.symbol(), "◎");
    }

    #[test]
    fn saving_throw_view_symbol_none() {
        let stv: SavingThrowView = SavingThrowView {
            name: "test",
            value: 1,
            proficient: false,
        };
        assert_eq!(stv.symbol(), "○");
    }

    #[test]
    fn saving_throw_view_symbol_proficient() {
        let stv: SavingThrowView = SavingThrowView {
            name: "test",
            value: 1,
            proficient: true,
        };
        assert_eq!(stv.symbol(), "●");
    }

    #[test]
    fn ability_mod_test_all() {
        let mut stats: Statistics = Statistics {
            strength: 1,
            dexterity: 2,
            constitution: 3,
            intelligence: 4,
            wisdom: 5,
            charisma: 6,
            inspiration: false,
            proficiency_bonus: 0,
            passive_wisdom_perception: 8,
        };
        assert_eq!(stats.ability_mod(stats.strength), -5);
        assert_eq!(stats.ability_mod(stats.dexterity), -4);
        assert_eq!(stats.ability_mod(stats.constitution), -4);
        assert_eq!(stats.ability_mod(stats.intelligence), -3);
        assert_eq!(stats.ability_mod(stats.wisdom), -3);
        assert_eq!(stats.ability_mod(stats.charisma), -2);
        stats.strength = 7;
        stats.dexterity = 8;
        stats.constitution = 9;
        stats.intelligence = 10;
        stats.wisdom = 11;
        stats.charisma = 12;
        assert_eq!(stats.ability_mod(stats.strength), -2);
        assert_eq!(stats.ability_mod(stats.dexterity), -1);
        assert_eq!(stats.ability_mod(stats.constitution), -1);
        assert_eq!(stats.ability_mod(stats.intelligence), 0);
        assert_eq!(stats.ability_mod(stats.wisdom), 0);
        assert_eq!(stats.ability_mod(stats.charisma), 1);
        stats.strength = 13;
        stats.dexterity = 14;
        stats.constitution = 15;
        stats.intelligence = 16;
        stats.wisdom = 17;
        stats.charisma = 18;
        assert_eq!(stats.ability_mod(stats.strength), 1);
        assert_eq!(stats.ability_mod(stats.dexterity), 2);
        assert_eq!(stats.ability_mod(stats.constitution), 2);
        assert_eq!(stats.ability_mod(stats.intelligence), 3);
        assert_eq!(stats.ability_mod(stats.wisdom), 3);
        assert_eq!(stats.ability_mod(stats.charisma), 4);
        stats.strength = 19;
        stats.dexterity = 20;
        stats.constitution = 21;
        stats.intelligence = 22;
        stats.wisdom = 23;
        stats.charisma = 24;
        assert_eq!(stats.ability_mod(stats.strength), 4);
        assert_eq!(stats.ability_mod(stats.dexterity), 5);
        assert_eq!(stats.ability_mod(stats.constitution), 5);
        assert_eq!(stats.ability_mod(stats.intelligence), 6);
        assert_eq!(stats.ability_mod(stats.wisdom), 6);
        assert_eq!(stats.ability_mod(stats.charisma), 7);
        stats.strength = 25;
        stats.dexterity = 26;
        stats.constitution = 27;
        stats.intelligence = 28;
        stats.wisdom = 29;
        stats.charisma = 30;
        assert_eq!(stats.ability_mod(stats.strength), 7);
        assert_eq!(stats.ability_mod(stats.dexterity), 8);
        assert_eq!(stats.ability_mod(stats.constitution), 8);
        assert_eq!(stats.ability_mod(stats.intelligence), 9);
        assert_eq!(stats.ability_mod(stats.wisdom), 9);
        assert_eq!(stats.ability_mod(stats.charisma), 10);
    }
}
