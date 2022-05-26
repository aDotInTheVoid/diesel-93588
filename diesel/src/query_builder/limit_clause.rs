simple_clause!(
    #[doc = " A query node indicating the absence of a limit clause"] #[doc = ""] #[doc =
    " This type is only relevant for implementing custom backends"] #[cfg_attr(feature =
    "i-implement-a-third-party-backend-and-opt-into-breaking-changes", cfg(feature =
    "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] NoLimitClause,
    #[doc = " A query node representing a limit clause"] #[doc = ""] #[doc =
    " This type is only relevant for implementing custom backends"] #[cfg_attr(feature =
    "i-implement-a-third-party-backend-and-opt-into-breaking-changes", cfg(feature =
    "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] LimitClause,
    " LIMIT "
);
