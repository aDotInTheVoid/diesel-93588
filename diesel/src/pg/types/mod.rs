mod array;
#[doc(hidden)]
pub(in crate::pg) mod date_and_time;
#[doc(hidden)]
pub(in crate::pg) mod floats;
mod integers;
#[cfg(feature = "serde_json")]
mod json;
mod mac_addr;
#[doc(hidden)]
pub(in crate::pg) mod money;
#[cfg(feature = "network-address")]
mod network_address;
mod numeric;
mod primitives;
mod ranges;
mod record;
#[cfg(feature = "uuid")]
mod uuid;
pub mod sql_types {
    use crate::query_builder::QueryId;
    use crate::sql_types::SqlType;
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 26, array_oid = 1018))]
    pub struct Oid;
    #[cfg_attr(
        feature = "chrono",
        doc = " [`chrono::NaiveDateTime`]: chrono::naive::NaiveDateTime"
    )]
    #[cfg_attr(
        not(feature = "chrono"),
        doc = " [`chrono::NaiveDateTime`]: https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDateTime.html"
    )]
    #[cfg_attr(feature = "chrono", doc = " [`chrono::DateTime`]: chrono::DateTime")]
    #[cfg_attr(
        not(feature = "chrono"),
        doc = " [`chrono::DateTime`]: https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html"
    )]
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 1184, array_oid = 1185))]
    pub struct Timestamptz;
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[cfg(feature = "postgres_backend")]
    pub struct Array<ST: 'static>(ST);
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[cfg(feature = "postgres_backend")]
    pub struct Range<ST: 'static>(ST);
    #[doc(hidden)]
    pub type Int4range = Range<crate::sql_types::Int4>;
    #[doc(hidden)]
    pub type Int8range = Range<crate::sql_types::Int8>;
    #[doc(hidden)]
    pub type Daterange = Range<crate::sql_types::Date>;
    #[doc(hidden)]
    pub type Numrange = Range<crate::sql_types::Numeric>;
    #[doc(hidden)]
    pub type Tsrange = Range<crate::sql_types::Timestamp>;
    #[doc(hidden)]
    pub type Tstzrange = Range<crate::sql_types::Timestamptz>;
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 2249, array_oid = 2287))]
    pub struct Record<ST: 'static>(ST);
    #[cfg(feature = "postgres_backend")]
    pub type SmallSerial = crate::sql_types::SmallInt;
    #[cfg(feature = "postgres_backend")]
    pub type Serial = crate::sql_types::Integer;
    #[cfg(feature = "postgres_backend")]
    pub type BigSerial = crate::sql_types::BigInt;
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 2950, array_oid = 2951))]
    pub struct Uuid;
    #[doc(hidden)]
    pub type Bytea = crate::sql_types::Binary;
    #[doc(hidden)]
    pub type Bpchar = crate::sql_types::VarChar;
    #[cfg_attr(
        feature = "serde_json",
        doc = "[`serde_json::Value`]: serde_json::value::Value"
    )]
    #[cfg_attr(
        not(feature = "serde_json"),
        doc = "[`serde_json::Value`]: https://docs.rs/serde_json/1.0.64/serde_json/value/enum.Value.html"
    )]
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 3802, array_oid = 3807))]
    pub struct Jsonb;
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 790, array_oid = 791))]
    pub struct Money;
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 829, array_oid = 1040))]
    pub struct MacAddr;
    #[doc(hidden)]
    pub type Macaddr = MacAddr;
    #[cfg_attr(feature = "ipnetwork", doc = " [IpNetwork]: ipnetwork::IpNetwork")]
    #[cfg_attr(
        not(feature = "ipnetwork"),
        doc = " [IpNetwork]: https://docs.rs/ipnetwork/*/ipnetwork/enum.IpNetwork.html"
    )]
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 869, array_oid = 1041))]
    pub struct Inet;
    #[cfg_attr(feature = "ipnetwork", doc = " [IpNetwork]: ipnetwork::IpNetwork")]
    #[cfg_attr(
        not(feature = "ipnetwork"),
        doc = " [IpNetwork]: https://docs.rs/ipnetwork/*/ipnetwork/enum.IpNetwork.html"
    )]
    #[cfg(feature = "postgres_backend")]
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[diesel(postgres_type(oid = 650, array_oid = 651))]
    pub struct Cidr;
}
mod ops {
    use super::sql_types::*;
    use crate::sql_types::ops::*;
    use crate::sql_types::{Bigint, Cidr, Inet, Interval};
    impl Add for Timestamptz {
        type Rhs = Interval;
        type Output = Timestamptz;
    }
    impl Sub for Timestamptz {
        type Rhs = Interval;
        type Output = Timestamptz;
    }
    impl Add for Cidr {
        type Rhs = Bigint;
        type Output = Inet;
    }
    impl Add for Inet {
        type Rhs = Bigint;
        type Output = Inet;
    }
    impl Sub for Cidr {
        type Rhs = Bigint;
        type Output = Inet;
    }
    impl Sub for Inet {
        type Rhs = Bigint;
        type Output = Inet;
    }
}
