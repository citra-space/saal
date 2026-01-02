use pyo3::prelude::*;
use pyo3::PyTypeInfo;

use crate::enums::{
    AssociationStatus, B3Type, Classification, DuplicateKeyMode, ElsetKeyMode, ElementType,
    FundamentalCatalog, KeyMode, KeyOrder, PositionInTrack, SGP4OutputEphemerisFrame, TLEType,
};

#[pyclass(name = "KeyMode", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyKeyMode {
    NoDuplicates,
    DirectMemoryAccess,
}

impl From<PyKeyMode> for KeyMode {
    fn from(value: PyKeyMode) -> Self {
        match value {
            PyKeyMode::NoDuplicates => KeyMode::NoDuplicates,
            PyKeyMode::DirectMemoryAccess => KeyMode::DirectMemoryAccess,
        }
    }
}

impl From<KeyMode> for PyKeyMode {
    fn from(value: KeyMode) -> Self {
        match value {
            KeyMode::NoDuplicates => PyKeyMode::NoDuplicates,
            KeyMode::DirectMemoryAccess => PyKeyMode::DirectMemoryAccess,
        }
    }
}

#[pymethods]
impl PyKeyMode {
    fn __int__(&self) -> i32 {
        match self {
            PyKeyMode::NoDuplicates => KeyMode::NoDuplicates as i32,
            PyKeyMode::DirectMemoryAccess => KeyMode::DirectMemoryAccess as i32,
        }
    }
}

#[pyclass(name = "KeyOrder", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyKeyOrder {
    Ascending,
    Descending,
    Fastest,
    LoadTime,
}

impl From<PyKeyOrder> for KeyOrder {
    fn from(value: PyKeyOrder) -> Self {
        match value {
            PyKeyOrder::Ascending => KeyOrder::Ascending,
            PyKeyOrder::Descending => KeyOrder::Descending,
            PyKeyOrder::Fastest => KeyOrder::Fastest,
            PyKeyOrder::LoadTime => KeyOrder::LoadTime,
        }
    }
}

impl From<KeyOrder> for PyKeyOrder {
    fn from(value: KeyOrder) -> Self {
        match value {
            KeyOrder::Ascending => PyKeyOrder::Ascending,
            KeyOrder::Descending => PyKeyOrder::Descending,
            KeyOrder::Fastest => PyKeyOrder::Fastest,
            KeyOrder::LoadTime => PyKeyOrder::LoadTime,
        }
    }
}

#[pymethods]
impl PyKeyOrder {
    fn __int__(&self) -> i32 {
        match self {
            PyKeyOrder::Ascending => KeyOrder::Ascending as i32,
            PyKeyOrder::Descending => KeyOrder::Descending as i32,
            PyKeyOrder::Fastest => KeyOrder::Fastest as i32,
            PyKeyOrder::LoadTime => KeyOrder::LoadTime as i32,
        }
    }
}

#[pyclass(name = "DuplicateKeyMode", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyDuplicateKeyMode {
    ReturnZero,
    ReturnKey,
}

impl From<PyDuplicateKeyMode> for DuplicateKeyMode {
    fn from(value: PyDuplicateKeyMode) -> Self {
        match value {
            PyDuplicateKeyMode::ReturnZero => DuplicateKeyMode::ReturnZero,
            PyDuplicateKeyMode::ReturnKey => DuplicateKeyMode::ReturnKey,
        }
    }
}

impl From<DuplicateKeyMode> for PyDuplicateKeyMode {
    fn from(value: DuplicateKeyMode) -> Self {
        match value {
            DuplicateKeyMode::ReturnZero => PyDuplicateKeyMode::ReturnZero,
            DuplicateKeyMode::ReturnKey => PyDuplicateKeyMode::ReturnKey,
        }
    }
}

#[pymethods]
impl PyDuplicateKeyMode {
    fn __int__(&self) -> i32 {
        match self {
            PyDuplicateKeyMode::ReturnZero => DuplicateKeyMode::ReturnZero as i32,
            PyDuplicateKeyMode::ReturnKey => DuplicateKeyMode::ReturnKey as i32,
        }
    }
}

#[pyclass(name = "ElsetKeyMode", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyElsetKeyMode {
    NoDuplicates,
    DirectMemoryAccess,
}

impl From<PyElsetKeyMode> for ElsetKeyMode {
    fn from(value: PyElsetKeyMode) -> Self {
        match value {
            PyElsetKeyMode::NoDuplicates => ElsetKeyMode::NoDuplicates,
            PyElsetKeyMode::DirectMemoryAccess => ElsetKeyMode::DirectMemoryAccess,
        }
    }
}

