// Copyright © 2024 Nathaniel Hardesty
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the “Software”), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! ## ITEM FORMAT
//! **Based on SEMI E5§9.2.2**
//! 
//! ----------------------------------------------------------------------------
//! 
//! The list of format codes associated with various [Item] types. This module
//! exists so that these codes can be referenced by name without undue syntax
//! overhead or other restrictions Rust requires when using Enums for a set of
//! constants rather than as a sum type.
//! 
//! [Item]: crate::Item

/// ### LIST
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o00**
pub const LIST: u8 = 0b000000_00;

/// ### BINARY
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o10**
pub const BIN: u8 = 0b001000_00;

/// ### BOOLEAN
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o11**
pub const BOOL: u8 = 0b001001_00;

/// ### ASCII
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o20**
pub const ASCII: u8 = 0b010000_00;

/// ### JIS-8
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o21**
pub const JIS8: u8 = 0b010001_00;

/// ### LOCALIZED STRING
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o22**
pub const LOCAL: u8 = 0b010010_00;

/// ### 8-BYTE SIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o30**
pub const I8: u8 = 0b011000_00;

/// ### 1-BYTE SIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o31**
pub const I1: u8 = 0b011001_00;

/// ### 2-BYTE SIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o32**
pub const I2: u8 = 0b011010_00;

/// ### 4-BYTE SIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o34**
pub const I4: u8 = 0b011100_00;

/// ### 8-BYTE FLOATING POINT NUMBER
/// **Based on SEMI E5§9.2.2**
/// 
/// - **Format Code 0o40**
pub const F8: u8 = 0b100000_00;

/// ### 4-BYTE FLOATING POINT NUMBER
/// **Based on SEMI E5§9.2.2**
/// 
/// - **Format Code 0o44**
pub const F4: u8 = 0b100100_00;

/// ### 8-BYTE UNSIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// - **Format Code 0o50**
pub const U8: u8 = 0b101000_00;

/// ### 1-BYTE UNSIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o51**
pub const U1: u8 = 0b101001_00;

/// ### 2-BYTE UNSIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// **Format Code 0o52**
pub const U2: u8 = 0b101010_00;

/// ### 4-BYTE UNSIGNED INTEGER
/// **Based on SEMI E5§9.2.2**
/// 
/// - **Format Code 0o54**
pub const U4: u8 = 0b101100_00;
