#![cfg_attr(rustfmt, rustfmt::skip)]
/// A stack-allocated string which can hold up to 3 UTF-8 encoded bytes.
#[derive(Clone, Copy)]
#[repr(C)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
pub struct NanoString {
    len: NanoStringLen,
    bytes: [u8; 3u8 as _],
}
impl NanoString {
    pub const EMPTY: Self = Self::new("").unwrap();
    pub const fn new(s: &str) -> Option<Self> {
        match NanoStringLen::from_usize(s.len()) {
            Some(len) => {
                let mut bytes = [0; 3u8 as _];
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        s.as_ptr(),
                        bytes.as_mut_ptr(),
                        s.len(),
                    )
                }
                Some(Self { len, bytes })
            }
            None => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(
                ::core::slice::from_raw_parts(
                    self.bytes.as_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
    pub const fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            str::from_utf8_unchecked_mut(
                ::core::slice::from_raw_parts_mut(
                    self.bytes.as_mut_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NanoStringError;
impl ::core::fmt::Display for NanoStringError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str("expected a string of at most 3 bytes")
    }
}
impl ::core::error::Error for NanoStringError {}
impl ::core::default::Default for NanoString {
    fn default() -> Self {
        Self::EMPTY
    }
}
impl ::core::default::Default for &NanoString {
    fn default() -> Self {
        &NanoString::EMPTY
    }
}
impl ::core::hash::Hash for NanoString {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
impl ::core::convert::AsRef<Self> for NanoString {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl ::core::convert::AsMut<Self> for NanoString {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl ::core::convert::AsRef<str> for NanoString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::convert::AsMut<str> for NanoString {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::convert::AsRef<[u8]> for NanoString {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
impl TryFrom<&str> for NanoString {
    type Error = NanoStringError;
    fn try_from(value: &str) -> Result<Self, NanoStringError> {
        Self::new(value).ok_or(NanoStringError)
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::ffi::OsStr> for NanoString {
    fn as_ref(&self) -> &::std::ffi::OsStr {
        self.as_str().as_ref()
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::path::Path> for NanoString {
    fn as_ref(&self) -> &::std::path::Path {
        self.as_str().as_ref()
    }
}
impl ::core::borrow::Borrow<str> for NanoString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl ::core::borrow::BorrowMut<str> for NanoString {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::ops::Deref for NanoString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::ops::DerefMut for NanoString {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialEq<T> for NanoString {
    fn eq(&self, other: &T) -> bool {
        self.as_str().eq(other.as_ref())
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialOrd<T> for NanoString {
    fn partial_cmp(&self, other: &T) -> Option<::core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}
impl ::core::fmt::Debug for NanoString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::fmt::Display for NanoString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::str::FromStr for NanoString {
    type Err = NanoStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or(NanoStringError)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for NanoString {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NanoString {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NanoString;
            fn expecting(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                f.write_str("a string of at most 3 bytes")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<NanoString, E> {
                NanoString::new(v)
                    .ok_or_else(|| serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ))
            }
        }
        d.deserialize_str(Visitor)
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for NanoString {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(::core::stringify!(NanoString))
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema! {
            { "type" : "string", "maxLength" : 3u8, }
        }
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(
            ::core::concat!(::core::module_path!(), "::", ::core::stringify!(NanoString)),
        )
    }
}
#[cfg(feature = "const-default")]
impl const_default::ConstDefault for NanoString {
    const DEFAULT: Self = NanoString::EMPTY;
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
enum NanoStringLen {
    _0 = 0u8,
    _1 = 1u8,
    _2 = 2u8,
    _3 = 3u8,
}
impl NanoStringLen {
    const fn from_usize(u: usize) -> Option<Self> {
        const U8_MAX: usize = u8::MAX as _;
        if u > U8_MAX {
            return None;
        }
        Self::from_u8(u as u8)
    }
    const fn from_u8(u: u8) -> Option<Self> {
        match u {
            0u8 => Some(Self::_0),
            1u8 => Some(Self::_1),
            2u8 => Some(Self::_2),
            3u8 => Some(Self::_3),
            _ => None,
        }
    }
}
/// A stack-allocated string which can hold up to 7 UTF-8 encoded bytes.
#[derive(Clone, Copy)]
#[repr(C)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
pub struct MicroString {
    len: MicroStringLen,
    bytes: [u8; 7u8 as _],
}
impl MicroString {
    pub const EMPTY: Self = Self::new("").unwrap();
    pub const fn new(s: &str) -> Option<Self> {
        match MicroStringLen::from_usize(s.len()) {
            Some(len) => {
                let mut bytes = [0; 7u8 as _];
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        s.as_ptr(),
                        bytes.as_mut_ptr(),
                        s.len(),
                    )
                }
                Some(Self { len, bytes })
            }
            None => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(
                ::core::slice::from_raw_parts(
                    self.bytes.as_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
    pub const fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            str::from_utf8_unchecked_mut(
                ::core::slice::from_raw_parts_mut(
                    self.bytes.as_mut_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicroStringError;
impl ::core::fmt::Display for MicroStringError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str("expected a string of at most 7 bytes")
    }
}
impl ::core::error::Error for MicroStringError {}
impl ::core::default::Default for MicroString {
    fn default() -> Self {
        Self::EMPTY
    }
}
impl ::core::default::Default for &MicroString {
    fn default() -> Self {
        &MicroString::EMPTY
    }
}
impl ::core::hash::Hash for MicroString {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
impl ::core::convert::AsRef<Self> for MicroString {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl ::core::convert::AsMut<Self> for MicroString {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl ::core::convert::AsRef<str> for MicroString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::convert::AsMut<str> for MicroString {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::convert::AsRef<[u8]> for MicroString {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
impl TryFrom<&str> for MicroString {
    type Error = MicroStringError;
    fn try_from(value: &str) -> Result<Self, MicroStringError> {
        Self::new(value).ok_or(MicroStringError)
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::ffi::OsStr> for MicroString {
    fn as_ref(&self) -> &::std::ffi::OsStr {
        self.as_str().as_ref()
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::path::Path> for MicroString {
    fn as_ref(&self) -> &::std::path::Path {
        self.as_str().as_ref()
    }
}
impl ::core::borrow::Borrow<str> for MicroString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl ::core::borrow::BorrowMut<str> for MicroString {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::ops::Deref for MicroString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::ops::DerefMut for MicroString {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialEq<T> for MicroString {
    fn eq(&self, other: &T) -> bool {
        self.as_str().eq(other.as_ref())
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialOrd<T> for MicroString {
    fn partial_cmp(&self, other: &T) -> Option<::core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}
impl ::core::fmt::Debug for MicroString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::fmt::Display for MicroString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::str::FromStr for MicroString {
    type Err = MicroStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or(MicroStringError)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MicroString {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MicroString {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MicroString;
            fn expecting(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                f.write_str("a string of at most 7 bytes")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MicroString, E> {
                MicroString::new(v)
                    .ok_or_else(|| serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ))
            }
        }
        d.deserialize_str(Visitor)
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for MicroString {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(::core::stringify!(MicroString))
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema! {
            { "type" : "string", "maxLength" : 7u8, }
        }
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(
            ::core::concat!(
                ::core::module_path!(), "::", ::core::stringify!(MicroString)
            ),
        )
    }
}
#[cfg(feature = "const-default")]
impl const_default::ConstDefault for MicroString {
    const DEFAULT: Self = MicroString::EMPTY;
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
enum MicroStringLen {
    _0 = 0u8,
    _1 = 1u8,
    _2 = 2u8,
    _3 = 3u8,
    _4 = 4u8,
    _5 = 5u8,
    _6 = 6u8,
    _7 = 7u8,
}
impl MicroStringLen {
    const fn from_usize(u: usize) -> Option<Self> {
        const U8_MAX: usize = u8::MAX as _;
        if u > U8_MAX {
            return None;
        }
        Self::from_u8(u as u8)
    }
    const fn from_u8(u: u8) -> Option<Self> {
        match u {
            0u8 => Some(Self::_0),
            1u8 => Some(Self::_1),
            2u8 => Some(Self::_2),
            3u8 => Some(Self::_3),
            4u8 => Some(Self::_4),
            5u8 => Some(Self::_5),
            6u8 => Some(Self::_6),
            7u8 => Some(Self::_7),
            _ => None,
        }
    }
}
/// A stack-allocated string which can hold up to 15 UTF-8 encoded bytes.
#[derive(Clone, Copy)]
#[repr(C)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
pub struct MilliString {
    len: MilliStringLen,
    bytes: [u8; 15u8 as _],
}
impl MilliString {
    pub const EMPTY: Self = Self::new("").unwrap();
    pub const fn new(s: &str) -> Option<Self> {
        match MilliStringLen::from_usize(s.len()) {
            Some(len) => {
                let mut bytes = [0; 15u8 as _];
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        s.as_ptr(),
                        bytes.as_mut_ptr(),
                        s.len(),
                    )
                }
                Some(Self { len, bytes })
            }
            None => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(
                ::core::slice::from_raw_parts(
                    self.bytes.as_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
    pub const fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            str::from_utf8_unchecked_mut(
                ::core::slice::from_raw_parts_mut(
                    self.bytes.as_mut_ptr(),
                    self.len as u8 as usize,
                ),
            )
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilliStringError;
impl ::core::fmt::Display for MilliStringError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str("expected a string of at most 15 bytes")
    }
}
impl ::core::error::Error for MilliStringError {}
impl ::core::default::Default for MilliString {
    fn default() -> Self {
        Self::EMPTY
    }
}
impl ::core::default::Default for &MilliString {
    fn default() -> Self {
        &MilliString::EMPTY
    }
}
impl ::core::hash::Hash for MilliString {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
impl ::core::convert::AsRef<Self> for MilliString {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl ::core::convert::AsMut<Self> for MilliString {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl ::core::convert::AsRef<str> for MilliString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::convert::AsMut<str> for MilliString {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::convert::AsRef<[u8]> for MilliString {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
impl TryFrom<&str> for MilliString {
    type Error = MilliStringError;
    fn try_from(value: &str) -> Result<Self, MilliStringError> {
        Self::new(value).ok_or(MilliStringError)
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::ffi::OsStr> for MilliString {
    fn as_ref(&self) -> &::std::ffi::OsStr {
        self.as_str().as_ref()
    }
}
#[cfg(feature = "std")]
impl ::core::convert::AsRef<::std::path::Path> for MilliString {
    fn as_ref(&self) -> &::std::path::Path {
        self.as_str().as_ref()
    }
}
impl ::core::borrow::Borrow<str> for MilliString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl ::core::borrow::BorrowMut<str> for MilliString {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl ::core::ops::Deref for MilliString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}
impl ::core::ops::DerefMut for MilliString {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialEq<T> for MilliString {
    fn eq(&self, other: &T) -> bool {
        self.as_str().eq(other.as_ref())
    }
}
impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialOrd<T> for MilliString {
    fn partial_cmp(&self, other: &T) -> Option<::core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}
impl ::core::fmt::Debug for MilliString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::fmt::Display for MilliString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl ::core::str::FromStr for MilliString {
    type Err = MilliStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or(MilliStringError)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MilliString {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MilliString {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MilliString;
            fn expecting(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                f.write_str("a string of at most 15 bytes")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<MilliString, E> {
                MilliString::new(v)
                    .ok_or_else(|| serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ))
            }
        }
        d.deserialize_str(Visitor)
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for MilliString {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(::core::stringify!(MilliString))
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema! {
            { "type" : "string", "maxLength" : 15u8, }
        }
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed(
            ::core::concat!(
                ::core::module_path!(), "::", ::core::stringify!(MilliString)
            ),
        )
    }
}
#[cfg(feature = "const-default")]
impl const_default::ConstDefault for MilliString {
    const DEFAULT: Self = MilliString::EMPTY;
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable)
)]
enum MilliStringLen {
    _0 = 0u8,
    _1 = 1u8,
    _2 = 2u8,
    _3 = 3u8,
    _4 = 4u8,
    _5 = 5u8,
    _6 = 6u8,
    _7 = 7u8,
    _8 = 8u8,
    _9 = 9u8,
    _10 = 10u8,
    _11 = 11u8,
    _12 = 12u8,
    _13 = 13u8,
    _14 = 14u8,
    _15 = 15u8,
}
impl MilliStringLen {
    const fn from_usize(u: usize) -> Option<Self> {
        const U8_MAX: usize = u8::MAX as _;
        if u > U8_MAX {
            return None;
        }
        Self::from_u8(u as u8)
    }
    const fn from_u8(u: u8) -> Option<Self> {
        match u {
            0u8 => Some(Self::_0),
            1u8 => Some(Self::_1),
            2u8 => Some(Self::_2),
            3u8 => Some(Self::_3),
            4u8 => Some(Self::_4),
            5u8 => Some(Self::_5),
            6u8 => Some(Self::_6),
            7u8 => Some(Self::_7),
            8u8 => Some(Self::_8),
            9u8 => Some(Self::_9),
            10u8 => Some(Self::_10),
            11u8 => Some(Self::_11),
            12u8 => Some(Self::_12),
            13u8 => Some(Self::_13),
            14u8 => Some(Self::_14),
            15u8 => Some(Self::_15),
            _ => None,
        }
    }
}
