use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{Decode, Postgres};
#[cfg(feature = "sqlx")]
use std::error::Error;
use std::fmt::{Debug, Display};

pub struct Secret<T>(T)
where
    T: Clone;

impl<T> Serialize for Secret<T>
where
    T: Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Secret<T>
where
    T: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Secret<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Secret(T::deserialize(deserializer)?))
    }
}

impl<T> Clone for Secret<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Secret<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> Display for Secret<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[hidden]")
    }
}

impl<T> Debug for Secret<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[hidden]")
    }
}

#[cfg(feature = "sqlx")]
impl<T> sqlx::Type<Postgres> for Secret<T>
where
    T: Clone + sqlx::Type<Postgres>,
{
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <T as sqlx::Type<Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<T> sqlx::Decode<'_, Postgres> for Secret<T>
where
    T: Clone,
    for<'a> T: sqlx::Type<Postgres> + sqlx::Decode<'a, Postgres>,
{
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <T as Decode<Postgres>>::decode(value)?;
        Ok(Secret::from(value))
    }
}

#[cfg(feature = "sqlx")]
impl<T> sqlx::Encode<'_, Postgres> for Secret<T>
where
    T: Clone,
    for<'a> T: sqlx::Encode<'a, Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <T as sqlx::Encode<Postgres>>::encode(self.as_ref().clone(), buf)
    }
}

impl<T> AsRef<T> for Secret<T>
where
    T: Clone,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Secret<T>
where
    T: Clone,
{
    pub fn expose_secret(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Secret<T>
where
    T: Clone,
{
    fn from(s: T) -> Self {
        Self(s)
    }
}
