use crate::domain::perk::PerkName;
use anyhow::anyhow;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwitterProfile {
    pub perk: PerkName,
    pub name: AggregateName,
    pub id: u64,
}

impl TwitterProfile {
    pub fn new(twitter_id: u64) -> anyhow::Result<Self> {
        Ok(match twitter_id {
            1838892491828785152 => Self {
                id: 1838892491828785152,
                name: AggregateName::Everlyn,
                perk: PerkName::Everlyn,
            },
            1880853313488609280 => Self {
                id: 1880853313488609280,
                name: AggregateName::PerceptronNTWK,
                perk: PerkName::PerceptronNTWK,
            },
            814974533769662464 => Self {
                id: 814974533769662464,
                name: AggregateName::MRRydon,
                perk: PerkName::MRRydon,
            },
            1903384295001010176 => Self {
                id: 1903384295001010176,
                name: AggregateName::Peter_thoc,
                perk: PerkName::Peter_thoc,
            },
            1766124448778784768 => Self {
                id: 1766124448778784768,
                name: AggregateName::Twitter,
                perk: PerkName::Twitter,
            },
            1778711300127821824 => Self {
                id: 1778711300127821824,
                name: AggregateName::FounderTwitter,
                perk: PerkName::FounderTwitter,
            },
            1851306491732709376 => Self {
                id: 1851306491732709376,
                name: AggregateName::XenoTwitter,
                perk: PerkName::XenoTwitter,
            },
            1434571586829357057 => Self {
                id: 1434571586829357057,
                name: AggregateName::WootzAppTwitter,
                perk: PerkName::WootzTwitter,
            },
            1902284045574402049 => Self {
                id: 1902284045574402049,
                name: AggregateName::UFBots,
                perk: PerkName::UFBots,
            },
            1493135152024678401 => Self {
                id: 1493135152024678401,
                name: AggregateName::FrodoBots,
                perk: PerkName::FrodoBots,
            },
            1853818882332434432 => Self {
                id: 1853818882332434432,
                name: AggregateName::SamIsMoving,
                perk: PerkName::SamIsMoving,
            },
            1861258248483151874 => Self {
                id: 1861258248483151874,
                name: AggregateName::BitRobotNetwork,
                perk: PerkName::BitRobotNetwork,
            },
            1861269701537734658 => Self {
                id: 1861269701537734658,
                name: AggregateName::RobotsDotFun,
                perk: PerkName::RobotsDotFun,
            },
            _ => return Err(anyhow!("Unknown")),
        })
    }
}