impl From<ElsetKeyMode> for PyElsetKeyMode {
    fn from(value: ElsetKeyMode) -> Self {
        match value {
            ElsetKeyMode::NoDuplicates => PyElsetKeyMode::NoDuplicates,
            ElsetKeyMode::DirectMemoryAccess => PyElsetKeyMode::DirectMemoryAccess,
        }
    }
}

#[pymethods]
impl PyElsetKeyMode {
    fn __int__(&self) -> i32 {
        match self {
            PyElsetKeyMode::NoDuplicates => ElsetKeyMode::NoDuplicates as i32,
            PyElsetKeyMode::DirectMemoryAccess => ElsetKeyMode::DirectMemoryAccess as i32,
        }
    }
}

#[pyclass(name = "ElementType", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyElementType {
    TwoLineSGP,
    TwoLineSGP4,
    TwoLineSP,
    SPVector,
    VCM,
    Ephemeris,
    TwoLineXP,
}

impl From<PyElementType> for ElementType {
    fn from(value: PyElementType) -> Self {
        match value {
            PyElementType::TwoLineSGP => ElementType::TwoLineSGP,
            PyElementType::TwoLineSGP4 => ElementType::TwoLineSGP4,
            PyElementType::TwoLineSP => ElementType::TwoLineSP,
            PyElementType::SPVector => ElementType::SPVector,
            PyElementType::VCM => ElementType::VCM,
            PyElementType::Ephemeris => ElementType::Ephemeris,
            PyElementType::TwoLineXP => ElementType::TwoLineXP,
        }
    }
}

impl From<ElementType> for PyElementType {
    fn from(value: ElementType) -> Self {
        match value {
            ElementType::TwoLineSGP => PyElementType::TwoLineSGP,
            ElementType::TwoLineSGP4 => PyElementType::TwoLineSGP4,
            ElementType::TwoLineSP => PyElementType::TwoLineSP,
            ElementType::SPVector => PyElementType::SPVector,
            ElementType::VCM => PyElementType::VCM,
            ElementType::Ephemeris => PyElementType::Ephemeris,
            ElementType::TwoLineXP => PyElementType::TwoLineXP,
        }
    }
}

#[pymethods]
impl PyElementType {
    fn __int__(&self) -> i32 {
        match self {
            PyElementType::TwoLineSGP => ElementType::TwoLineSGP as i32,
            PyElementType::TwoLineSGP4 => ElementType::TwoLineSGP4 as i32,
            PyElementType::TwoLineSP => ElementType::TwoLineSP as i32,
            PyElementType::SPVector => ElementType::SPVector as i32,
            PyElementType::VCM => ElementType::VCM as i32,
            PyElementType::Ephemeris => ElementType::Ephemeris as i32,
            PyElementType::TwoLineXP => ElementType::TwoLineXP as i32,
        }
    }
}

#[pyclass(name = "TLEType", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyTleType {
    SGP,
    SGP4,
    SP,
    XP,
}

impl From<PyTleType> for TLEType {
    fn from(value: PyTleType) -> Self {
        match value {
            PyTleType::SGP => TLEType::SGP,
            PyTleType::SGP4 => TLEType::SGP4,
            PyTleType::SP => TLEType::SP,
            PyTleType::XP => TLEType::XP,
        }
    }
}

impl From<TLEType> for PyTleType {
    fn from(value: TLEType) -> Self {
        match value {
            TLEType::SGP => PyTleType::SGP,
            TLEType::SGP4 => PyTleType::SGP4,
            TLEType::SP => PyTleType::SP,
            TLEType::XP => PyTleType::XP,
        }
    }
}

#[pymethods]
impl PyTleType {
    fn __int__(&self) -> i32 {
        match self {
            PyTleType::SGP => TLEType::SGP as i32,
            PyTleType::SGP4 => TLEType::SGP4 as i32,
            PyTleType::SP => TLEType::SP as i32,
            PyTleType::XP => TLEType::XP as i32,
        }
    }
}

#[pyclass(name = "Classification", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyClassification {
    Unclassified,
    Confidential,
    Secret,
}

impl From<PyClassification> for Classification {
    fn from(value: PyClassification) -> Self {
        match value {
            PyClassification::Unclassified => Classification::Unclassified,
            PyClassification::Confidential => Classification::Confidential,
            PyClassification::Secret => Classification::Secret,
        }
    }
}

