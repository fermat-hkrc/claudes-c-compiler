/// IR constants: numeric literal representations used throughout the compiler.
///
/// `IrConst` represents compile-time constant values (integers, floats, zero).
/// `ConstHashKey` provides a hashable wrapper using bit patterns for floats.
///
/// Float encoding utilities (`f64_to_f128_bytes`, `f64_to_x87_bytes`) convert
/// between Rust f64 and target-specific long double formats.
use crate::common::types::IrType;

/// IR constants.
#[derive(Debug, Clone, Copy)]
pub enum IrConst {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    /// 128-bit integer constant (signed or unsigned, stored as i128).
    I128(i128),
    F32(f32),
    F64(f64),
    /// Long double constant with full precision.
    /// - `f64`: approximate value for computations (lossy, 52-bit mantissa).
    /// - `[u8; 16]`: IEEE 754 binary128 (f128) bytes with full 112-bit mantissa precision.
    ///   For ARM64/RISC-V, these bytes are used directly for data emission.
    ///   For x86, they are converted to x87 80-bit format at emission time.
    LongDouble(f64, [u8; 16]),
    Zero,
}

/// Hashable representation of IR constants, using bit patterns for floats.
/// This allows constants to be used as HashMap keys for value numbering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstHashKey {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(u32),
    F64(u64),
    LongDouble([u8; 16]),
    Zero,
}

/// Convert an f64 value to IEEE 754 binary128 (quad-precision) encoding (16 bytes, little-endian).
/// Quad format: 1 sign bit, 15 exponent bits (bias 16383), 112 mantissa bits (implicit leading 1).
/// This is used for long double on AArch64 and RISC-V.
pub fn f64_to_f128_bytes(val: f64) -> [u8; 16] {
    let bits = val.to_bits();
    let sign = (bits >> 63) & 1;
    let exp11 = ((bits >> 52) & 0x7FF) as i64;
    let mantissa52 = bits & 0x000F_FFFF_FFFF_FFFF;

    if exp11 == 0 && mantissa52 == 0 {
        // Zero (positive or negative)
        let mut bytes = [0u8; 16];
        if sign == 1 {
            bytes[15] = 0x80; // sign bit in MSB
        }
        return bytes;
    }

    if exp11 == 0x7FF {
        // Infinity or NaN
        let exp15: u16 = 0x7FFF;
        if mantissa52 == 0 {
            // Infinity
            let mut bytes = [0u8; 16];
            bytes[14] = (exp15 & 0xFF) as u8;
            bytes[15] = ((exp15 >> 8) as u8) | ((sign as u8) << 7);
            return bytes;
        } else {
            // NaN - set high mantissa bit
            let mut bytes = [0u8; 16];
            bytes[13] = 0x80; // quiet NaN bit
            bytes[14] = (exp15 & 0xFF) as u8;
            bytes[15] = ((exp15 >> 8) as u8) | ((sign as u8) << 7);
            return bytes;
        }
    }

    // Normal number
    // f64 exponent bias is 1023, f128 exponent bias is 16383
    let exp15 = (exp11 - 1023 + 16383) as u16;
    // f64 has 52-bit mantissa (implicit 1.), f128 has 112-bit mantissa (implicit 1.)
    // Shift the 52-bit mantissa to the top of the 112-bit mantissa field:
    // mantissa112 = mantissa52 << (112 - 52) = mantissa52 << 60
    // The 112-bit mantissa occupies bits [0..111] of the 128-bit value
    // Layout (little-endian): bytes[0..13] = mantissa (112 bits = 14 bytes),
    //   bytes[14..15] = exponent[0..14] (15 bits) + sign (1 bit)
    // But actually: the quad format is:
    //   bit 127 = sign, bits [112..126] = exponent (15 bits), bits [0..111] = mantissa
    // In little-endian u128:
    let mantissa112: u128 = (mantissa52 as u128) << 60;
    let exp_sign: u128 = ((exp15 as u128) << 112) | ((sign as u128) << 127);
    let val128 = mantissa112 | exp_sign;
    val128.to_le_bytes()
}

/// Convert an f64 value to x87 80-bit extended precision encoding (10 bytes, little-endian).
/// x87 format: 1 sign bit, 15 exponent bits (bias 16383), 64 mantissa bits (explicit integer bit).
/// Used for long double on x86-64.
pub fn f64_to_x87_bytes(val: f64) -> [u8; 10] {
    let bits = val.to_bits();
    let sign = (bits >> 63) & 1;
    let exp11 = ((bits >> 52) & 0x7FF) as i64;
    let mantissa52 = bits & 0x000F_FFFF_FFFF_FFFF;

    if exp11 == 0 && mantissa52 == 0 {
        // Zero
        let mut bytes = [0u8; 10];
        if sign == 1 {
            bytes[9] = 0x80;
        }
        return bytes;
    }

    if exp11 == 0x7FF {
        // Infinity or NaN
        if mantissa52 == 0 {
            // Infinity
            let mut bytes = [0u8; 10];
            bytes[7] = 0x80; // integer bit set, mantissa = 0
            let exp15: u16 = 0x7FFF;
            bytes[8] = (exp15 & 0xFF) as u8;
            bytes[9] = ((exp15 >> 8) as u8) | ((sign as u8) << 7);
            return bytes;
        } else {
            // NaN
            let mut bytes = [0xFFu8; 10];
            bytes[8] = 0xFF;
            bytes[9] = 0x7F | ((sign as u8) << 7);
            return bytes;
        }
    }

    // Normal number
    // f64 exponent bias is 1023, x87 exponent bias is 16383
    let exp15 = (exp11 - 1023 + 16383) as u16;
    // f64 has 52-bit mantissa (implicit 1.), x87 has 64-bit mantissa (explicit 1.)
    // Shift mantissa: 52 bits -> 63 bits (bottom), then set bit 63 (integer bit)
    let mantissa64 = (1u64 << 63) | (mantissa52 << 11);

    let mut bytes = [0u8; 10];
    // Bytes 0..7: mantissa (64 bits, little-endian)
    bytes[..8].copy_from_slice(&mantissa64.to_le_bytes());
    // Bytes 8..9: exponent (15 bits) + sign (1 bit)
    bytes[8] = (exp15 & 0xFF) as u8;
    bytes[9] = ((exp15 >> 8) as u8) | ((sign as u8) << 7);
    bytes
}

