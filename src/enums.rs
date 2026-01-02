use crate::{
    ALL_KEYMODE_DMA, ALL_KEYMODE_NODUP, DUPKEY_ACTUAL, DUPKEY_ZERO, ELSET_KEYMODE_DMA, ELSET_KEYMODE_NODUP,
    ELTTYPE_EXTEPH, ELTTYPE_SPVEC_B1P, ELTTYPE_TLE_SGP, ELTTYPE_TLE_SGP4, ELTTYPE_TLE_SP, ELTTYPE_TLE_XP, ELTTYPE_VCM,
    GetSetString, IDX_ORDER_ASC, IDX_ORDER_DES, IDX_ORDER_QUICK, IDX_ORDER_READ,
};

use crate::environment::{XF_FKMOD_4, XF_FKMOD_5, XF_GEOMOD_EGM08, XF_GEOMOD_EGM96, XF_GEOMOD_WGS72, XF_GEOMOD_WGS84};
use crate::obs::{EQUINOX_B1950, EQUINOX_J2K, EQUINOX_OBSTIME, EQUINOX_OBSYEAR};
use crate::sgp4::{SGP4_EPHEM_ECI, SGP4_EPHEM_J2K};
use crate::tle::{TLETYPE_SGP, TLETYPE_SGP4, TLETYPE_SP, TLETYPE_XP};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMode {
    NoDuplicates = ALL_KEYMODE_NODUP,
    DirectMemoryAccess = ALL_KEYMODE_DMA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateKeyMode {
    ReturnZero = DUPKEY_ZERO,
    ReturnKey = DUPKEY_ACTUAL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElsetKeyMode {
    NoDuplicates = ELSET_KEYMODE_NODUP,
    DirectMemoryAccess = ELSET_KEYMODE_DMA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    TwoLineSGP = ELTTYPE_TLE_SGP,
    TwoLineSGP4 = ELTTYPE_TLE_SGP4,
    TwoLineSP = ELTTYPE_TLE_SP,
    SPVector = ELTTYPE_SPVEC_B1P,
    VCM = ELTTYPE_VCM,
    Ephemeris = ELTTYPE_EXTEPH,
    TwoLineXP = ELTTYPE_TLE_XP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FundamentalCatalog {
    Four = XF_FKMOD_4,
    Five = XF_FKMOD_5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeopotentialModel {
    WGS72 = XF_GEOMOD_WGS72,
    WGS84 = XF_GEOMOD_WGS84,
    EGM96 = XF_GEOMOD_EGM96,
    EGM08 = XF_GEOMOD_EGM08,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyOrder {
    Ascending = IDX_ORDER_ASC,
    Descending = IDX_ORDER_DES,
    Fastest = IDX_ORDER_QUICK,
    LoadTime = IDX_ORDER_READ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TLEType {
    SGP = TLETYPE_SGP,
    SGP4 = TLETYPE_SGP4,
    SP = TLETYPE_SP,
    XP = TLETYPE_XP,
}

impl From<f64> for TLEType {
    fn from(value: f64) -> Self {
        match value as isize {
            TLETYPE_SGP => TLEType::SGP,
            TLETYPE_SGP4 => TLEType::SGP4,
            TLETYPE_SP => TLEType::SP,
            TLETYPE_XP => TLEType::XP,
            _ => TLEType::SGP4,
        }
    }
}

impl From<TLEType> for f64 {
    fn from(value: TLEType) -> Self {
        match value {
            TLEType::SGP => TLETYPE_SGP as f64,
            TLEType::SGP4 => TLETYPE_SGP4 as f64,
            TLEType::SP => TLETYPE_SP as f64,
            TLEType::XP => TLETYPE_XP as f64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Classification {
    Unclassified = b'U',
    Confidential = b'C',
    Secret = b'S',
}

impl From<&str> for Classification {
    fn from(s: &str) -> Self {
        match s {
            "U" => Classification::Unclassified,
            "C" => Classification::Confidential,
            "S" => Classification::Secret,
            _ => Classification::Unclassified,
        }
    }
}

impl From<GetSetString> for Classification {
    fn from(value: GetSetString) -> Self {
        match value.value().trim() {
            "U" => Classification::Unclassified,
            "C" => Classification::Confidential,
            "S" => Classification::Secret,
            _ => Classification::Unclassified,
        }
    }
}

impl From<i8> for Classification {
    fn from(value: i8) -> Self {
        match value as u8 {
            b'U' => Classification::Unclassified,
            b'C' => Classification::Confidential,
            b'S' => Classification::Secret,
            _ => Classification::Unclassified,
        }
    }
}

impl From<Classification> for i8 {
    fn from(classification: Classification) -> Self {
        classification as i8
    }
}

impl From<&Classification> for &str {
    fn from(classification: &Classification) -> Self {
        match classification {
            Classification::Unclassified => "U",
            Classification::Confidential => "C",
            Classification::Secret => "S",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SGP4OutputEphemerisFrame {
    TEME = SGP4_EPHEM_ECI,
    J2000 = SGP4_EPHEM_J2K,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UVWType {
    Inertial = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum B3Type {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Eight = 8,
    Nine = 9,
}

impl From<&str> for B3Type {
    fn from(s: &str) -> Self {
        match s {
            "5" => B3Type::Five,
            "9" => B3Type::Nine,
            _ => B3Type::Five,
        }
    }
}

impl From<GetSetString> for B3Type {
    fn from(value: GetSetString) -> Self {
        match value.value().trim() {
            "5" => B3Type::Five,
            "9" => B3Type::Nine,
            _ => B3Type::Five,
        }
    }
}

impl From<B3Type> for i8 {
    fn from(b3_type: B3Type) -> Self {
        match b3_type {
            B3Type::Zero => b'0' as i8,
            B3Type::One => b'1' as i8,
            B3Type::Two => b'2' as i8,
            B3Type::Three => b'3' as i8,
            B3Type::Four => b'4' as i8,
            B3Type::Five => b'5' as i8,
            B3Type::Six => b'6' as i8,
            B3Type::Eight => b'8' as i8,
            B3Type::Nine => b'9' as i8,
        }
    }
}

impl From<i8> for B3Type {
    fn from(value: i8) -> Self {
        match value {
            5 => B3Type::Five,
            9 => B3Type::Nine,
            _ => B3Type::Five,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeanEquinox {
    Date = EQUINOX_OBSTIME,
    Year = EQUINOX_OBSYEAR,
    B1950 = EQUINOX_B1950,
    J2000 = EQUINOX_J2K,
}

impl From<MeanEquinox> for i32 {
    fn from(value: MeanEquinox) -> Self {
        match value {
            MeanEquinox::Date => EQUINOX_OBSTIME as i32,
            MeanEquinox::Year => EQUINOX_OBSYEAR as i32,
            MeanEquinox::B1950 => EQUINOX_B1950 as i32,
            MeanEquinox::J2000 => EQUINOX_J2K as i32,
        }
    }
}

impl From<f64> for MeanEquinox {
    fn from(value: f64) -> Self {
        match value as isize {
            EQUINOX_OBSTIME => MeanEquinox::Date,
            EQUINOX_OBSYEAR => MeanEquinox::Year,
            EQUINOX_B1950 => MeanEquinox::B1950,
            EQUINOX_J2K => MeanEquinox::J2000,
            _ => MeanEquinox::J2000,
        }
    }
}

impl From<MeanEquinox> for f64 {
    fn from(value: MeanEquinox) -> Self {
        match value {
            MeanEquinox::Date => EQUINOX_OBSTIME as f64,
            MeanEquinox::Year => EQUINOX_OBSYEAR as f64,
            MeanEquinox::B1950 => EQUINOX_B1950 as f64,
            MeanEquinox::J2000 => EQUINOX_J2K as f64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionInTrack {
    Beginning = 3,
    Middle = 4,
    End = 5,
}

impl From<i32> for PositionInTrack {
    fn from(value: i32) -> Self {
        match value {
            3 => PositionInTrack::Beginning,
            4 => PositionInTrack::Middle,
            5 => PositionInTrack::End,
            _ => PositionInTrack::Middle,
        }
    }
}

impl From<PositionInTrack> for i32 {
    fn from(value: PositionInTrack) -> Self {
        match value {
            PositionInTrack::Beginning => 3,
            PositionInTrack::Middle => 4,
            PositionInTrack::End => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssociationStatus {
    Statistical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
    None = 4,
}

impl From<i32> for AssociationStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => AssociationStatus::Statistical,
            1 => AssociationStatus::High,
            2 => AssociationStatus::Medium,
            3 => AssociationStatus::Low,
            4 => AssociationStatus::None,
            _ => AssociationStatus::None,
        }
    }
}

impl From<AssociationStatus> for i32 {
    fn from(value: AssociationStatus) -> Self {
        match value {
            AssociationStatus::Statistical => 0,
            AssociationStatus::High => 1,
            AssociationStatus::Medium => 2,
            AssociationStatus::Low => 3,
            AssociationStatus::None => 4,
        }
    }
}