impl From<Classification> for PyClassification {
    fn from(value: Classification) -> Self {
        match value {
            Classification::Unclassified => PyClassification::Unclassified,
            Classification::Confidential => PyClassification::Confidential,
            Classification::Secret => PyClassification::Secret,
        }
    }
}

#[pymethods]
impl PyClassification {
    fn __int__(&self) -> i32 {
        match self {
            PyClassification::Unclassified => Classification::Unclassified as i32,
            PyClassification::Confidential => Classification::Confidential as i32,
            PyClassification::Secret => Classification::Secret as i32,
        }
    }
}

#[pyclass(name = "B3Type", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyB3Type {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Eight,
    Nine,
}

impl From<PyB3Type> for B3Type {
    fn from(value: PyB3Type) -> Self {
        match value {
            PyB3Type::Zero => B3Type::Zero,
            PyB3Type::One => B3Type::One,
            PyB3Type::Two => B3Type::Two,
            PyB3Type::Three => B3Type::Three,
            PyB3Type::Four => B3Type::Four,
            PyB3Type::Five => B3Type::Five,
            PyB3Type::Six => B3Type::Six,
            PyB3Type::Eight => B3Type::Eight,
            PyB3Type::Nine => B3Type::Nine,
        }
    }
}

impl From<B3Type> for PyB3Type {
    fn from(value: B3Type) -> Self {
        match value {
            B3Type::Zero => PyB3Type::Zero,
            B3Type::One => PyB3Type::One,
            B3Type::Two => PyB3Type::Two,
            B3Type::Three => PyB3Type::Three,
            B3Type::Four => PyB3Type::Four,
            B3Type::Five => PyB3Type::Five,
            B3Type::Six => PyB3Type::Six,
            B3Type::Eight => PyB3Type::Eight,
            B3Type::Nine => PyB3Type::Nine,
        }
    }
}

#[pymethods]
impl PyB3Type {
    fn __int__(&self) -> i32 {
        let value: B3Type = (*self).into();
        i8::from(value) as i32
    }
}

#[pyclass(name = "PositionInTrack", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyPositionInTrack {
    Beginning,
    Middle,
    End,
}

impl From<PyPositionInTrack> for PositionInTrack {
    fn from(value: PyPositionInTrack) -> Self {
        match value {
            PyPositionInTrack::Beginning => PositionInTrack::Beginning,
            PyPositionInTrack::Middle => PositionInTrack::Middle,
            PyPositionInTrack::End => PositionInTrack::End,
        }
    }
}

impl From<PositionInTrack> for PyPositionInTrack {
    fn from(value: PositionInTrack) -> Self {
        match value {
            PositionInTrack::Beginning => PyPositionInTrack::Beginning,
            PositionInTrack::Middle => PyPositionInTrack::Middle,
            PositionInTrack::End => PyPositionInTrack::End,
        }
    }
}

#[pymethods]
impl PyPositionInTrack {
    fn __int__(&self) -> i32 {
        let value: PositionInTrack = (*self).into();
        i32::from(value)
    }
}

#[pyclass(name = "AssociationStatus", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyAssociationStatus {
    Statistical,
    High,
    Medium,
    Low,
    None,
}

impl From<PyAssociationStatus> for AssociationStatus {
    fn from(value: PyAssociationStatus) -> Self {
        match value {
            PyAssociationStatus::Statistical => AssociationStatus::Statistical,
            PyAssociationStatus::High => AssociationStatus::High,
            PyAssociationStatus::Medium => AssociationStatus::Medium,
            PyAssociationStatus::Low => AssociationStatus::Low,
            PyAssociationStatus::None => AssociationStatus::None,
        }
    }
}

impl From<AssociationStatus> for PyAssociationStatus {
    fn from(value: AssociationStatus) -> Self {
        match value {
            AssociationStatus::Statistical => PyAssociationStatus::Statistical,
            AssociationStatus::High => PyAssociationStatus::High,
            AssociationStatus::Medium => PyAssociationStatus::Medium,
            AssociationStatus::Low => PyAssociationStatus::Low,
            AssociationStatus::None => PyAssociationStatus::None,
        }
    }
}

#[pymethods]
impl PyAssociationStatus {
    fn __int__(&self) -> i32 {
        let value: AssociationStatus = (*self).into();
        i32::from(value)
    }
}