impl IrConst {
    /// Returns true if this constant is zero (integer or float).
    pub fn is_zero(&self) -> bool {
        match self {
            IrConst::I8(0) | IrConst::I16(0) | IrConst::I32(0) | IrConst::I64(0) | IrConst::I128(0) => true,
            IrConst::F32(v) => *v == 0.0,
            IrConst::F64(v) => *v == 0.0,
            IrConst::LongDouble(v, _) => *v == 0.0,
            IrConst::Zero => true,
            _ => false,
        }
    }

    /// Create a LongDouble constant from an f64 value (low precision - bytes derived from f64).
    /// Use this when no full-precision source text is available.
    pub fn long_double(v: f64) -> IrConst {
        IrConst::LongDouble(v, crate::common::long_double::f64_to_f128_bytes_lossless(v))
    }

    /// Create a LongDouble constant with full-precision f128 bytes.
    pub fn long_double_with_bytes(v: f64, bytes: [u8; 16]) -> IrConst {
        IrConst::LongDouble(v, bytes)
    }

    /// Create a LongDouble constant from a signed i64 with full precision.
    /// Uses direct integer-to-f128 conversion to preserve all 64 bits of precision.
    pub fn long_double_from_i64(val: i64) -> IrConst {
        let bytes = crate::common::long_double::i64_to_f128_bytes(val);
        IrConst::LongDouble(val as f64, bytes)
    }

    /// Create a LongDouble constant from an unsigned u64 with full precision.
    /// Uses direct integer-to-f128 conversion to preserve all 64 bits of precision.
    pub fn long_double_from_u64(val: u64) -> IrConst {
        let bytes = crate::common::long_double::u64_to_f128_bytes(val);
        IrConst::LongDouble(val as f64, bytes)
    }

    /// Create a LongDouble constant from an unsigned u128 with maximum precision.
    /// Uses direct integer-to-f128 conversion. Values up to 2^112 are exact;
    /// larger values are rounded (f128 has 112-bit mantissa).
    pub fn long_double_from_u128(val: u128) -> IrConst {
        let bytes = crate::common::long_double::u128_to_f128_bytes(val);
        IrConst::LongDouble(val as f64, bytes)
    }

    /// Create a LongDouble constant from a signed i128 with maximum precision.
    /// Uses direct integer-to-f128 conversion. Values with magnitude up to 2^112
    /// are exact; larger values are rounded.
    pub fn long_double_from_i128(val: i128) -> IrConst {
        let bytes = crate::common::long_double::i128_to_f128_bytes(val);
        IrConst::LongDouble(val as f64, bytes)
    }

