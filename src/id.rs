use crate::encoding::{parse_base32, stringify_base32};
use crate::{Error, Type};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

/// Type of the underlying data stored in an `Id`.
pub type IdBytes = [u8; 8];

/// A typed 64-bit identifier.
///
/// ```
/// use souvenir::{Type, Id};
///
/// struct User {
///     // fields omitted
/// }
///
/// impl Type for User {
///     const PREFIX: &'static str = "user";
/// }
///
/// let id: Id<User> = Id::random();
/// println!("{}", id);
///
/// let id2: Id<User> = Id::parse("user_4n3y65asan4bj").unwrap();
/// assert_eq!(id2.to_string(), "user_4n3y65asan4bj");
/// ```
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "diesel",
    derive(::diesel::AsExpression, ::diesel::FromSqlRow)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type = ::diesel::sql_types::Int8))]
pub struct Id<T: Type + ?Sized> {
    marker: PhantomData<T>,
    value: IdBytes,
}

impl<T: Type + ?Sized> Id<T> {
    /// Create a new `Id<T>` with the following underlying value.
    pub fn new(value: [u8; 8]) -> Self {
        Self {
            marker: PhantomData,
            value,
        }
    }

    /// Get the data value of the identifier.
    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.value
    }

    /// Get the data value of the identifier.
    pub fn to_bytes(self) -> [u8; 8] {
        self.value
    }

    /// Get the data value of the identifier as a `u64`.
    pub fn to_u64(self) -> u64 {
        u64::from_be_bytes(self.value)
    }

    /// Get the data value of the identifier as an `i64`.
    pub fn to_i64(self) -> i64 {
        i64::from_be_bytes(self.value)
    }

    /// Test to see if the provided string is a valid `Id<T>`.
    pub fn test(value: &str) -> bool {
        Self::parse(value).is_ok()
    }

    /// Attempt to parse the provided string into an `Id<T>`.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let (prefix, value) = value.split_once('_').ok_or(Error::InvalidData)?;

        if prefix != T::PREFIX {
            return Err(Error::PrefixMismatch {
                expected: T::PREFIX,
                actual: String::from(prefix),
            });
        }

        Ok(Self::new(parse_base32(value)?))
    }

    /// Get the prefix of this identifier
    pub const fn prefix(self) -> &'static str {
        T::PREFIX
    }
}

impl<T: Type + ?Sized> Copy for Id<T> {}

impl<T: Type + ?Sized> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Type + ?Sized> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: Type + ?Sized> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}_{}",
            T::PREFIX,
            stringify_base32(self.value).expect("id value to stringify correctly")
        )
    }
}

impl<T: Type + ?Sized> FromStr for Id<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl<T: Type + ?Sized> From<Id<T>> for u64 {
    fn from(value: Id<T>) -> Self {
        value.to_u64()
    }
}

impl<T: Type + ?Sized> From<Id<T>> for i64 {
    fn from(value: Id<T>) -> Self {
        value.to_i64()
    }
}

impl<T: Type + ?Sized> From<Id<T>> for IdBytes {
    fn from(value: Id<T>) -> Self {
        value.to_bytes()
    }
}

impl<T: Type + ?Sized> From<u64> for Id<T> {
    fn from(value: u64) -> Self {
        Self::new(value.to_be_bytes())
    }
}

impl<T: Type + ?Sized> From<i64> for Id<T> {
    fn from(value: i64) -> Self {
        Self::new(value.to_be_bytes())
    }
}

impl<T: Type + ?Sized> From<IdBytes> for Id<T> {
    fn from(value: IdBytes) -> Self {
        Self::new(value)
    }
}
