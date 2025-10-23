use std::convert::TryFrom;

pub enum Error {
    MissingDLL,

    UnknownVarjoError(i64),
    VarjoError(VarjoError),
}

pub enum VarjoError {
    ErrorUnknown = -1,
    NoError = 0,
    InvalidSession = 1,
    OutOfMemory = 2,
    InvalidVersion = 3,
    GraphicsNotInitialized = 4,
    FrameNotStarted = 5,
    FrameAlreadyStarted = 6,
    ViewIndexOutOfBounds = 7,
    InvalidPoseType = 8,
    NullPointer = 9,
    MixingTextures = 10,
    NaN = 11,
    NoHMDConnected = 12,
    ValidationFailure = 13,
    IndexOutOfBounds = 14,
    AlreadyLocked = 15,
    NotLocked = 16,
    InvalidSize = 17,
    GazeNotInitialized = 100,
    GazeNotConnected = 101,
    GazeAlreadyInitialized = 102,
    GazeNotAllowed = 103,
    InvalidParameter = 104,
    UnsupportedParameter = 105,
}

impl TryFrom<i64> for VarjoError {
    type Error = Error;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            -1 => Ok(Self::ErrorUnknown),
            0 => Ok(Self::NoError),
            1 => Ok(Self::InvalidSession),
            2 => Ok(Self::OutOfMemory),
            3 => Ok(Self::InvalidSession),
            4 => Ok(Self::GraphicsNotInitialized),
            5 => Ok(Self::FrameNotStarted),
            6 => Ok(Self::FrameAlreadyStarted),
            7 => Ok(Self::ViewIndexOutOfBounds),
            8 => Ok(Self::InvalidPoseType),
            9 => Ok(Self::NullPointer),
            10 => Ok(Self::MixingTextures),
            11 => Ok(Self::NaN),
            12 => Ok(Self::NoHMDConnected),
            13 => Ok(Self::ValidationFailure),
            14 => Ok(Self::IndexOutOfBounds),
            15 => Ok(Self::AlreadyLocked),
            16 => Ok(Self::NotLocked),
            17 => Ok(Self::InvalidSize),
            100 => Ok(Self::GazeNotInitialized),
            101 => Ok(Self::GazeNotConnected),
            102 => Ok(Self::GazeAlreadyInitialized),
            103 => Ok(Self::GazeNotAllowed),
            104 => Ok(Self::InvalidParameter),
            105 => Ok(Self::UnsupportedParameter),
            _ => Err(Error::UnknownVarjoError(v)),
        }
    }
}
