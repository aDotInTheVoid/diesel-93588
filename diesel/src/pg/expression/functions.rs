use super::expression_methods::InetOrCidr;
use crate::expression::functions::sql_function;
use crate::sql_types::*;
sql_function! {
    #[doc = " Creates an abbreviated display format as text."] #[cfg(feature =
    "postgres_backend")] fn abbrev < T : InetOrCidr + SingleValue > (addr : T) -> Text;
}
sql_function! {
    #[doc = " Computes the broadcast address for the address's network."] #[cfg(feature =
    "postgres_backend")] fn broadcast < T : InetOrCidr + SingleValue > (addr : T) ->
    Inet;
}
sql_function! {
    #[doc = " Returns the address's family: 4 for IPv4, 6 for IPv6."] #[cfg(feature =
    "postgres_backend")] fn family < T : InetOrCidr + SingleValue > (addr : T) ->
    Integer;
}
sql_function! {
    #[doc = " Returns the IP address as text, ignoring the netmask."] #[cfg(feature =
    "postgres_backend")] fn host < T : InetOrCidr + SingleValue > (addr : T) -> Text;
}
sql_function! {
    #[doc = " Computes the host mask for the address's network."] #[cfg(feature =
    "postgres_backend")] fn hostmask < T : InetOrCidr + SingleValue > (addr : T) -> Inet;
}
sql_function! {
    #[doc = " Computes the smallest network that includes both of the given networks."]
    #[cfg(feature = "postgres_backend")] fn inet_merge < T : InetOrCidr + SingleValue, U
    : InetOrCidr + SingleValue > (a : T, b : U) -> Cidr;
}
sql_function! {
    #[doc = " Tests whether the addresses belong to the same IP family."] #[cfg(feature =
    "postgres_backend")] fn inet_same_family < T : InetOrCidr + SingleValue, U :
    InetOrCidr + SingleValue > (a : T, b : U) -> Bool;
}
sql_function! {
    #[doc = " Returns the netmask length in bits."] #[cfg(feature = "postgres_backend")]
    fn masklen < T : InetOrCidr + SingleValue > (addr : T) -> Integer;
}
sql_function! {
    #[doc = " Computes the network mask for the address's network."] #[cfg(feature =
    "postgres_backend")] fn netmask < T : InetOrCidr + SingleValue > (addr : T) -> Inet;
}
sql_function! {
    #[doc =
    " Returns the network part of the address, zeroing out whatever is to the right of the"]
    #[doc = " netmask. (This is equivalent to casting the value to cidr.)"] #[cfg(feature
    = "postgres_backend")] fn network < T : InetOrCidr + SingleValue > (addr : T) ->
    Cidr;
}
sql_function! {
    #[doc = " Sets the netmask length for an inet or cidr value."] #[doc =
    " For inet, the address part does not changes. For cidr, address bits to the right of the new"]
    #[doc = " netmask are set to zero."] #[cfg(feature = "postgres_backend")] fn
    set_masklen < T : InetOrCidr + SingleValue > (addr : T, len : Integer) -> T;
}
