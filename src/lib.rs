use serde::{Deserialize, Serialize};
// TODO: before pushing this change this lib to not rely on ratatui... lol
use ratatui::widgets::ListItem;

use serde_json::Deserializer;
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

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
            death_save_saves: "".to_string(),
            death_save_fails: "".to_string(),
        };
        h.increase();
        h.increase();
        assert_eq!(h.current_hp, 10);
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
            death_save_saves: "".to_string(),
            death_save_fails: "".to_string(),
        };
        h.decrease();
        h.decrease();
        assert_eq!(h.current_hp, 0);
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

#[derive(Clone, Copy)]
pub struct StatView {
    pub name: &'static str,
    pub value: u8,
    pub modifier: i8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Information {
    pub character_name: String,
    pub class: String,
    pub level: u8,
    pub player_name: String,
    pub race: String,
    pub alignment: String,
    pub experience: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
// Modifiers will be calculated based on rules of the game
pub struct Statistics {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub inspiration: bool,
    pub proficiency_bonus: u8,
    pub passive_wisdom_perception: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct SavingThrows {
    pub strength_proficent: bool,
    pub dexterity_proficent: bool,
    pub constitution_proficent: bool,
    pub intelligence_proficent: bool,
    pub wisdom_proficent: bool,
    pub charisma_proficent: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Skills {
    pub acrobatics: String,
    pub animal_handling: String,
    pub arcana: String,
    pub athletics: String,
    pub deception: String,
    pub history: String,
    pub insight: String,
    pub intimidation: String,
    pub investigation: String,
    pub medicine: String,
    pub nature: String,
    pub perception: String,
    pub performance: String,
    pub persuasion: String,
    pub religion: String,
    pub slight_of_hand: String,
    pub stealth: String,
    pub survival: String,
    pub acrobatics_skill: String,
    pub animal_handling_skill: String,
    pub arcana_skill: String,
    pub athletics_skill: String,
    pub deception_skill: String,
    pub history_skill: String,
    pub insight_skill: String,
    pub intimidation_skill: String,
    pub investigation_skill: String,
    pub medicine_skill: String,
    pub nature_skill: String,
    pub perception_skill: String,
    pub performance_skill: String,
    pub persuasion_skill: String,
    pub religion_skill: String,
    pub slight_of_hand_skill: String,
    pub stealth_skill: String,
    pub survival_skill: String,
}
#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ProficienciesAndLanguage {
    pub languages_known: String,
    pub armor_proficiency: String,
    pub weapon_proficiency: String,
    pub tools_proficiency: String,
}
#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Health {
    pub armor_class: u8,
    pub initiative: String,
    pub speed: u8,
    pub current_hp: u8,
    pub maximum_hp: u8,
    pub temporary_hp: u8,
    pub short_rests: u8,
    pub hit_dice_type: String,
    pub total_hit_dice: u8,
    pub current_hit_dice: u8,
    pub unconscious: bool,
    pub death_save_saves: String,
    pub death_save_fails: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Background {
    background: String,
    background_fulltext: String,
    personality_traits: String,
    ideals: String,
    bonds: String,
    flaws: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct CharSheet {
    pub information: Information,
    pub statistics: Statistics,
    pub saving_throws: SavingThrows,
    pub skills: Skills,
    pub proficiencies_and_language: ProficienciesAndLanguage,
    pub health: Health,
    pub background: Background,
    pub classes: Character, //pub traits: Traits,
}

impl Information {
    #[allow(dead_code)]
    pub fn information_to_list_item(&self) -> Vec<ListItem<'static>> {
        vec![
            ListItem::new(format!("Char Name: {}", self.character_name)),
            ListItem::new(format!("Class: {}", self.class)),
            ListItem::new(format!("Level: {}", self.level)),
            ListItem::new(format!("Player Name: {}", self.player_name)),
            ListItem::new(format!("Race: {}", self.race)),
            ListItem::new(format!("Alignment: {}", self.alignment)),
            ListItem::new(format!("Experience: {}", self.experience)),
        ]
    }
}

impl Health {
    pub fn increase(&mut self) {
        if self.current_hp == self.maximum_hp {
            self.temporary_hp += 1;
        }
        self.current_hp = (self.current_hp + 1).min(self.maximum_hp);
    }

    pub fn decrease(&mut self) {
        if self.temporary_hp == 0 {
            if self.current_hp >= 1 {
                self.current_hp -= 1;
            } else {
                self.current_hp = 0;
            }
        } else {
            if self.temporary_hp >= 1 {
                self.temporary_hp -= 1;
            } else {
                self.temporary_hp = 0;
            }
        }
    }

    pub fn short_rest(&mut self) {}

    pub fn long_rest(&mut self) {
        self.current_hp = self.maximum_hp;
        self.short_rests = 0;
    }
}

impl SavingThrows {
    pub fn saving_throw_views(&self, stats: &Statistics) -> [SavingThrowView; 6] {
        [
            Self::saving_throw(
                "STR",
                stats.strength,
                self.strength_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "DEX",
                stats.dexterity,
                self.dexterity_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "CON",
                stats.constitution,
                self.constitution_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "INT",
                stats.intelligence,
                self.intelligence_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "WIS",
                stats.wisdom,
                self.wisdom_proficent,
                stats.proficiency_bonus,
                stats,
            ),
            Self::saving_throw(
                "CHA",
                stats.charisma,
                self.charisma_proficent,
                stats.proficiency_bonus,
                stats,
            ),
        ]
    }

    fn saving_throw(
        name: &'static str,
        score: u8,
        proficient: bool,
        prof_bonus: u8,
        stats: &Statistics,
    ) -> SavingThrowView {
        let mut value = Statistics::ability_mod(stats, score);

        if proficient {
            value += prof_bonus as i8;
        }

        SavingThrowView {
            name,
            value,
            proficient,
        }
    }
}
#[derive(Clone, Copy)]
pub struct SavingThrowView {
    pub name: &'static str,
    pub value: i8,
    pub proficient: bool,
}

impl SavingThrowView {
    pub fn symbol(&self) -> &'static str {
        if !self.proficient {
            "○"
        } else {
            "●"
        }
    }
}

#[derive(Clone, Copy)]
pub struct SkillsView {
    pub name: &'static str,
    pub value: i8,
    pub sp: SkillProficiency,
}

impl Skills {
    pub fn skills_views(&self) -> [SkillsView; 18] {
        [
            Self::skills(
                "Acrobatics (Dex)",
                self.acrobatics.clone(),
                self.acrobatics_skill.clone(),
            ),
            Self::skills(
                "Animal Handling (Wis)",
                self.animal_handling.clone(),
                self.animal_handling_skill.clone(),
            ),
            Self::skills(
                "Arcana (Int)",
                self.arcana.clone(),
                self.arcana_skill.clone(),
            ),
            Self::skills(
                "Athletics (Str)",
                self.athletics.clone(),
                self.athletics_skill.clone(),
            ),
            Self::skills(
                "Deception (Dex)",
                self.deception.clone(),
                self.deception_skill.clone(),
            ),
            Self::skills(
                "History (Int)",
                self.history.clone(),
                self.history_skill.clone(),
            ),
            Self::skills(
                "Insight (Wis)",
                self.insight.clone(),
                self.insight_skill.clone(),
            ),
            Self::skills(
                "Intimidation (Cha)",
                self.intimidation.clone(),
                self.intimidation_skill.clone(),
            ),
            Self::skills(
                "Investigation (Int)",
                self.investigation.clone(),
                self.investigation_skill.clone(),
            ),
            Self::skills(
                "Medicine (Wis)",
                self.medicine.clone(),
                self.medicine_skill.clone(),
            ),
            Self::skills(
                "Nature (Int)",
                self.nature.clone(),
                self.nature_skill.clone(),
            ),
            Self::skills(
                "Perception (Wis)",
                self.perception.clone(),
                self.perception_skill.clone(),
            ),
            Self::skills(
                "Performance (Cha)",
                self.performance.clone(),
                self.performance_skill.clone(),
            ),
            Self::skills(
                "Persuasion (Cha)",
                self.persuasion.clone(),
                self.persuasion_skill.clone(),
            ),
            Self::skills(
                "Religion (Int)",
                self.religion.clone(),
                self.religion_skill.clone(),
            ),
            Self::skills(
                "Slight of Hand (Dex)",
                self.slight_of_hand.clone(),
                self.slight_of_hand_skill.clone(),
            ),
            Self::skills(
                "Stealth (Dex)",
                self.stealth.clone(),
                self.stealth_skill.clone(),
            ),
            Self::skills(
                "Survival (Wis)",
                self.survival.clone(),
                self.survival_skill.clone(),
            ),
        ]
    }

    fn skills(name: &'static str, score: String, proficient_str: String) -> SkillsView {
        let value;
        let mut sp: SkillProficiency = SkillProficiency::None;

        if score.is_empty() {
            value = 0;
        } else {
            let value_opt = parse_string(&score);
            match value_opt {
                Some(val) => {
                    value = val;
                }
                None => {
                    value = 0;
                }
            }
        }

        if proficient_str == "proficient" {
            sp = SkillProficiency::Proficient;
        } else if proficient_str == "expertise" {
            sp = SkillProficiency::Expertise;
        }

        SkillsView { name, value, sp }
    }
}

fn parse_string(s: &str) -> Option<i8> {
    match s.parse::<i8>() {
        Ok(num) => Some(num),
        Err(e) => {
            println!("Failed to parse \"{}\": {}", s, e);
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SkillProficiency {
    None,
    Proficient,
    Expertise,
}

impl SkillProficiency {
    pub fn symbol(self) -> &'static str {
        match self {
            SkillProficiency::None => "○",
            SkillProficiency::Proficient => "●",
            SkillProficiency::Expertise => "◎",
        }
    }
}

impl Statistics {
    pub fn ability_mod(&self, stat: u8) -> i8 {
        let mut modifier: i8 = (stat as i8 - 10) / 2;
        if stat < 10 && stat % 2 == 1 {
            modifier -= 1;
        }
        modifier
    }

    pub fn ability_scores(&self) -> [StatView; 6] {
        [
            StatView {
                name: "STR",
                value: self.strength,
                modifier: self.ability_mod(self.strength),
            },
            StatView {
                name: "DEX",
                value: self.dexterity,
                modifier: self.ability_mod(self.dexterity),
            },
            StatView {
                name: "CON",
                value: self.constitution,
                modifier: self.ability_mod(self.constitution),
            },
            StatView {
                name: "INT",
                value: self.intelligence,
                modifier: self.ability_mod(self.intelligence),
            },
            StatView {
                name: "WIS",
                value: self.wisdom,
                modifier: self.ability_mod(self.wisdom),
            },
            StatView {
                name: "CHA",
                value: self.charisma,
                modifier: self.ability_mod(self.charisma),
            },
        ]
    }

    pub fn insp_toggle(&mut self) -> bool {
        self.inspiration = !self.inspiration;
        self.inspiration
    }
}

impl ProficienciesAndLanguage {
    pub fn profs_and_lang_to_list_item(&self) -> Vec<ListItem<'static>> {
        vec![
            ListItem::new(format!("Languages Known: {}", self.languages_known)),
            ListItem::new(format!("Armor Proficiency: {}", self.armor_proficiency)),
            ListItem::new(format!("Tools Proficiency: {}", self.tools_proficiency)),
            ListItem::new(format!("Weapon Proficiency: {}", self.weapon_proficiency)),
        ]
    }
}

impl Background {
    pub fn background_to_list_item(&self) -> Vec<ListItem<'static>> {
        vec![
            ListItem::new(format!("Background: {}", self.background)),
            ListItem::new(format!("Personality Traits: {}", self.personality_traits)),
            ListItem::new(format!("Ideals: {}", self.ideals)),
            ListItem::new(format!("Bonds: {}", self.bonds)),
            ListItem::new(format!("Flaws: {}", self.flaws)),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassLevel {
    //#[default]
    Barbarian {
        level: u8,
        subclass: Option<BarbarianSubclass>,
        rages_used: u8,
        totem_spirits: Vec<TotemSpirit>,
    },
    Sorcerer {
        subclass: Option<SorcererSubclass>,
        level: u8,
    },
    /*    Bard {
            subclass: Option<BardSubclass>,
            level: u8,
        },
        Cleric {
            subclass: Option<ClericSubclass>,
            level: u8,
        },
        Druid {
            subclass: Option<DruidSubclass>,
            level: u8,
        },
        Fighter {
            subclass: Option<FighterSubclass>,
            level: u8,
        },
        Monk {
            subclass: Option<MonkSubclass>,
            level: u8,
        },
        Paladin {
            subclass: Option<PaladinSubclass>,
            level: u8,
        },
        Ranger {
            subclass: Option<RangerSubclass>,
            level: u8,
        },
        Rogue {
            subclass: Option<RogueSubclass>,
            level: u8,
        },
        Sorcerer {
            subclass: Option<SorcererSubclass>,
            level: u8,
        },
        Warlock {
            subclass: Option<WarlockSubclass>,
            level: u8,
        },
        Wizard {
            subclass: Option<WizardSubclass>,
            level: u8,
        },
    */
}

impl ClassLevel {
    #[allow(dead_code)]
    fn default() -> Self {
        ClassLevel::Barbarian {
            level: 1,
            subclass: None,
            rages_used: 0,
            totem_spirits: Vec::new(),
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            ClassLevel::Barbarian { level, .. } | ClassLevel::Sorcerer { level, .. } => *level,
            /*
                        | ClassLevel::Bard { level, .. }
                        | ClassLevel::Cleric { level, .. }
                        | ClassLevel::Druid { level, .. }
                        | ClassLevel::Fighter { level, .. }
                        | ClassLevel::Monk { level, .. }
                        | ClassLevel::Paladin { level, .. }
                        | ClassLevel::Ranger { level, .. }
                        | ClassLevel::Rogue { level, .. }
                        | ClassLevel::Sorcerer { level, .. }
                        | ClassLevel::Warlock { level, .. }
                        | ClassLevel::Wizard { level, .. } => *level,
            */
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum FighterSubclass {
//     Champion,
//     BattleMaster,
//     EldritchKnight,
//     // ...
// }

/// PHB + common Barbarian subclasses ("Primal Paths").
/// Chosen at level 3.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarbarianSubclass {
    AncestralGuardian,
    Beast,
    Berserker,
    #[default]
    Giant,
    StormHerald,
    Totem,
    WildMagic,
    Zealot,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum SorcererSubclass {
    #[default]
    DraconicSorcery,
    DivineSoul,
    ShadowMagic,
    StormSorcery,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub classes: Vec<ClassLevel>,
    // name, race, abilities, etc.
}

impl Character {
    pub fn total_level(&self) -> u8 {
        self.classes.iter().map(ClassLevel::level).sum()
    }
}

/// Barbarian-specific mechanical state that isn't just "level".
/// This is the data that actually changes character-sheet behavior
/// (rage tracking, damage bonus, etc.) beyond what a bare u8 level tells you.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Barbarian {
    pub level: u8,
    pub subclass: Option<BarbarianSubclass>,

    /// How many rages the character has used since their last long rest.
    /// Compared against `rages_per_long_rest()` to know how many remain.
    pub rages_used: u8,

    /// Totem spirits chosen (for the Totem Warrior path), if any.
    /// Kept separate from `subclass` since a character could multiclass
    /// out of Barbarian and this data would still be relevant to display.
    pub totem_spirits: Vec<TotemSpirit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TotemSpirit {
    Bear,
    Eagle,
    Wolf,
    Elk,
    Tiger,
}

impl Barbarian {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            subclass: None,
            level: 1,
            rages_used: 0,
            totem_spirits: Vec::new(),
        }
    }

    pub fn new(level: u8) -> Self {
        Self {
            level,
            subclass: None,
            rages_used: 0,
            totem_spirits: Vec::new(),
        }
    }

    /// Number of rages per long rest, per the PHB table.
    /// Returns None at level 20 (unlimited rages).
    pub fn rages_per_long_rest(&self) -> Option<u8> {
        match self.level {
            1..=2 => Some(2),
            3..=5 => Some(3),
            6..=11 => Some(4),
            12..=16 => Some(5),
            17..=19 => Some(6),
            20 => None,
            _ => None, // invalid level; caller should validate 1..=20 upstream
        }
    }

    pub fn rages_remaining(&self) -> Option<u8> {
        self.rages_per_long_rest()
            .map(|max| max.saturating_sub(self.rages_used))
    }

    /// Bonus damage on melee weapon attacks while raging, per the PHB table.
    pub fn rage_damage_bonus(&self) -> i8 {
        match self.level {
            1..=8 => 2,
            9..=15 => 3,
            16..=20 => 4,
            _ => 0,
        }
    }

    pub fn has_unarmored_defense(&self) -> bool {
        self.level >= 1
    }

    pub fn has_extra_attack(&self) -> bool {
        self.level >= 5
    }

    pub fn has_fast_movement(&self) -> bool {
        self.level >= 5
    }

    pub fn on_long_rest(&mut self) {
        self.rages_used = 0;
    }
}

/*
 *
pub struct CharSheet {
    pub information: Information,
    pub statistics: Statistics,
    pub saving_throws: SavingThrows,
    pub skills: Skills,
    pub proficiencies_and_language: ProficienciesAndLanguage,
    pub health: Health,
    pub background: Background,
    pub classes: Character, //pub traits: Traits,
}
 */

pub fn parse_char_sheet<R: Read>(reader: R) -> Result<CharSheet, Box<dyn std::error::Error>> {
    // 1. Create a stateful stream deserializer from your reader
    let mut stream = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // 2. Pull out the information, non-vector structs sequentially
    let information_json = stream.next().ok_or("Missing information struct")??;
    let information: Information = serde_json::from_value(information_json)?;
    println!("Parsed Information: {:?}", information);

    let statistics_json = stream.next().ok_or("Missing statistics struct")??;
    let statistics: Statistics = serde_json::from_value(statistics_json)?;
    println!("Parsed Statistics: {:?}", statistics);

    let saving_throws_json = stream.next().ok_or("Missing saving throws struct")??;
    let saving_throws: SavingThrows = serde_json::from_value(saving_throws_json)?;
    println!("Parsed SavingThrows: {:?}", saving_throws);

    let skills_json = stream.next().ok_or("Missing header")??;
    let skills: Skills = serde_json::from_value(skills_json)?;
    println!("Parsed Skills: {:?}", skills);

    let proficiencies_and_language_json = stream.next().ok_or("Missing struct")??;
    let proficiencies_and_language: ProficienciesAndLanguage =
        serde_json::from_value(proficiencies_and_language_json)?;
    println!(
        "Parsed ProficienciesAndLanguage: {:?}",
        proficiencies_and_language
    );

    let health_json = stream.next().ok_or("Missing header")??;
    let health: Health = serde_json::from_value(health_json)?;
    println!("Parsed Health: {:?}", health);

    let background_json = stream.next().ok_or("Missing header")??;
    let background: Background = serde_json::from_value(background_json)?;
    println!("Parsed Background: {:?}", background);

    let meta_json = stream.next().ok_or("Missing metadata")??;
    let version: u32 = serde_json::from_value(meta_json)?;
    println!("Parsed standalone integer: {:?}", version);

    // 3. Pull out the final vector
    let list_json = stream.next().ok_or("Missing Vec<Chracter>")??;
    let characters: Vec<ClassLevel> = serde_json::from_value(list_json)?;
    println!("Parsed Character: {:?}", characters);
    let classes = Character {
        classes: characters,
    };

    Ok(CharSheet {
        background,
        classes,
        information,
        proficiencies_and_language,
        statistics,
        skills,
        saving_throws,
        health,
    })
}

// example of reading static data then a Vec at the end
/*
 *
 *
 *
 *

#[derive(Deserialize, Debug)]
struct Header {
    version: u32,
    source: String,
}

#[derive(Deserialize, Debug)]
struct Item {
    id: u32,
    value: String,
}

fn parse_stream<R: Read>(reader: R) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a stateful stream deserializer from your reader
    let mut stream = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // 2. Pull out the individual, non-vector structs sequentially
    let header_json = stream.next().ok_or("Missing header")??;
    let header: Header = serde_json::from_value(header_json)?;
    println!("Parsed Header: {:?}", header);

    let meta_json = stream.next().ok_or("Missing metadata")??;
    let version: u32 = serde_json::from_value(meta_json)?;
    println!("Parsed standalone integer: {:?}", version);

    // 3. Pull out the final vector
    let list_json = stream.next().ok_or("Missing final vector")??;
    let items: Vec<Item> = serde_json::from_value(list_json)?;
    println!("Parsed Vector: {:?}", items);

    Ok(())
}

 *
 *
 *
 *
 *
 */
