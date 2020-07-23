use std::{fmt, ops};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Secret<T>(T);

impl<T> From<T> for Secret<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Secret<T> {
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for Secret<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Secret<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<hidden>")
    }
}

impl<T> fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<hidden>")
    }
}

#[cfg(feature = "serde")]
mod serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::Secret;

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Self)
        }
    }

    pub fn insecure_serialize<T: Serialize, S>(x: &Secret<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        x.0.serialize(s)
    }

    pub fn serialize<T, S>(_: &Secret<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        "<hidden>".serialize(s)
    }
}

#[cfg(feature = "serde")]
pub use self::serde::insecure_serialize;
#[cfg(feature = "serde")]
pub use self::serde::serialize;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_insecure_serialize() {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        struct X {
            #[serde(serialize_with = "insecure_serialize")]
            y: Secret<u32>,
            #[serde(serialize_with = "serialize")]
            z: Secret<u32>,
        }
    }
}
