use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryInto;
use std::fmt::Formatter;
use std::io::{self, Read};
use std::mem;
use std::str;

use crate::bossdata::{EliteSpec, Profession};

#[repr(C)]
#[derive(Debug)]
pub struct EvtcAgent {
    addr: u64,
    prof: u32,
    is_elite: u32,
    toughness: u16,
    concentration: u16,
    healing: u16,
    hitbox_width: u16,
    condition: u16,
    hitbox_height: u16,
    name: [u8; 64],
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct EvtcSkill {
    id: i32,
    name: [u8; 64],
}

#[repr(C, packed)]
#[derive(Debug)]
struct RawHeader {
    evtc_magic: [u8; 4],
    version: [u8; 8],
    revision: u8,
    boss_id: u16,
    _unused: u8,
}

#[derive(Debug)]
pub struct Header {
    pub version: String,
    pub revision: u8,
    pub boss_id: u16,
}

fn read_header(file: &mut impl Read) -> io::Result<Header> {
    let mut header: RawHeader = unsafe { mem::zeroed() };
    let buf: &mut [u8; mem::size_of::<RawHeader>()] = unsafe { std::mem::transmute(&mut header) };
    file.read_exact(buf)?;
    if header.evtc_magic != *b"EVTC" {
        return Err(io::Error::other("Invalid magic number"));
    }

    let version = str::from_utf8(header.version.as_ref()).unwrap_or("");
    let revision = header.revision;
    let boss_id = header.boss_id;

    Ok(Header {
        version: version.to_string(),
        revision,
        boss_id,
    })
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub addr: u64,
    pub prof: Profession,
    pub elite_spec: EliteSpec,
    pub character_name: String,
    pub account_name: String,
    pub subgroup: String,
}

impl TryFrom<EvtcAgent> for Agent {
    type Error = anyhow::Error;

    fn try_from(raw: EvtcAgent) -> Result<Self, Self::Error> {
        if raw.is_elite != 0xFFFFFFFF {
            let mut it = raw.name.iter();
            let character_name: String =
                String::from_utf8(it.by_ref().take_while(|&&c| c != 0).cloned().collect())?;
            let account_name: String =
                String::from_utf8(it.by_ref().take_while(|&&c| c != 0).cloned().collect())?
                    .trim_start_matches(':')
                    .to_string();
            let subgroup: String =
                String::from_utf8(it.by_ref().take_while(|&&c| c != 0).cloned().collect())?;
            Ok(Self {
                addr: raw.addr,
                prof: Profession::from_evtc(raw.prof),
                elite_spec: EliteSpec::from_evtc(raw.is_elite),
                character_name,
                account_name,
                subgroup,
            })
        } else {
            anyhow::bail!("Not a player agent");
        }
    }
}
// we only care about players
fn read_agents(file: &mut impl Read, count: u32) -> io::Result<Vec<Agent>> {
    let mut agents = Vec::new();
    for _ in 0..count {
        let mut agent: EvtcAgent = unsafe { mem::zeroed() };
        let agent_bytes: &mut [u8; mem::size_of::<EvtcAgent>()] =
            unsafe { mem::transmute(&mut agent) };
        file.read_exact(agent_bytes)?;

        if agent.is_elite != 0xFFFFFFFF {
            if let Ok(a) = agent.try_into() {
                agents.push(a);
            }
        }
    }
    Ok(agents)
}

fn read_skills(file: &mut impl Read, count: u32) -> io::Result<Vec<EvtcSkill>> {
    let mut skill_bytes = vec![0; count as usize * mem::size_of::<EvtcSkill>()];
    file.read_exact(&mut skill_bytes)?;
    skill_bytes.shrink_to_fit();
    let skills: Vec<EvtcSkill> = unsafe {
        Vec::from_raw_parts(
            skill_bytes.as_ptr() as *mut EvtcSkill,
            skill_bytes.len() / mem::size_of::<EvtcSkill>(),
            skill_bytes.capacity() / mem::size_of::<EvtcSkill>(),
        )
    };
    // leak the vec to prevent it from being dropped
    // because we are basically transferring ownership
    // to skills
    skill_bytes.leak();
    Ok(skills)
}

fn read_log(file: &mut impl Read) -> io::Result<Vec<CbtEvent>> {
    let mut combat_log = vec![];
    file.read_to_end(&mut combat_log)?;
    if combat_log.len() % mem::size_of::<CbtEvent>() != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Combat log length is not a multiple of CbtEvent",
        ));
    }
    // make sure capacity is a multiple of CbtEvent
    combat_log.shrink_to_fit();
    let cbtlog: Vec<CbtEvent> = unsafe {
        Vec::from_raw_parts(
            combat_log.as_ptr() as *mut CbtEvent,
            combat_log.len() / mem::size_of::<CbtEvent>(),
            combat_log.capacity() / mem::size_of::<CbtEvent>(),
        )
    };
    // leak the vec to prevent it from being dropped
    // because we are basically transferring ownership
    // to cbtlog
    combat_log.leak();
    Ok(cbtlog)
}

