// Copied from Daniel @dunj3
// https://gitlab.com/dunj3/evtclib
// modified by me (belst)
// Licensed under the MIT license

//! This module contains some low-level game data, such as different boss IDs.
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::{self, Display, Formatter};

/// The different rulesets, affecting skill & trait balancing.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ruleset {
    /// Player-versus-Environment.
    ///
    /// Active in open world, raids, strikes, fractals, ...
    PvE,
    /// World-versus-World.
    WvW,
    /// (Structured) Player-versus-Player.
    PvP,
}

/// The game mode in which a log was produced.
///
/// Note that the distinction made here is relatively arbitrary, but hopefully still useful. In
/// Guild Wars 2 terms, there is no clear definition of what a "game mode" is.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameMode {
    /// The log is from a raid encounter.
    Raid,
    /// The log is from a fractal fight.
    Fractal,
    /// The log is from a strike mission.
    Strike,
    /// The log is from a training golem.
    Golem,
    /// The log is from a world-versus-world fight.
    WvW,
}

/// Enum containing all boss IDs.
///
/// For a high-level event categorization, take a look at the [`Encounter`] enum. The IDs listed
/// here are for a more fine-grained control, e.g. if you specifically need to differentiate
/// between Nikare and Kenut in the Twin Largos encounter.
///
/// This enum is non-exhaustive to ensure that future bosses can be added without
/// inducing a breaking change.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
#[non_exhaustive]
#[repr(u16)]
pub enum BossId {
    ValeGuardian = 15438,
    Gorseval = 15429,
    Sabetha = 15375,
    Slothasor = 16123,
    Matthias = 16115,
    KeepConstruct = 16235,
    Xera = 16246,
    Cairn = 17194,
    Mo = 17172,
    Samarog = 17188,
    Deimos = 17154,
    SoullessHorror = 19767,
    Dhuum = 19450,
    ConjuredAmalgamate = 43974,
    Nikare = 21105,
    Kenut = 21089,
    Qadim = 20934,
    Adina = 22006,
    Sabir = 21964,
    QadimThePeerless = 22000,
    // Raid "Events"
    Berg = 16088,
    Zane = 16137,
    Nurella = 16125,
    McLeod = 16253,
    TwistedCastle = 16247,
    River = 19828,
    BrokenKing = 19691,
    SoulEater = 19536,
    EyeOfJudgement = 19651,
    EyeOfFate = 19844,
    // Fractal CMs
    // Only CMs
    // Nightmare
    Mama = 17021,
    Siax = 17028,
    Ensolyss = 16948,
    // Shattered Observatory
    Skorvald = 17632,
    Artsariiv = 17949,
    Arkk = 17759,
    // Sunqua Peak
    SorrowfulSpellcaster = 23254,
    // Silent Surf
    Kanaxai = 25577,
    // Lonely Tower
    CerusLonelyTower = 26257,
    DeimosLonelyTower = 26226,
    EparchLonelyTower = 26231,
    // IBS Strikes
    Icebrood = 22154,
    TheVoice = 22343,
    TheClaw = 22481,
    Fraenir = 22492,
    FraenirConstruct = 22436,
    Boneskinner = 22521,
    WhisperOfJormag = 22711,
    VariniaStormsounder = 22836,
    // EOD Strikes
    CaptainMaiTrin = 24033,
    CaptainMaiTrin2 = 24768,
    CaptainMaiTrin3 = 25247,
    Ankka = 23957,
    MinisterLi = 24485,
    MinisterLiCm = 24266,
    DragonVoid1 = 24375,
    DragonVoid2 = 1378,
    DragonVoid3 = 43488,
    // Base Strikes
    // Old Lion's Court
    PrototypeVermilion = 25413,
    PrototypeIndigo = 25419,
    PrototypeArsenite = 25415,
    PrototypeVermilionCm = 25414,
    PrototypeArseniteCm = 25416,
    PrototypeIndigoCm = 25423,
    // Holiday Missions
    Freezie = 21333,
    // Soto Strikes
    Dagda = 25705,
    Cerus = 25989,
    // Golems
    StandardGolem = 16199,
    MediumGolem = 19645,
    LargeGolem = 19676,
    MassiveGolem = 16202,
    AverageGolem = 16177,
    VitalGolem = 16198,
    // WVW
    Wvw = 1,
    // Misc
    Instance = 2,
    Unknown = 0,
}
impl BossId {
    pub fn from_header_id(id: u16) -> Self {
        Self::from_u16(id).unwrap_or(Self::Unknown)
    }
}