    /// Get the raw f128 bytes from a LongDouble constant.
    pub fn long_double_bytes(&self) -> Option<&[u8; 16]> {
        match self {
            IrConst::LongDouble(_, bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Get the x87 80-bit byte representation for any float constant.
    /// For LongDouble, converts stored f128 bytes to x87 format (lossy: 112→64 bit mantissa).
    /// For F64/F32, converts to x87 format (widening, zero-fills lower mantissa bits).
    /// For integer types, converts to f64 first then to x87 bytes.
    pub fn x87_bytes(&self) -> [u8; 16] {
        match self {
            IrConst::LongDouble(_, f128_bytes) => {
                crate::common::long_double::f128_bytes_to_x87_bytes(f128_bytes)
            }
            _ => {
                if let Some(v) = self.to_f64() {
                    crate::common::long_double::f64_to_x87_bytes_simple(v)
                } else {
                    [0u8; 16]
                }
            }
        }
    }

    /// Returns true if this constant is one (integer only).
    pub fn is_one(&self) -> bool {
        matches!(self, IrConst::I8(1) | IrConst::I16(1) | IrConst::I32(1) | IrConst::I64(1) | IrConst::I128(1))
    }

    /// Returns true if this constant is nonzero (for truthiness checks in const eval).
    pub fn is_nonzero(&self) -> bool {
        !self.is_zero()
    }

    /// Convert to a hashable key representation (using bit patterns for floats).
    pub fn to_hash_key(self) -> ConstHashKey {
        match self {
            IrConst::I8(v) => ConstHashKey::I8(v),
            IrConst::I16(v) => ConstHashKey::I16(v),
            IrConst::I32(v) => ConstHashKey::I32(v),
            IrConst::I64(v) => ConstHashKey::I64(v),
            IrConst::I128(v) => ConstHashKey::I128(v),
            IrConst::F32(v) => ConstHashKey::F32(v.to_bits()),
            IrConst::F64(v) => ConstHashKey::F64(v.to_bits()),
            IrConst::LongDouble(_, bytes) => ConstHashKey::LongDouble(bytes),
            IrConst::Zero => ConstHashKey::Zero,
        }
    }

    /// Extract as f64 (works for all numeric types).
    pub fn to_f64(self) -> Option<f64> {
        match self {
            IrConst::I8(v) => Some(v as f64),
            IrConst::I16(v) => Some(v as f64),
            IrConst::I32(v) => Some(v as f64),
            IrConst::I64(v) => Some(v as f64),
            IrConst::I128(v) => Some(v as f64),
            IrConst::F32(v) => Some(v as f64),
            IrConst::F64(v) => Some(v),
            IrConst::LongDouble(v, _) => Some(v),
            IrConst::Zero => Some(0.0),
        }
    }

    /// Cast a float value (as f64) to the target IR type, producing a new IrConst.
    /// For unsigned integer targets, converts via the unsigned type first to get correct
    /// wrapping behavior (e.g., 200.0 as u8 = 200, not saturated to i8 max).
    pub fn cast_float_to_target(fv: f64, target: IrType) -> Option<IrConst> {
        Some(match target {
            IrType::F64 => IrConst::F64(fv),
            IrType::F128 => IrConst::long_double(fv),
            IrType::F32 => IrConst::F32(fv as f32),
            IrType::I8 => IrConst::I8(fv as i8),
            IrType::U8 => IrConst::I8(fv as u8 as i8),
            IrType::I16 => IrConst::I16(fv as i16),
            IrType::U16 => IrConst::I16(fv as u16 as i16),
            IrType::I32 => IrConst::I32(fv as i32),
            IrType::U32 => IrConst::I64(fv as u32 as i64),
            IrType::I64 => IrConst::I64(fv as i64),
            IrType::Ptr => IrConst::ptr_int(fv as i64),
            IrType::U64 => IrConst::I64(fv as u64 as i64),
            IrType::I128 => IrConst::I128(fv as i128),
            IrType::U128 => IrConst::I128(fv as u128 as i128),
            _ => return None,
        })
    }

    /// Cast a long double (with f128 bytes) to the target IR type.
    /// Uses full precision for integer conversions.
    pub fn cast_long_double_to_target(fv: f64, bytes: &[u8; 16], target: IrType) -> Option<IrConst> {
        use crate::common::long_double::{f128_bytes_to_i64, f128_bytes_to_u64, f128_bytes_to_i128, f128_bytes_to_u128};
        Some(match target {
            IrType::F64 => IrConst::F64(fv),
            IrType::F128 => IrConst::long_double_with_bytes(fv, *bytes),
            IrType::F32 => IrConst::F32(fv as f32),
            // For integer targets, use full precision via f128 bytes
            IrType::I8 => IrConst::I8(f128_bytes_to_i64(bytes)? as i8),
            IrType::U8 => IrConst::I8(f128_bytes_to_u64(bytes)? as u8 as i8),
            IrType::I16 => IrConst::I16(f128_bytes_to_i64(bytes)? as i16),
            IrType::U16 => IrConst::I16(f128_bytes_to_u64(bytes)? as u16 as i16),
            IrType::I32 => IrConst::I32(f128_bytes_to_i64(bytes)? as i32),
            IrType::U32 => IrConst::I64(f128_bytes_to_u64(bytes)? as u32 as i64),
            IrType::I64 => IrConst::I64(f128_bytes_to_i64(bytes)?),
            IrType::Ptr => IrConst::ptr_int(f128_bytes_to_i64(bytes)?),
            IrType::U64 => IrConst::I64(f128_bytes_to_u64(bytes)? as i64),
            IrType::I128 => IrConst::I128(f128_bytes_to_i128(bytes)?),
            IrType::U128 => IrConst::I128(f128_bytes_to_u128(bytes)? as i128),
            _ => return None,
        })
    }

    /// Extract as i64 (integer constants only; floats return None).
    pub fn to_i64(self) -> Option<i64> {
        match self {
            IrConst::I8(v) => Some(v as i64),
            IrConst::I16(v) => Some(v as i64),
            IrConst::I32(v) => Some(v as i64),
            IrConst::I64(v) => Some(v),
            IrConst::I128(v) => Some(v as i64),
            IrConst::Zero => Some(0),
            IrConst::F32(_) | IrConst::F64(_) | IrConst::LongDouble(..) => None,
        }
    }

    /// Extract as i128 (integer constants only; floats return None).
    /// Unlike to_i64(), this preserves the full 128-bit value.
    pub fn to_i128(self) -> Option<i128> {
        match self {
            IrConst::I8(v) => Some(v as i128),
            IrConst::I16(v) => Some(v as i128),
            IrConst::I32(v) => Some(v as i128),
            IrConst::I64(v) => Some(v as i128),
            IrConst::I128(v) => Some(v),
            IrConst::Zero => Some(0),
            IrConst::F32(_) | IrConst::F64(_) | IrConst::LongDouble(..) => None,
        }
    }

    /// Extract as u64 (integer constants only; floats return None).
    pub fn to_u64(self) -> Option<u64> {
        match self {
            IrConst::I8(v) => Some(v as u64),
            IrConst::I16(v) => Some(v as u64),
            IrConst::I32(v) => Some(v as u64),
            IrConst::I64(v) => Some(v as u64),
            IrConst::I128(v) => Some(v as u64),
            IrConst::Zero => Some(0),
            IrConst::F32(_) | IrConst::F64(_) | IrConst::LongDouble(..) => None,
        }
    }

    /// Extract as usize (integer constants only).
    pub fn to_usize(self) -> Option<usize> {
        self.to_i64().map(|v| v as usize)
    }

    /// Extract as u32 (integer constants only).
    pub fn to_u32(self) -> Option<u32> {
        self.to_i64().map(|v| v as u32)
    }

    /// Convert to bytes in little-endian format, pushing onto a byte buffer.
    /// Writes `size` bytes for integer types, or full float representation for floats.
    /// For long double on ARM64, emits f64 approximation + zero padding (matching the
    /// ARM64 codegen's f64-based internal representation). For RISC-V, use
    /// `push_le_bytes_riscv` which emits full IEEE binary128.
    /// Use `push_le_bytes_x86` for x86 targets that need x87 80-bit extended precision.
    pub fn push_le_bytes(&self, out: &mut Vec<u8>, size: usize) {
        match self {
            IrConst::F32(v) => {
                out.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            IrConst::F64(v) => {
                out.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            IrConst::LongDouble(_, f128_bytes) => {
                // Default: emit full f128 bytes. For architecture-specific emission,
                // use push_le_bytes_x86 (x87 format) or push_le_bytes_riscv (f128 format).
                out.extend_from_slice(f128_bytes);
            }
            IrConst::I128(v) => {
                let le_bytes = v.to_le_bytes();
                out.extend_from_slice(&le_bytes[..size.min(16)]);
            }
            _ => {
                let le_bytes = self.to_i64().unwrap_or(0).to_le_bytes();
                if size <= 8 {
                    out.extend_from_slice(&le_bytes[..size]);
                } else {
                    // For sizes > 8 (e.g. __int128), zero-extend
                    out.extend_from_slice(&le_bytes);
                    out.extend_from_slice(&vec![0u8; size - 8]);
                }
            }
        }
    }

    /// Convert to bytes for x86-64 targets, using x87 80-bit extended precision
    /// for long doubles. This ensures correct memory representation when code
    /// type-puns long doubles through unions or integer arrays (e.g., TCC's CValue).
    pub fn push_le_bytes_x86(&self, out: &mut Vec<u8>, size: usize) {
        match self {
            IrConst::LongDouble(_, f128_bytes) => {
                // x86-64: convert f128 bytes to x87 80-bit format, emit 10 bytes + 6 padding.
                let x87 = crate::common::long_double::f128_bytes_to_x87_bytes(f128_bytes);
                out.extend_from_slice(&x87[..10]);
                out.extend_from_slice(&[0u8; 6]); // pad to 16 bytes
            }
            _ => self.push_le_bytes(out, size),
        }
    }

    /// Convert to bytes for RISC-V/ARM64 targets, using IEEE 754 binary128 format for
    /// long doubles. The f128 bytes are stored directly since they are already in
    /// IEEE binary128 format.
    pub fn push_le_bytes_riscv(&self, out: &mut Vec<u8>, size: usize) {
        match self {
            IrConst::LongDouble(_, f128_bytes) => {
                // ARM64/RISC-V: f128 bytes are already in IEEE binary128 format.
                out.extend_from_slice(f128_bytes);
            }
            _ => self.push_le_bytes(out, size),
        }
    }

    /// Create a pointer-width integer constant: I32 on ILP32 targets, I64 on LP64.
    /// Use this instead of hardcoding `IrConst::I64(val)` for pointer arithmetic
    /// step sizes, element sizes, and other constants in pointer-width operations.
    pub fn ptr_int(val: i64) -> Self {
        if crate::common::types::target_is_32bit() {
            IrConst::I32(val as i32)
        } else {
            IrConst::I64(val)
        }
    }

    /// Construct an IrConst of the given type from an i64 value.
    ///
    /// Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended
    /// values to preserve unsigned semantics. Storing them in their native
    /// IrConst variants (I8, I16, I32) would cause to_i64() to sign-extend,
    /// turning e.g. U8(255) into -1 instead of 255.
    pub fn from_i64(val: i64, ty: IrType) -> Self {
        match ty {
            IrType::I8 => IrConst::I8(val as i8),
            // U8: store as I64 with zero-extended value to preserve unsigned semantics.
            // Using I8 would sign-extend when to_i64() is called (e.g.,
            // I8(0xFF as i8) = I8(-1) becomes -1 instead of 255).
            IrType::U8 => IrConst::I64(val as u8 as i64),
            IrType::I16 => IrConst::I16(val as i16),
            // U16: same as U8 - store as I64 with zero-extended value.
            IrType::U16 => IrConst::I64(val as u16 as i64),
            IrType::I32 => IrConst::I32(val as i32),
            // U32: store as I64 with zero-extended value to preserve unsigned semantics.
            // Using I32 would sign-extend when loaded as a 64-bit immediate (e.g.,
            // I32(-2147483648) becomes 0xFFFFFFFF80000000 instead of 0x0000000080000000).
            IrType::U32 => IrConst::I64(val as u32 as i64),
            // Ptr: use target-appropriate width (I32 on ILP32, I64 on LP64)
            IrType::Ptr => IrConst::ptr_int(val),
            // I128: sign-extend i64 to i128 (preserves signed value)
            IrType::I128 => IrConst::I128(val as i128),
            // U128: zero-extend by first reinterpreting i64 as u64, then widening to u128.
            // Using `val as i128` would sign-extend, turning e.g. the unsigned value
            // 0xCAFEBABE12345678 (stored as negative i64) into 0xFFFFFFFFFFFFFFFF_CAFEBABE12345678
            // instead of the correct 0x00000000_00000000_CAFEBABE12345678.
            IrType::U128 => IrConst::I128((val as u64 as u128) as i128),
            IrType::F32 => IrConst::F32(val as f32),
            IrType::F64 => IrConst::F64(val as f64),
            IrType::F128 => IrConst::long_double_from_i64(val),
            _ => IrConst::I64(val),
        }
    }

    /// Coerce this constant to match a target IrType, with optional source type for signedness.
    pub fn coerce_to_with_src(&self, target_ty: IrType, src_ty: Option<IrType>) -> IrConst {
        // Check if already the right type
        match (self, target_ty) {
            (IrConst::I8(_), IrType::I8 | IrType::U8) => return *self,
            (IrConst::I16(_), IrType::I16 | IrType::U16) => return *self,
            (IrConst::I32(_), IrType::I32) => return *self,
            // U32 is stored as I64 (zero-extended), so I32 must be converted
            (IrConst::I64(_), IrType::U32) => return *self,
            (IrConst::I64(_), IrType::I64 | IrType::U64) => return *self,
            // Ptr: on LP64 I64 is already correct; on ILP32 we need I32
            (IrConst::I64(v), IrType::Ptr) => {
                if crate::common::types::target_is_32bit() {
                    return IrConst::I32(*v as i32);
                }
                return *self;
            }
            (IrConst::I32(_), IrType::Ptr) => {
                if crate::common::types::target_is_32bit() {
                    return *self;
                }
                // On LP64, widen I32 to I64 for Ptr
            }
            (IrConst::I128(_), IrType::I128 | IrType::U128) => return *self,
            (IrConst::F32(_), IrType::F32) => return *self,
            (IrConst::F64(_), IrType::F64) => return *self,
            (IrConst::LongDouble(..), IrType::F64 | IrType::F128) => return *self,
            _ => {}
        }
        // Convert integer types via from_i64, with unsigned-aware paths
        if let Some(int_val) = self.to_i64() {
            // When the source type is unsigned, we need to zero-extend (not sign-extend)
            // when widening to a larger type. Mask to the source width to get the correct
            // unsigned value (e.g., U32 0xFFFFFFF8 = 4294967288, not -8).
            if let Some(src) = src_ty.filter(|t| t.is_unsigned()) {
                // Mask to the source type's width to get the correct unsigned value
                let src_size = src.size();
                let uint_val = match src_size {
                    1 => (int_val as u8) as u64,
                    2 => (int_val as u16) as u64,
                    4 => (int_val as u32) as u64,
                    8 => int_val as u64,
                    _ => int_val as u64,
                };
                if target_ty.is_float() {
                    return match target_ty {
                        IrType::F32 => IrConst::F32(uint_val as f32),
                        IrType::F64 => IrConst::F64(uint_val as f64),
                        IrType::F128 => IrConst::long_double_from_u64(uint_val),
                        _ => IrConst::I64(uint_val as i64),
                    };
                }
                // For integer targets, use the zero-extended value
                return IrConst::from_i64(uint_val as i64, target_ty);
            }
            return IrConst::from_i64(int_val, target_ty);
        }
        // Convert float types: extract as f64 and use cast_float_to_target
        if let Some(fv) = self.to_f64() {
            if let Some(result) = Self::cast_float_to_target(fv, target_ty) {
                return result;
            }
        }
        *self
    }

    /// Coerce this constant to match a target IrType (assumes signed source for int-to-float).
    pub fn coerce_to(&self, target_ty: IrType) -> IrConst {
        self.coerce_to_with_src(target_ty, None)
    }

    /// Normalize a constant for _Bool storage: any nonzero value becomes I8(1), zero becomes I8(0).
    /// Implements C11 6.3.1.2: "When any scalar value is converted to _Bool, the result
    /// is 0 if the value compares equal to 0; otherwise, the result is 1."
    pub fn bool_normalize(&self) -> IrConst {
        IrConst::I8(if self.is_zero() { 0 } else { 1 })
    }

    /// Get the zero constant for a given IR type.
    pub fn zero(ty: IrType) -> IrConst {
        match ty {
            IrType::I8 | IrType::U8 => IrConst::I8(0),
            IrType::I16 | IrType::U16 => IrConst::I16(0),
            IrType::I32 => IrConst::I32(0),
            IrType::U32 => IrConst::I64(0),
            IrType::F32 => IrConst::F32(0.0),
            IrType::F64 => IrConst::F64(0.0),
            IrType::F128 => IrConst::long_double(0.0),
            _ => IrConst::I64(0),
        }
    }

    /// Narrow a constant to match a target IR type.
    /// If the constant is wider than needed (e.g., I64 for an I32 slot),
    /// truncate it to the correct width. This preserves the numeric value
    /// for values that fit, and truncates for values that don't.
    pub fn narrowed_to(self, ty: IrType) -> IrConst {
        match (&self, ty) {
            // Already the right type or narrower — return as-is
            (IrConst::I8(_), IrType::I8 | IrType::U8) => self,
            (IrConst::I16(_), IrType::I16 | IrType::U16) => self,
            (IrConst::I32(_), IrType::I32) => self,
            (IrConst::F32(_), IrType::F32) => self,
            (IrConst::F64(_), IrType::F64) => self,
            // Wide integer constant being stored to a narrower slot
            (IrConst::I64(v), IrType::I8) => IrConst::I8(*v as i8),
            (IrConst::I64(v), IrType::U8) => IrConst::I64(*v as u8 as i64),
            (IrConst::I64(v), IrType::I16) => IrConst::I16(*v as i16),
            (IrConst::I64(v), IrType::U16) => IrConst::I64(*v as u16 as i64),
            (IrConst::I64(v), IrType::I32) => IrConst::I32(*v as i32),
            // U32 must use I64 with zero-extension to match from_i64() convention.
            // Using I32 would sign-extend when to_i64() is called (e.g.,
            // 0xFFFFFFFF stored as I32(-1) becomes -1 instead of 4294967295).
            (IrConst::I64(v), IrType::U32) => IrConst::I64(*v as u32 as i64),
            (IrConst::I32(v), IrType::I8) => IrConst::I8(*v as i8),
            (IrConst::I32(v), IrType::U8) => IrConst::I64(*v as u8 as i64),
            (IrConst::I32(v), IrType::I16) => IrConst::I16(*v as i16),
            (IrConst::I32(v), IrType::U16) => IrConst::I64(*v as u16 as i64),
            (IrConst::I32(v), IrType::U32) => IrConst::I64(*v as u32 as i64),
            // Pointer types on 32-bit: I64 -> I32
            (IrConst::I64(v), IrType::Ptr) => {
                if crate::common::types::target_is_32bit() {
                    IrConst::I32(*v as i32)
                } else {
                    self
                }
            }
            // Everything else: keep as-is (F64, I64, I128, etc.)
            _ => self,
        }
    }

    /// Serialize this constant to little-endian bytes.
    /// Returns a Vec containing the value in little-endian byte order.
    pub fn to_le_bytes(self) -> Vec<u8> {
        match self {
            IrConst::I8(v) => vec![v as u8],
            IrConst::I16(v) => v.to_le_bytes().to_vec(),
            IrConst::I32(v) => v.to_le_bytes().to_vec(),
            IrConst::I64(v) => v.to_le_bytes().to_vec(),
            IrConst::I128(v) => v.to_le_bytes().to_vec(),
            IrConst::F32(v) => v.to_bits().to_le_bytes().to_vec(),
            IrConst::F64(v) => v.to_bits().to_le_bytes().to_vec(),
            IrConst::LongDouble(_, bytes) => bytes.to_vec(),
            IrConst::Zero => vec![0],
        }
    }

    /// Get the one constant for a given IR type.
    pub fn one(ty: IrType) -> IrConst {
        match ty {
            IrType::I8 | IrType::U8 => IrConst::I8(1),
            IrType::I16 | IrType::U16 => IrConst::I16(1),
            IrType::I32 => IrConst::I32(1),
            IrType::U32 => IrConst::I64(1),
            IrType::F32 => IrConst::F32(1.0),
            IrType::F64 => IrConst::F64(1.0),
            IrType::F128 => IrConst::long_double(1.0),
            _ => IrConst::I64(1),
        }
    }
}

// ── PBT properties (round 06) ─────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::types::IrType;
    use proptest::prelude::*;

    /// P5: unsigned representation convention — a constant of unsigned sub-64-bit type
    /// must be stored zero-extended so to_i64() reads back the unsigned value. All
    /// three writers of "a U8/U16/U32 constant with value v" must agree.
    /// Doc contract (from_i64): "Store unsigned sub-64-bit types (U8, U16, U32) as I64
    /// with zero-extended values to preserve unsigned semantics." (86a284c8)
    /// Doc contract (cast_float_to_target): "For unsigned integer targets, converts via
    /// the unsigned type first to get correct wrapping behavior (e.g., 200.0 as u8 = 200,
    /// not saturated to i8 max)." (a21e7eb9)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p5_unsigned_repr_writer_differential(ty_idx in 0usize..3, raw in any::<u32>()) {
            let (ty, mask): (IrType, u32) = match ty_idx {
                0 => (IrType::U8, u8::MAX as u32),
                1 => (IrType::U16, u16::MAX as u32),
                _ => (IrType::U32, u32::MAX),
            };
            let v = raw & mask;

            // writer 1: from_i64 (the convention's author)
            prop_assert_eq!(IrConst::from_i64(v as i64, ty).to_i64(), Some(v as i64),
                "from_i64 {:?} {}", ty, v);

            // writer 2: cast_float_to_target — same job, must read back the same value
            let cf = IrConst::cast_float_to_target(v as f64, ty)
                .unwrap_or_else(|| panic!("cast_float_to_target returned None for {:?}", ty));
            prop_assert_eq!(cf.to_i64(), Some(v as i64),
                "cast_float_to_target({:?}, {}) = {:?} reads back {:?}", ty, v, cf, cf.to_i64());

            // writer 3: cast_long_double_to_target (full-precision integer path)
            let ld = IrConst::long_double_from_u64(v as u64);
            let bytes = *ld.long_double_bytes().expect("long double bytes");
            let cl = IrConst::cast_long_double_to_target(v as f64, &bytes, ty)
                .unwrap_or_else(|| panic!("cast_long_double_to_target returned None for {:?}", ty));
            prop_assert_eq!(cl.to_i64(), Some(v as i64),
                "cast_long_double_to_target({:?}, {}) = {:?} reads back {:?}", ty, v, cl, cl.to_i64());
        }
    }

    /// P6: coerce_to must normalize a signed-width constant to the zero-extended I64
    /// representation when the target type is unsigned sub-64-bit (same convention).
    /// The early-return `(I8(_), U8)` / `(I16(_), U16)` arms keep the forbidden
    /// sign-extending native variant instead.
    /// Doc contract: from_i64 convention comment (constants.rs, 86a284c8).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p6_coerce_to_unsigned_normalization(kind in 0usize..2, raw in any::<u32>()) {
            match kind {
                0 => {
                    let v = 128u32 + (raw % 128); // 128..=255: values whose I8 form is negative
                    let coerced = IrConst::from_i64(v as i64, IrType::I8).coerce_to(IrType::U8);
                    prop_assert_eq!(coerced.to_i64(), Some(v as i64),
                        "I8({}).coerce_to(U8) = {:?} reads back {}", v as i8, coerced, coerced.to_i64().unwrap_or(-1));
                }
                _ => {
                    let v = 32768u32 + (raw % 32768); // 32768..=65535
                    let coerced = IrConst::from_i64(v as i64, IrType::I16).coerce_to(IrType::U16);
                    prop_assert_eq!(coerced.to_i64(), Some(v as i64),
                        "I16({}).coerce_to(U16) = {:?} reads back {}", v as i16, coerced, coerced.to_i64().unwrap_or(-1));
                }
            }
        }
    }

