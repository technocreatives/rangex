#[cfg(feature = "sqlx")]
use std::ops::Bound;

#[cfg(feature = "sqlx")]
use sqlx::{
    postgres::{types::PgRange, PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct InExRange<T> {
    pub inclusive_start: T,
    pub exclusive_end: T,
}

impl<T> From<InExRange<T>> for std::ops::Range<T> {
    fn from(x: InExRange<T>) -> Self {
        x.inclusive_start..x.exclusive_end
    }
}

#[cfg(feature = "sqlx")]
impl<'r, T> Decode<'r, Postgres> for InExRange<T>
where
    T: Type<Postgres> + for<'a> Decode<'a, Postgres>,
{
    #[inline(always)]
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let pgrange = PgRange::decode(value)?;

        let (inclusive_start, exclusive_end) = match (pgrange.start, pgrange.end) {
            (Bound::Included(a), Bound::Excluded(b)) => (a, b),
            (a, b) => todo!(),
        };

        Ok(InExRange {
            inclusive_start,
            exclusive_end,
        })
    }
}

#[cfg(feature = "sqlx")]
impl<'q, T> Encode<'q, Postgres> for InExRange<T>
where
    T: Encode<'q, Postgres> + Clone,
{
    #[inline(always)]
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        PgRange::encode_by_ref(
            &PgRange {
                start: Bound::Included(self.inclusive_start.clone()),
                end: Bound::Excluded(self.exclusive_end.clone()),
            },
            buf,
        )
    }
}

#[cfg(feature = "sqlx")]
impl<T> Type<Postgres> for InExRange<T>
where
    PgRange<T>: sqlx::Type<Postgres>,
{
    #[inline(always)]
    fn type_info() -> PgTypeInfo {
        PgRange::<T>::type_info()
    }

    #[inline(always)]
    fn compatible(ty: &PgTypeInfo) -> bool {
        PgRange::<T>::compatible(ty)
    }
}