impl Display for BossId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use BossId as BI;
        let name = match *self {
            // Raids
            BI::ValeGuardian => "Vale Guardian",
            BI::Gorseval => "Gorseval",
            BI::Sabetha => "Sabetha",
            BI::Slothasor => "Slothasor",
            BI::Matthias => "Matthias",
            BI::KeepConstruct => "Keep Construct",
            BI::Xera => "Xera",
            BI::Cairn => "Cairn",
            BI::Mo => "Mursaat Overseer",
            BI::Samarog => "Samarog",
            BI::Deimos => "Deimos",
            BI::SoullessHorror => "Soulless Horror",
            BI::Dhuum => "Dhuum",
            BI::ConjuredAmalgamate => "Conjured Amalgamate",
            BI::Nikare | BI::Kenut => "Twin Largos",
            BI::Qadim => "Qadim",
            BI::Adina => "Cardinal Adina",
            BI::Sabir => "Cardinal Sabir",
            BI::QadimThePeerless => "Qadim The Peerless",
            BI::Berg | BI::Zane | BI::Nurella => "Bandit Trio",
            BI::McLeod => "Escort",
            BI::TwistedCastle => "Twisted Castle",
            BI::River => "River of Souls",
            BI::BrokenKing => "Broken King",
            BI::SoulEater => "Soul Eater",
            BI::EyeOfJudgement | BI::EyeOfFate => "Eyes",
            // Fractals
            BI::Mama => "Mama",
            BI::Siax => "Siax",
            BI::Ensolyss => "Ensolyss",
            BI::Skorvald => "Skorvald",
            BI::Artsariiv => "Artsariiv",
            BI::Arkk => "Arkk",
            BI::SorrowfulSpellcaster => "Ai",
            BI::Kanaxai => "Kanaxai",
            BI::CerusLonelyTower | BI::DeimosLonelyTower => "Cerus and Deimos",
            BI::EparchLonelyTower => "Eparch",
            // Strikes IBS
            BI::Icebrood => "Icebrood",
            BI::TheVoice | BI::TheClaw => "Voice and Claw",
            BI::Fraenir | BI::FraenirConstruct => "Fraenir of Jormag",
            BI::Boneskinner => "Boneskinner",
            BI::WhisperOfJormag => "Whisper of Jormag",
            BI::VariniaStormsounder => "Varinia Stormsounder",
            // Strikes EoD
            BI::CaptainMaiTrin | BI::CaptainMaiTrin2 | BI::CaptainMaiTrin3 => "Captain Mai Trin",
            BI::Ankka => "Ankka",
            BI::MinisterLi | BI::MinisterLiCm => "Minister Li",
            BI::DragonVoid1 | BI::DragonVoid2 | BI::DragonVoid3 => "Dragon Void",
            BI::PrototypeIndigo
            | BI::PrototypeVermilion
            | BI::PrototypeVermilionCm
            | BI::PrototypeArseniteCm
            | BI::PrototypeArsenite
            | BI::PrototypeIndigoCm => "Old Lion's Court",
            // Strikes soto
            BI::Dagda => "Dagda",
            BI::Cerus => "Cerus",
            // Misc
            BI::Freezie => "Freezie",
            BI::StandardGolem => "Standard Golem",
            BI::MediumGolem => "Medium Golem",
            BI::LargeGolem => "Large Golem",
            BI::MassiveGolem => "Massive Golem",
            BI::AverageGolem => "Average Golem",
            BI::VitalGolem => "Vital Golem",
            BI::Wvw => "WvW",
            BI::Instance => "Instance",
            BI::Unknown => "Unknown",
        };
        write!(f, "{name}")
    }
}

/// An in-game profession.
///
/// This only contains the 9 base professions. For elite specializations, see
/// [`EliteSpec`][EliteSpec].
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum Profession {
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,

    Unknown = 0,
}