    // Independent IEEE 754 decoders (field definitions per the format specs quoted in
    // the encoders' doc comments — not a transcription of the encoder logic).
    fn decode_f128_to_f64(bytes: &[u8; 16]) -> f64 {
        let bits = u128::from_le_bytes(*bytes);
        let neg = bits >> 127 == 1;
        let sign = if neg { -1.0f64 } else { 1.0 };
        let exp = ((bits >> 112) & 0x7FFF) as i32;
        let mantissa = (bits & ((1u128 << 112) - 1)) as f64;
        if exp == 0x7FFF {
            return if mantissa == 0.0 {
                sign * f64::INFINITY
            } else {
                f64::NAN
            };
        }
        if exp == 0 {
            // f128 subnormal: cannot represent an f64-sourced value except zero
            return sign * mantissa * 2.0f64.powi(1 - 16383 - 112);
        }
        sign * (1.0 + mantissa / 2.0f64.powi(112)) * 2.0f64.powi(exp - 16383)
    }

    fn decode_x87_to_f64(bytes: &[u8; 10]) -> f64 {
        let mant = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        let se = u16::from_le_bytes(bytes[8..].try_into().unwrap());
        let neg = se >> 15 == 1;
        let sign = if neg { -1.0f64 } else { 1.0 };
        let exp = (se & 0x7FFF) as i32;
        if exp == 0x7FFF {
            return if mant & (1u64 << 63) != 0 && mant & !(1u64 << 63) == 0 {
                sign * f64::INFINITY
            } else {
                f64::NAN
            };
        }
        if exp == 0 {
            return sign * (mant as f64) * 2.0f64.powi(1 - 16383 - 63);
        }
        sign * (mant as f64 / 2.0f64.powi(63)) * 2.0f64.powi(exp - 16383)
    }

