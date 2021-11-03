mod inex;

pub use inex::InExRange;

use std::ops::{Bound, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

#[cfg(feature = "sqlx")]
use sqlx::{
    postgres::{types::PgRange, PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Range<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}

impl<T> From<[Bound<T>; 2]> for Range<T> {
    fn from(v: [Bound<T>; 2]) -> Self {
        let [start, end] = v;
        Self { start, end }
    }
}

impl<T> From<(Bound<T>, Bound<T>)> for Range<T> {
    fn from(v: (Bound<T>, Bound<T>)) -> Self {
        Self {
            start: v.0,
            end: v.1,
        }
    }
}

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(v: std::ops::Range<T>) -> Self {
        Self {
            start: Bound::Included(v.start),
            end: Bound::Excluded(v.end),
        }
    }
}

impl<T> From<RangeFrom<T>> for Range<T> {
    fn from(v: RangeFrom<T>) -> Self {
        Self {
            start: Bound::Included(v.start),
            end: Bound::Unbounded,
        }
    }
}

impl<T> From<RangeInclusive<T>> for Range<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        let (start, end) = v.into_inner();
        Self {
            start: Bound::Included(start),
            end: Bound::Included(end),
        }
    }
}

impl<T> From<RangeTo<T>> for Range<T> {
    fn from(v: RangeTo<T>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(v.end),
        }
    }
}

impl<T> From<RangeToInclusive<T>> for Range<T> {
    fn from(v: RangeToInclusive<T>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Included(v.end),
        }
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        match self.start {
            Bound::Included(ref start) => std::ops::Bound::Included(start),
            Bound::Excluded(ref start) => std::ops::Bound::Excluded(start),
            Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> std::ops::Bound<&T> {
        match self.end {
            Bound::Included(ref start) => std::ops::Bound::Included(start),
            Bound::Excluded(ref start) => std::ops::Bound::Excluded(start),
            Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }
}

#[cfg(feature = "sqlx")]
impl<T> From<Range<T>> for PgRange<T> {
    fn from(range: Range<T>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[cfg(feature = "sqlx")]
impl<'r, T> Decode<'r, Postgres> for Range<T>
where
    T: Type<Postgres> + for<'a> Decode<'a, Postgres>,
{
    #[inline(always)]
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let pgrange = PgRange::decode(value)?;
        Ok(Range {
            start: pgrange.start,
            end: pgrange.end,
        })
    }
}

#[cfg(feature = "sqlx")]
impl<'q, T> Encode<'q, Postgres> for Range<T>
where
    T: Encode<'q, Postgres> + Clone,
{
    #[inline(always)]
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        PgRange::encode_by_ref(
            &PgRange {
                start: self.start.clone(),
                end: self.end.clone(),
            },
            buf,
        )
    }
}

#[cfg(feature = "sqlx")]
impl<T> Type<Postgres> for Range<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        let range = Range::from(2u32..=400u32);
        let result = serde_json::to_string_pretty(&range).unwrap();
        println!("{}", result);
    }
}
