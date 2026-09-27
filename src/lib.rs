use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassLevel {
    Barbarian {
        subclass: Option<BarbarianSubclass>,
        level: u8,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarbarianSubclass {
    Berserker,
    Totem,
    AncestralGuardian,
    StormHerald,
    Zealot,
    Beast,
    WildMagic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SorcererSubclass {
    DraconicSorcery,
    DivineSoul,
    ShadowMagic,
    StormSorcery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
