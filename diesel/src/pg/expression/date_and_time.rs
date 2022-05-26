use crate::expression::{Expression, ValidGrouping};
use crate::pg::Pg;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::{
    is_nullable, Date, Nullable, SqlType, Timestamp, Timestamptz, VarChar,
};
pub trait DateTimeLike {}
impl DateTimeLike for Date {}
impl DateTimeLike for Timestamp {}
impl DateTimeLike for Timestamptz {}
impl<T> DateTimeLike for Nullable<T>
where
    T: SqlType<IsNull = is_nullable::NotNull> + DateTimeLike,
{}
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
pub struct AtTimeZone<Ts, Tz> {
    timestamp: Ts,
    timezone: Tz,
}
impl<Ts, Tz> AtTimeZone<Ts, Tz> {
    pub fn new(timestamp: Ts, timezone: Tz) -> Self {
        loop {}
    }
}
impl<Ts, Tz> Expression for AtTimeZone<Ts, Tz>
where
    Ts: Expression,
    Ts::SqlType: DateTimeLike,
    Tz: Expression<SqlType = VarChar>,
{
    type SqlType = Timestamp;
}
impl<Ts, Tz> QueryFragment<Pg> for AtTimeZone<Ts, Tz>
where
    Ts: QueryFragment<Pg>,
    Tz: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(AtTimeZone < Ts, Tz >);