    /// P7: the encoders' documented contract is a faithful IEEE 754 encoding of the
    /// f64 value; an independent field-level decoder must reconstruct every finite f64,
    /// including subnormals (all f64 subnormals are exactly representable in both
    /// binary128 and x87 80-bit formats).
    /// Doc contract: "Convert an f64 value to IEEE 754 binary128 (quad-precision)
    /// encoding (16 bytes, little-endian)." (a40c100e)
    /// Doc contract: "x87 format: 1 sign bit, 15 exponent bits (bias 16383), 64 mantissa
    /// bits (explicit integer bit)." (3512a019)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p7_f128_x87_encoding_roundtrip(
            v in prop_oneof![
                any::<f64>().prop_filter("finite", |x| x.is_finite()),
                Just(5e-324), Just(-5e-324),
                Just(2.2250738585072011e-308), Just(-2.2250738585072011e-308),
                Just(f64::MIN_POSITIVE), Just(-f64::MIN_POSITIVE),
                Just(0.0), Just(-0.0), Just(1.0), Just(-1.0), Just(0.5),
                Just(f64::MAX), Just(-f64::MAX), Just(1e300), Just(-1e300),
            ],
        ) {
            let f128_bytes = f64_to_f128_bytes(v);
            let decoded = decode_f128_to_f64(&f128_bytes);
            prop_assert!(decoded == v, "f128: v={} decoded={}", v, decoded);
            prop_assert_eq!(f128_bytes[15] >> 7, (v.to_bits() >> 63) as u8, "sign bit lost for {}", v);

            let x87_bytes = f64_to_x87_bytes(v);
            let decoded_x = decode_x87_to_f64(&x87_bytes);
            prop_assert!(decoded_x == v, "x87: v={} decoded={}", v, decoded_x);
            prop_assert_eq!(x87_bytes[9] >> 7, (v.to_bits() >> 63) as u8, "x87 sign bit lost for {}", v);
        }
    }

    /// P7b: special-value encoding (KAT): ±0, ±inf, NaN.
    #[test]
    fn p7b_special_values() {
        for (v, name) in [(0.0f64, "+0"), (-0.0, "-0")] {
            let b = f64_to_f128_bytes(v);
            let bits128 = u128::from_le_bytes(b);
            assert_eq!(bits128 & !(1u128 << 127), 0, "{} f128 has non-sign bits set", name);
            assert_eq!(bits128 >> 127 == 1, v.is_sign_negative(), "{} f128 sign bit", name);
            assert_eq!(decode_f128_to_f64(&b) == 0.0, true);
            let x = f64_to_x87_bytes(v);
            let mant = u64::from_le_bytes(x[..8].try_into().unwrap());
            let se = u16::from_le_bytes(x[8..].try_into().unwrap());
            assert_eq!(mant, 0, "{} x87 mantissa", name);
            assert_eq!(se, if v.is_sign_negative() { 0x8000 } else { 0 }, "{} x87 exp/sign", name);
        }
        for v in [f64::INFINITY, f64::NEG_INFINITY] {
            let b = f64_to_f128_bytes(v);
            assert_eq!(((u128::from_le_bytes(b) >> 112) & 0x7FFF) as i32, 0x7FFF, "inf exponent");
            assert!(decode_f128_to_f64(&b).is_infinite());
            assert_eq!(decode_f128_to_f64(&b).is_sign_negative(), v.is_sign_negative());
            let x = f64_to_x87_bytes(v);
            let se = u16::from_le_bytes(x[8..].try_into().unwrap());
            assert_eq!((se & 0x7FFF) as i32, 0x7FFF, "x87 inf exponent");
            assert!(decode_x87_to_f64(&x).is_infinite());
        }
        for v in [f64::NAN, f64::from_bits(f64::to_bits(f64::NAN) | (1 << 63))] {
            assert!(decode_f128_to_f64(&f64_to_f128_bytes(v)).is_nan(), "f128 NaN");
            assert!(decode_x87_to_f64(&f64_to_x87_bytes(v)).is_nan(), "x87 NaN");
        }
    }

    /// P8: zero/one/bool contracts across every IrType enumerator.
    /// Doc contract: C11 6.3.1.2 quoted in bool_normalize doc (084709b7).
    #[test]
    fn p8_zero_one_bool_contracts() {
        let types = [IrType::I8, IrType::U8, IrType::I16, IrType::U16, IrType::I32,
                     IrType::U32, IrType::I64, IrType::U64, IrType::I128, IrType::U128,
                     IrType::F32, IrType::F64, IrType::F128, IrType::Ptr, IrType::Void];
        for ty in types {
            assert!(IrConst::zero(ty).is_zero(), "zero({:?})", ty);
            assert!(!IrConst::one(ty).is_zero(), "one({:?})", ty);
            assert!(IrConst::one(ty).is_nonzero(), "one({:?})", ty);
        }
        for (k, ty) in [(0i64, IrType::I8), (127, IrType::I8), (-128, IrType::I8),
                        (1, IrType::I32), (-1, IrType::I64), (i32::MAX as i64, IrType::I32)] {
            assert_eq!(IrConst::from_i64(k, ty).to_i64(), Some(k), "from_i64/to_i64 {:?}", ty);
        }
        // float constructions read back through to_f64
        assert_eq!(IrConst::from_i64(3, IrType::F64).to_f64(), Some(3.0));
        assert_eq!(IrConst::from_i64(3, IrType::F128).to_f64(), Some(3.0));
        // C11 6.3.1.2 bool normalization
        for (c, want) in [
            (IrConst::I64(0), 0i8), (IrConst::I64(42), 1), (IrConst::I64(-1), 1),
            (IrConst::I128(0), 0), (IrConst::I128(i128::MIN), 1),
            (IrConst::F64(0.0), 0), (IrConst::F64(-0.0), 0), (IrConst::F64(1e-300), 1),
            (IrConst::F32(f32::NAN), 1), (IrConst::Zero, 0),
            (IrConst::LongDouble(0.0, [0u8; 16]), 0), (IrConst::LongDouble(2.5, [1u8; 16]), 1),
        ] {
            assert_eq!(c.bool_normalize().to_hash_key(), IrConst::I8(want).to_hash_key(), "bool_normalize({:?})", c);
        }
    }
}