pub fn register_enums(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<PyKeyMode>()?;
    parent_module.add_class::<PyKeyOrder>()?;
    parent_module.add_class::<PyDuplicateKeyMode>()?;
    parent_module.add_class::<PyElsetKeyMode>()?;
    parent_module.add_class::<PyElementType>()?;
    parent_module.add_class::<PyTleType>()?;
    parent_module.add_class::<PyClassification>()?;
    parent_module.add_class::<PyB3Type>()?;
    parent_module.add_class::<PyPositionInTrack>()?;
    parent_module.add_class::<PyAssociationStatus>()?;
    parent_module.add_class::<PyFundamentalCatalog>()?;
    parent_module.add_class::<PySGP4OutputEphemerisFrame>()?;
    Ok(())
}

fn enum_member<T: PyTypeInfo>(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let enum_type = py.get_type::<T>();
    let member = enum_type.getattr(name)?;
    Ok(member.into())
}

pub fn py_key_mode(py: Python<'_>, mode: KeyMode) -> PyResult<PyObject> {
    let name = match mode {
        KeyMode::NoDuplicates => "NoDuplicates",
        KeyMode::DirectMemoryAccess => "DirectMemoryAccess",
    };
    enum_member::<PyKeyMode>(py, name)
}

pub fn py_elset_key_mode(py: Python<'_>, mode: ElsetKeyMode) -> PyResult<PyObject> {
    let name = match mode {
        ElsetKeyMode::NoDuplicates => "NoDuplicates",
        ElsetKeyMode::DirectMemoryAccess => "DirectMemoryAccess",
    };
    enum_member::<PyElsetKeyMode>(py, name)
}

pub fn py_duplicate_key_mode(py: Python<'_>, mode: DuplicateKeyMode) -> PyResult<PyObject> {
    let name = match mode {
        DuplicateKeyMode::ReturnZero => "ReturnZero",
        DuplicateKeyMode::ReturnKey => "ReturnKey",
    };
    enum_member::<PyDuplicateKeyMode>(py, name)
}

pub fn py_fundamental_catalog(py: Python<'_>, catalog: FundamentalCatalog) -> PyResult<PyObject> {
    let name = match catalog {
        FundamentalCatalog::Four => "Four",
        FundamentalCatalog::Five => "Five",
    };
    enum_member::<PyFundamentalCatalog>(py, name)
}

#[pyclass(name = "FundamentalCatalog", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyFundamentalCatalog {
    Four,
    Five,
}

impl From<PyFundamentalCatalog> for FundamentalCatalog {
    fn from(value: PyFundamentalCatalog) -> Self {
        match value {
            PyFundamentalCatalog::Four => FundamentalCatalog::Four,
            PyFundamentalCatalog::Five => FundamentalCatalog::Five,
        }
    }
}

impl From<FundamentalCatalog> for PyFundamentalCatalog {
    fn from(value: FundamentalCatalog) -> Self {
        match value {
            FundamentalCatalog::Four => PyFundamentalCatalog::Four,
            FundamentalCatalog::Five => PyFundamentalCatalog::Five,
        }
    }
}

#[pymethods]
impl PyFundamentalCatalog {
    fn __int__(&self) -> i32 {
        match self {
            PyFundamentalCatalog::Four => FundamentalCatalog::Four as i32,
            PyFundamentalCatalog::Five => FundamentalCatalog::Five as i32,
        }
    }
}

#[pyclass(name = "SGP4OutputEphemerisFrame", eq, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PySGP4OutputEphemerisFrame {
    TEME,
    J2000,
}

impl From<PySGP4OutputEphemerisFrame> for SGP4OutputEphemerisFrame {
    fn from(value: PySGP4OutputEphemerisFrame) -> Self {
        match value {
            PySGP4OutputEphemerisFrame::TEME => SGP4OutputEphemerisFrame::TEME,
            PySGP4OutputEphemerisFrame::J2000 => SGP4OutputEphemerisFrame::J2000,
        }
    }
}

impl From<SGP4OutputEphemerisFrame> for PySGP4OutputEphemerisFrame {
    fn from(value: SGP4OutputEphemerisFrame) -> Self {
        match value {
            SGP4OutputEphemerisFrame::TEME => PySGP4OutputEphemerisFrame::TEME,
            SGP4OutputEphemerisFrame::J2000 => PySGP4OutputEphemerisFrame::J2000,
        }
    }
}

#[pymethods]
impl PySGP4OutputEphemerisFrame {
    fn __int__(&self) -> i32 {
        match self {
            PySGP4OutputEphemerisFrame::TEME => SGP4OutputEphemerisFrame::TEME as i32,
            PySGP4OutputEphemerisFrame::J2000 => SGP4OutputEphemerisFrame::J2000 as i32,
        }
    }
}