pub struct Encounter {
    pub header: Header,
    pub agents: Vec<Agent>,
    pub skills: Vec<EvtcSkill>,
    pub combat_log: Vec<CbtEvent>,
    pub pov: Option<Agent>,
}

impl Encounter {
    /// Deletes all cbtlog and skills
    pub fn shrink(&mut self) {
        self.combat_log.clear();
        self.skills.clear();
    }
}

impl std::fmt::Debug for Encounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Encounter {{ header: {:?}, agents: {:?}, skills: Vec({}), combat_log: Vec({}), pov: {:?} }}",
            self.header,
            self.agents,
            self.skills.len(),
            self.combat_log.len(),
            self.pov
        )
    }
}

pub fn read_encounter(rdr: &mut impl Read) -> io::Result<Encounter> {
    // Read header
    let header = read_header(rdr)?;

    // Read agent count
    let agent_count = rdr.read_u32::<LittleEndian>()?;

    // Read agent data
    let agents = read_agents(rdr, agent_count)?;

    // Read skill count
    let skill_count = rdr.read_u32::<LittleEndian>()?;

    // Read skill data
    let skills = read_skills(rdr, skill_count)?;

    // Read combat log
    let combat_log = read_log(rdr)?;

    // Find pov
    let pov = find_pov(combat_log.as_slice(), agents.as_slice());

    Ok(Encounter {
        header,
        agents,
        skills,
        combat_log,
        pov,
    })
}