impl Display for Profession {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            Profession::Guardian => "Guardian",
            Profession::Warrior => "Warrior",
            Profession::Engineer => "Engineer",
            Profession::Ranger => "Ranger",
            Profession::Thief => "Thief",
            Profession::Elementalist => "Elementalist",
            Profession::Mesmer => "Mesmer",
            Profession::Necromancer => "Necromancer",
            Profession::Revenant => "Revenant",
            Profession::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

impl Profession {
    pub fn from_evtc(id: u32) -> Self {
        if let Some(prof) = Self::from_u32(id) {
            prof
        } else {
            Self::Unknown
        }
    }
}

/// All possible elite specializations.
///
/// Note that the numeric value of the enum variants correspond to the specialization ID in the API
/// as well. See [the official wiki](https://wiki.guildwars2.com/wiki/API:2/specializations) for
/// more information regarding the API usage.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum EliteSpec {
    // Heart of Thorns elites:
    Dragonhunter = 27,
    Berserker = 18,
    Scrapper = 43,
    Druid = 5,
    Daredevil = 7,
    Tempest = 48,
    Chronomancer = 40,
    Reaper = 34,
    Herald = 52,

    // Path of Fire elites:
    Firebrand = 62,
    Spellbreaker = 61,
    Holosmith = 57,
    Soulbeast = 55,
    Deadeye = 58,
    Weaver = 56,
    Mirage = 59,
    Scourge = 60,
    Renegade = 63,

    // End of Dragons elites:
    Willbender = 65,
    Bladesworn = 68,
    Untamed = 72,
    Specter = 71,
    Catalyst = 67,
    Virtuoso = 66,
    Harbinger = 64,
    Vindicator = 69,
    Mechanist = 70,

    // Misc
    Unknown = 0,
}

impl Display for EliteSpec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match *self {
            EliteSpec::Dragonhunter => "Dragonhunter",
            EliteSpec::Berserker => "Berserker",
            EliteSpec::Scrapper => "Scrapper",
            EliteSpec::Druid => "Druid",
            EliteSpec::Daredevil => "Daredevil",
            EliteSpec::Tempest => "Tempest",
            EliteSpec::Chronomancer => "Chronomancer",
            EliteSpec::Reaper => "Reaper",
            EliteSpec::Herald => "Herald",
            EliteSpec::Firebrand => "Firebrand",
            EliteSpec::Spellbreaker => "Spellbreaker",
            EliteSpec::Holosmith => "Holosmith",
            EliteSpec::Soulbeast => "Soulbeast",
            EliteSpec::Deadeye => "Deadeye",
            EliteSpec::Weaver => "Weaver",
            EliteSpec::Mirage => "Mirage",
            EliteSpec::Scourge => "Scourge",
            EliteSpec::Renegade => "Renegade",
            EliteSpec::Willbender => "Willbender",
            EliteSpec::Bladesworn => "Bladesworn",
            EliteSpec::Mechanist => "Mechanist",
            EliteSpec::Untamed => "Untamed",
            EliteSpec::Specter => "Specter",
            EliteSpec::Catalyst => "Catalyst",
            EliteSpec::Virtuoso => "Virtuoso",
            EliteSpec::Harbinger => "Harbinger",
            EliteSpec::Vindicator => "Vindicator",
            EliteSpec::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

impl EliteSpec {
    /// Return the profession that this elite specialization belongs to.
    ///
    /// This value is hardcoded (and not expected to change), and does not require a network
    /// connection or API access.
    pub fn profession(self) -> Profession {
        use EliteSpec::*;
        match self {
            Dragonhunter | Firebrand | Willbender => Profession::Guardian,
            Berserker | Spellbreaker | Bladesworn => Profession::Warrior,
            Scrapper | Holosmith | Mechanist => Profession::Engineer,
            Druid | Soulbeast | Untamed => Profession::Ranger,
            Daredevil | Deadeye | Specter => Profession::Thief,
            Tempest | Weaver | Catalyst => Profession::Elementalist,
            Chronomancer | Mirage | Virtuoso => Profession::Mesmer,
            Reaper | Scourge | Harbinger => Profession::Necromancer,
            Herald | Renegade | Vindicator => Profession::Revenant,
            _ => Profession::Unknown,
        }
    }

    pub fn from_evtc(id: u32) -> Self {
        if let Some(spec) = Self::from_u32(id) {
            spec
        } else {
            Self::Unknown
        }
    }
}
