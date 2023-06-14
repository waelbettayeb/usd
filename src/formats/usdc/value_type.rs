#[repr(u64)]
pub(super) enum ValueType {
    Invalid = 0,

    Bool = 1,
    UChar = 2,
    Int = 3,
    UInt = 4,
    Int64 = 5,
    UInt64 = 6,

    Half = 7,
    Float = 8,
    Double = 9,

    String = 10,

    Token = 11,

    AssetPath = 12,

    Quatd = 16,
    Quatf = 17,
    Quath = 18,

    Vec2d = 19,
    Vec2f = 20,
    Vec2h = 21,
    Vec2i = 22,

    Vec3d = 23,
    Vec3f = 24,
    Vec3h = 25,
    Vec3i = 26,

    Vec4d = 27,
    Vec4f = 28,
    Vec4h = 29,
    Vec4i = 30,

    Matrix2d = 13,
    Matrix3d = 14,
    Matrix4d = 15,

    Dictionary = 31,

    TokenListOp = 32,
    StringListOp = 33,
    PathListOp = 34,
    ReferenceListOp = 35,
    IntListOp = 36,
    Int64ListOp = 37,
    UIntListOp = 38,
    UInt64ListOp = 39,

    PathVector = 40,
    TokenVector = 41,

    Specifier = 42,
    Permission = 43,
    Variability = 44,

    VariantSelectionMap = 45,
    TimeSamples = 46,
    Payload = 47,

    DoubleVector = 48,
    LayerOffsetVector = 49,
    StringVector = 50,

    ValueBlock = 51,
    Value = 52,

    UnregisteredValue = 53,
    UnregisteredValueListOp = 54,
    PayloadListOp = 55,
    TimeCode = 56,
}

impl From<u64> for ValueType {
    fn from(value: u64) -> Self {
        match value {
            0 => ValueType::Invalid,
            1 => ValueType::Bool,
            2 => ValueType::UChar,
            3 => ValueType::Int,
            4 => ValueType::UInt,
            5 => ValueType::Int64,
            6 => ValueType::UInt64,

            7 => ValueType::Half,
            8 => ValueType::Float,
            9 => ValueType::Double,

            10 => ValueType::String,

            11 => ValueType::Token,

            12 => ValueType::AssetPath,

            16 => ValueType::Quatd,
            17 => ValueType::Quatf,
            18 => ValueType::Quath,

            19 => ValueType::Vec2d,
            20 => ValueType::Vec2f,
            21 => ValueType::Vec2h,
            22 => ValueType::Vec2i,

            23 => ValueType::Vec3d,
            24 => ValueType::Vec3f,
            25 => ValueType::Vec3h,
            26 => ValueType::Vec3i,

            27 => ValueType::Vec4d,
            28 => ValueType::Vec4f,
            29 => ValueType::Vec4h,
            30 => ValueType::Vec4i,

            13 => ValueType::Matrix2d,
            14 => ValueType::Matrix3d,
            15 => ValueType::Matrix4d,

            31 => ValueType::Dictionary,

            32 => ValueType::TokenListOp,
            33 => ValueType::StringListOp,
            34 => ValueType::PathListOp,
            35 => ValueType::ReferenceListOp,
            36 => ValueType::IntListOp,
            37 => ValueType::Int64ListOp,
            38 => ValueType::UIntListOp,
            39 => ValueType::UInt64ListOp,

            40 => ValueType::PathVector,
            41 => ValueType::TokenVector,

            42 => ValueType::Specifier,
            43 => ValueType::Permission,
            44 => ValueType::Variability,

            45 => ValueType::VariantSelectionMap,
            46 => ValueType::TimeSamples,
            47 => ValueType::Payload,

            48 => ValueType::DoubleVector,
            49 => ValueType::LayerOffsetVector,
            50 => ValueType::StringVector,

            51 => ValueType::ValueBlock,
            52 => ValueType::Value,

            53 => ValueType::UnregisteredValue,
            54 => ValueType::UnregisteredValueListOp,
            55 => ValueType::PayloadListOp,
            56 => ValueType::TimeCode,

            _ => ValueType::Invalid,
        }
    }
}
