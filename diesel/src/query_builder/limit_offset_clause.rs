use super::QueryFragment;
use crate::query_builder::QueryId;
#[derive(Debug, Clone, Copy, QueryId)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub struct LimitOffsetClause<Limit, Offset> {
        pub limit_clause: Limit,
        pub offset_clause: Offset,
}
#[allow(missing_debug_implementations)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub struct BoxedLimitOffsetClause<'a, DB> {
        pub limit: Option<Box<dyn QueryFragment<DB> + Send + 'a>>,
        pub offset: Option<Box<dyn QueryFragment<DB> + Send + 'a>>,
}