fn find_pov(evts: &[CbtEvent], agents: &[Agent]) -> Option<Agent> {
    for evt in evts {
        if evt.is_statechange == CbtStateChange::PointOfView as u32 as u8 {
            return agents.iter().find(|a| a.addr == evt.src_agent).cloned();
        }
    }
    None
}
#[repr(u32)] // ensures the enum is represented as a 32-bit unsigned integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CbtStateChange {
    /// Not used - not this kind of event
    None = 0,
    /// Agent entered combat
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: subgroup
    /// - `value`: profession ID
    /// - `buff_dmg`: elite specialization ID
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: limited to squad
    EnterCombat,
    /// Agent left combat
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: limited to squad
    ExitCombat,
    /// Agent is alive at time of event
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    ChangeUp,
    /// Agent is dead at time of event
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    ChangeDead,
    /// Agent is down at time of event
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    ChangeDown,
    /// Agent entered tracking
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Spawn,
    /// Agent left tracking
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Despawn,
    /// Agent health percentage changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: percentage * 10000, e.g. 99.5% will be 9950
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    HealthPctUpdate,
    /// Squad combat start, first player enters combat (previously named log start)
    ///
    /// - `value`: as `u32`, server UNIX timestamp
    /// - `buff_dmg`: local UNIX timestamp
    /// - `evtc`: yes
    /// - `realtime`: yes
    SqCombatStart,
    /// Squad combat stop, last player left combat (previously named log end)
    ///
    /// - `value`: as `u32`, server UNIX timestamp
    /// - `buff_dmg`: local UNIX timestamp
    /// - `evtc`: yes
    /// - `realtime`: yes
    LogEnd,
    /// Agent weapon set changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: new weapon set ID
    /// - `value`: old weapon set ID
    /// - `evtc`: yes
    /// - `realtime`: yes
    WeapSwap,
    /// Agent maximum health changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: new max health
    /// - `evtc`: limited to non-players
    /// - `realtime`: no
    MaxHealthUpdate,
    /// "Recording" player
    ///
    /// - `src_agent`: relates to agent
    /// - `evtc`: yes
    /// - `realtime`: no
    PointOfView,
    /// Text language ID
    ///
    /// - `src_agent`: text language ID
    /// - `evtc`: yes
    /// - `realtime`: no
    Language,
    /// Game build
    ///
    /// - `src_agent`: game build number
    /// - `evtc`: yes
    /// - `realtime`: no
    GwBuild,
    /// Server shard ID
    ///
    /// - `src_agent`: shard ID
    /// - `evtc`: yes
    /// - `realtime`: no
    ShardId,
    /// Wiggly box reward
    ///
    /// - `dst_agent`: reward ID
    /// - `value`: reward type
    /// - `evtc`: yes
    /// - `realtime`: yes
    Reward,
    /// Buff application for buffs already existing at time of event
    ///
    /// - Identical to buff application in the `cbtevent` struct
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: limited to squad
    BuffInitial,
    /// Agent position changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: `(float*)&dst_agent` is `float[3]`, x/y/z
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Position,
    /// Agent velocity changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: `(float*)&dst_agent` is `float[3]`, x/y/z
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Velocity,
    /// Agent facing direction changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: `(float*)&dst_agent` is `float[2]`, x/y
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Facing,
    /// Agent team ID changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: new team ID
    /// - `value`: old team ID
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    TeamChange,
    /// Attack target to gadget association
    ///
    /// - `src_agent`: relates to agent, the attack target
    /// - `dst_agent`: the gadget
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    AttackTarget,
    /// Agent targetable state
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: new targetable state
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: no
    Targetable,
    /// Map ID
    ///
    /// - `src_agent`: map ID
    /// - `evtc`: yes
    /// - `realtime`: no
    MapId,
    /// Internal use
    ReplInfo,
    /// Buff instance is now active
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: buff instance ID
    /// - `value`: current buff duration
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: limited to squad
    StackActive,
    /// Buff instance duration changed, value is the duration to reset to
    ///
    /// - Also marks inactive, `pad61-pad64`: buff instance ID
    /// - `src_agent`: relates to agent
    /// - `value`: new duration
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: limited to squad
    StackReset,
    /// Agent is a member of guild
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: `(uint8_t*)&dst_agent` is `uint8_t[16]`, GUID of guild
    /// - `evtc`: limited to squad outside instances
    /// - `realtime`: no
    Guild,
    /// Buff information
    ///
    /// - `skillid`: skilldef ID of buff
    /// - `overstack_value`: max combined duration
    /// - `src_master_instid`, `is_src_flanking`, `is_shields`, `is_offcycle`, etc.: detailed buff properties
    /// - `evtc`: yes
    /// - `realtime`: no
    BuffInfo,
    /// Buff formula
    ///
    /// - `skillid`: skilldef ID of buff
    /// - `time`: `(float*)&time` is `float[9]`, type attribute1 attribute2 parameter1 parameter2 parameter3 trait_condition_source trait_condition_self content_reference
    /// - `src_instid`: `(float*)&src_instid` is `float[2]`, buff_condition_source buff_condition_self
    /// - `evtc`: yes
    /// - `realtime`: no
    BuffFormula,
    /// Skill information
    ///
    /// - `skillid`: skilldef ID of skill
    /// - `time`: `(float*)&time` is `float[4]`, cost, range0, range1, tooltiptime
    /// - `evtc`: yes
    /// - `realtime`: no
    SkillInfo,
    /// Skill timing
    ///
    /// - `skillid`: skilldef ID of skill
    /// - `src_agent`: timing type
    /// - `dst_agent`: at time since activation in milliseconds
    /// - `evtc`: yes
    /// - `realtime`: no
    SkillTiming,
    /// Agent breakbar state changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: new breakbar state
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    BreakbarState,
    /// Agent breakbar percentage changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: breakbar percent * 10000
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    BreakbarPercent,
    /// One event per message, previously named error
    ///
    /// - Used for various integrity checks
    Integrity,
    /// One event per marker on an agent
    ///
    /// - Used for squad markers, icons, etc.
    Marker,
    /// Agent barrier percentage changed
    ///
    /// - `src_agent`: relates to agent
    /// - `dst_agent`: barrier percent * 10000
    /// - `evtc`: limited to agent table outside instances
    /// - `realtime`: limited to squad
    BarrierPctUpdate,
    /// ArcDPS stats reset
    StatReset,
    /// For extension use
    Extension,
    /// Event deemed unsafe for realtime
    ApiDelayed,
    /// Map instance start
    InstanceStart,
    /// Tick health, previously named tickrate
    RateHealth,
    /// Retired event
    Last90BeforeDown,
    /// Retired event
    Effect,
    /// Content ID to GUID association
    IdToGuid,
    /// Log boss agent changed
    LogNpcUpdate,
    /// Internal use
    IdleEvent,
    /// For extension use
    ExtensionCombat,
    /// Fractal scale for fractals
    FractalScale,
    /// Play graphical effect
    Effect2,
    /// Ruleset for self
    Ruleset,
    /// Squad ground markers
    SquadMarker,
    /// Arc build info
    ArcBuild,
    /// Glider status change
    Glider,
    /// Disable stopped early
    StunBreak,
    /// Unknown/unsupported type newer than this list
    Unknown,
}
/// Represents a combat event.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CbtEvent {
    /// Time of event, retrieved using `timegettime()`.
    pub time: u64,
    /// Source agent ID.
    pub src_agent: u64,
    /// Destination agent ID.
    pub dst_agent: u64,
    /// Event-specific value.
    pub value: i32,
    /// Buff damage, if applicable.
    pub buff_dmg: i32,
    /// Overstack value for buffs.
    pub overstack_value: u32,
    /// Skill ID related to the event.
    pub skillid: u32,
    /// Source agent's instance ID.
    pub src_instid: u16,
    /// Destination agent's instance ID.
    pub dst_instid: u16,
    /// Source agent's master instance ID (for pets, etc.).
    pub src_master_instid: u16,
    /// Destination agent's master instance ID.
    pub dst_master_instid: u16,
    /// Indicator for whether the source and destination agents are friend or foe.
    pub iff: u8,
    /// Whether this event is related to a buff.
    pub buff: u8,
    /// The result of the event.
    pub result: u8,
    /// Whether this event represents an activation of a skill.
    pub is_activation: u8,
    /// Whether this event represents the removal of a buff.
    pub is_buffremove: u8,
    /// Whether the source agent was at 90% or more health.
    pub is_ninety: u8,
    /// Whether the source agent was at 50% or less health.
    pub is_fifty: u8,
    /// Whether the source agent was moving at the time of the event.
    pub is_moving: u8,
    /// Whether this event represents a state change (such as entering combat).
    pub is_statechange: u8,
    /// Whether the source agent was flanking the target.
    pub is_flanking: u8,
    /// Whether shields were involved in this event.
    pub is_shields: u8,
    /// Whether the event occurred off the main skill cycle.
    pub is_offcycle: u8,
    /// Padding bytes for alignment (not used).
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}
