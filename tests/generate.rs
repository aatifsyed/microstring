use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::parse_quote;

#[test]
fn test() {
    let nano = string(&parse_quote!(NanoString), 3);
    let micro = string(&parse_quote!(MicroString), 7);
    let milli = string(&parse_quote!(MilliString), 15);
    let pretty = prettyplease::unparse(
        &syn::parse2(quote! {
            #![cfg_attr(rustfmt, rustfmt::skip)]
            #nano
            #micro
            #milli
        })
        .unwrap(),
    );
    expect_test::expect_file!["../src/generated.rs"].assert_eq(&pretty);
}

fn string(ident: &Ident, n: u8) -> TokenStream {
    let err_ident = Ident::new(&format!("{ident}Error"), Span::call_site());
    let len_ident = Ident::new(&format!("{ident}Len"), Span::call_site());
    let len = len(&len_ident, n);
    let doc = format!(" A stack-allocated string which can hold up to {n} UTF-8 encoded bytes.");
    let err_msg = format!("expected a string of at most {n} bytes");
    let deser_err_msg = format!("a string of at most {n} bytes");
    quote! {
        #[doc = #doc]
        #[derive(Clone, Copy)]
        #[repr(C)]
        #[cfg_attr(feature = "zerocopy", derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable))]
        pub struct #ident {
            len: #len_ident,
            bytes: [u8; #n as _]
        }
        impl #ident {
            const DEFAULT: Self = Self::new("").unwrap();

            pub const fn new(s: &str) -> Option<Self> {
                match #len_ident::from_usize(s.len()) {
                    Some(len) => {
                        let mut bytes = [0; #n as _];
                        unsafe {
                            ::core::ptr::copy_nonoverlapping(s.as_ptr(), bytes.as_mut_ptr(), s.len())
                        }
                        Some(Self { len, bytes })
                    },
                    None => None
                }
            }
            pub const fn as_str(&self) -> &str {
                unsafe {
                    str::from_utf8_unchecked(
                        ::core::slice::from_raw_parts(
                            self.bytes.as_ptr(),
                            self.len as u8 as usize
                        )
                    )
                }
            }
            pub const fn as_mut_str(&mut self) -> &mut str {
                unsafe {
                    str::from_utf8_unchecked_mut(
                        ::core::slice::from_raw_parts_mut(
                            self.bytes.as_mut_ptr(),
                            self.len as u8 as usize
                        )
                    )
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #err_ident;

        impl ::core::fmt::Display for #err_ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(#err_msg)
            }
        }

        impl ::core::error::Error for #err_ident {}

        // default
        // -------

        impl ::core::default::Default for #ident {
            fn default() -> Self {
                Self::DEFAULT
            }
        }
        impl ::core::default::Default for &#ident {
            fn default() -> Self {
                &#ident::DEFAULT
            }
        }

        // hash
        // ----

        impl ::core::hash::Hash for #ident {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state)
            }
        }

        // convert
        // -------

        impl ::core::convert::AsRef<Self> for #ident {
            fn as_ref(&self) -> &Self {
                self
            }
        }
        impl ::core::convert::AsMut<Self> for #ident {
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        impl ::core::convert::AsRef<str> for #ident {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl ::core::convert::AsMut<str> for #ident {
            fn as_mut(&mut self) -> &mut str {
                self.as_mut_str()
            }
        }

        impl ::core::convert::AsRef<[u8]> for #ident {
            fn as_ref(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }

        impl TryFrom<&str> for #ident {
            type Error = #err_ident;
            fn try_from(value: &str) -> Result<Self, #err_ident> {
                Self::new(value).ok_or(#err_ident)
            }
        }

        #[cfg(feature = "std")]
        impl ::core::convert::AsRef<::std::ffi::OsStr> for #ident {
            fn as_ref(&self) -> &::std::ffi::OsStr {
                self.as_str().as_ref()
            }
        }

        #[cfg(feature = "std")]
        impl ::core::convert::AsRef<::std::path::Path> for #ident {
            fn as_ref(&self) -> &::std::path::Path {
                self.as_str().as_ref()
            }
        }

        // borrow
        // ------

        impl ::core::borrow::Borrow<str> for #ident {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }
        impl ::core::borrow::BorrowMut<str> for #ident {
            fn borrow_mut(&mut self) -> &mut str {
                self.as_mut_str()
            }
        }

        // ops
        // ---

        impl ::core::ops::Deref for #ident {
            type Target = str;
            fn deref(&self) -> &str {
                self.as_str()
            }
        }
        impl ::core::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut str {
                self.as_mut_str()
            }
        }

        // cmp
        // ---

        impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialEq<T> for #ident {
            fn eq(&self, other: &T) -> bool {
                self.as_str().eq(other.as_ref())
            }
        }
        impl<T: ::core::convert::AsRef<str>> ::core::cmp::PartialOrd<T> for #ident {
            fn partial_cmp(&self, other: &T) -> Option<::core::cmp::Ordering> {
                self.as_str().partial_cmp(other.as_ref())
            }
        }

        // fmt
        // ---

        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.as_str().fmt(f)
            }
        }
        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.as_str().fmt(f)
            }
        }

        // str
        // ---

        impl ::core::str::FromStr for #ident {
            type Err = #err_ident;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s).ok_or(#err_ident)
            }
        }

        // serde
        // -----

        #[cfg(feature = "serde")]
        impl serde::Serialize for #ident {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(self)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = #ident;
                    fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.write_str(#deser_err_msg)
                    }
                    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<#ident, E> {
                        #ident::new(v).ok_or_else(||serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self))
                    }
                }
                d.deserialize_str(Visitor)
            }
        }

        // schemars
        // --------

        #[cfg(feature = "schemars")]
        impl schemars::JsonSchema for #ident {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(::core::stringify!(#ident))
            }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema! {{
                    "type": "string",
                    "maxLength": #n,
                }}
            }
            fn schema_id() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(::core::concat!(
                    ::core::module_path!(),
                    "::",
                    ::core::stringify!(#ident)
                ))
            }
        }

        // len
        // ---

        #len
    }
}

fn len(ident: &Ident, n: u8) -> TokenStream {
    let (variants, vals) = (0..=n)
        .map(|i| {
            let ident = Ident::new(&format!("_{i}"), Span::call_site());
            let val = Literal::u8_suffixed(i);
            (ident, val)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u8)]
        #[cfg_attr(feature = "zerocopy", derive(zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable))]
        enum #ident {
            #(#variants = #vals),*
        }
        impl #ident {
            const fn from_usize(u: usize) -> Option<Self> {
                const U8_MAX: usize = u8::MAX as _;
                if u > U8_MAX {
                    return None;
                }
                Self::from_u8(u as u8)
            }
            const fn from_u8(u: u8) -> Option<Self> {
                match u {
                    #(
                        #vals => Some(Self::#variants),
                    )*
                    _ => None
                }
            }
        }
    }
}