// Deterministic red regression witnesses (round 06, bugs b1–b3).
#[cfg(test)]
mod pbt_regression {
    use super::*;
    use crate::common::types::IrType;

    /// b1: cast_float_to_target must store U8/U16/U32 constants zero-extended
    /// (constants.rs from_i64 convention); 200.0 → U8 currently reads back -56.
    #[test]
    fn test_ir_const_regression_cast_float_u8_repr() {
        let c = IrConst::cast_float_to_target(200.0, IrType::U8).unwrap();
        assert_eq!(c.to_i64(), Some(200), "cast_float_to_target(200.0, U8) = {:?}", c);
    }

    /// b2: coerce_to must normalize I8→U8 to the zero-extended I64 representation;
    /// the early-return arm keeps the sign-extending native variant.
    #[test]
    fn test_ir_const_regression_coerce_u8_early_return() {
        let c = IrConst::from_i64(200, IrType::I8).coerce_to(IrType::U8);
        assert_eq!(c.to_i64(), Some(200), "I8(-56).coerce_to(U8) = {:?}", c);
    }

    /// b3: f64 subnormals must encode exactly in f128/x87 (they are normal numbers
    /// there); the current encoders treat them as normals of the source width.
    #[test]
    fn test_ir_const_regression_subnormal_encoding() {
        let v = 5e-324f64; // smallest f64 subnormal
        let b = f64_to_f128_bytes(v);
        let bits = u128::from_le_bytes(b);
        let exp = ((bits >> 112) & 0x7FFF) as i64;
        let mant = bits & ((1u128 << 112) - 1);
        // correct f128 for 2^-1074: exponent field 16383-1023=15360 is WRONG — the
        // value must be m52 * 2^-1074, i.e. a normal f128 with exp 15361 and the
        // implicit integer bit set; assert the decoded value round-trips
        let decoded = if exp == 0 {
            (mant as f64) * 2.0f64.powi(1 - 16383 - 112)
        } else {
            (1.0 + (mant as f64) / 2.0f64.powi(112)) * 2.0f64.powi(exp as i32 - 16383)
        };
        assert_eq!(decoded, v, "f64_to_f128_bytes(5e-324) decodes to {}", decoded);
    }
}
