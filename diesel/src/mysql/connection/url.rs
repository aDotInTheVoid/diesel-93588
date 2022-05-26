extern crate percent_encoding;
extern crate url;
use self::percent_encoding::percent_decode;
use self::url::{Host, Url};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use crate::result::{ConnectionError, ConnectionResult};
use mysqlclient_sys::mysql_ssl_mode;
bitflags::bitflags! {
    pub struct CapabilityFlags : u32 { const CLIENT_LONG_PASSWORD = 0x00000001; const
    CLIENT_FOUND_ROWS = 0x00000002; const CLIENT_LONG_FLAG = 0x00000004; const
    CLIENT_CONNECT_WITH_DB = 0x00000008; const CLIENT_NO_SCHEMA = 0x00000010; const
    CLIENT_COMPRESS = 0x00000020; const CLIENT_ODBC = 0x00000040; const
    CLIENT_LOCAL_FILES = 0x00000080; const CLIENT_IGNORE_SPACE = 0x00000100; const
    CLIENT_PROTOCOL_41 = 0x00000200; const CLIENT_INTERACTIVE = 0x00000400; const
    CLIENT_SSL = 0x00000800; const CLIENT_IGNORE_SIGPIPE = 0x00001000; const
    CLIENT_TRANSACTIONS = 0x00002000; const CLIENT_RESERVED = 0x00004000; const
    CLIENT_SECURE_CONNECTION = 0x00008000; const CLIENT_MULTI_STATEMENTS = 0x00010000;
    const CLIENT_MULTI_RESULTS = 0x00020000; const CLIENT_PS_MULTI_RESULTS = 0x00040000;
    const CLIENT_PLUGIN_AUTH = 0x00080000; const CLIENT_CONNECT_ATTRS = 0x00100000; const
    CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA = 0x00200000; const
    CLIENT_CAN_HANDLE_EXPIRED_PASSWORDS = 0x00400000; const CLIENT_SESSION_TRACK =
    0x00800000; const CLIENT_DEPRECATE_EOF = 0x01000000; }
}
pub(super) struct ConnectionOptions {
    host: Option<CString>,
    user: CString,
    password: Option<CString>,
    database: Option<CString>,
    port: Option<u16>,
    unix_socket: Option<CString>,
    client_flags: CapabilityFlags,
    ssl_mode: Option<mysql_ssl_mode>,
    ssl_ca: Option<CString>,
}
impl ConnectionOptions {
    pub(super) fn parse(database_url: &str) -> ConnectionResult<Self> {
        loop {}
    }
    pub(super) fn host(&self) -> Option<&CStr> {
        loop {}
    }
    pub(super) fn user(&self) -> &CStr {
        loop {}
    }
    pub(super) fn password(&self) -> Option<&CStr> {
        loop {}
    }
    pub(super) fn database(&self) -> Option<&CStr> {
        loop {}
    }
    pub(super) fn port(&self) -> Option<u16> {
        loop {}
    }
    pub(super) fn unix_socket(&self) -> Option<&CStr> {
        loop {}
    }
    pub(super) fn ssl_ca(&self) -> Option<&CStr> {
        loop {}
    }
    pub(super) fn client_flags(&self) -> CapabilityFlags {
        loop {}
    }
    pub(super) fn ssl_mode(&self) -> Option<mysql_ssl_mode> {
        loop {}
    }
}
fn decode_into_cstring(s: &str) -> ConnectionResult<CString> {
    loop {}
}
fn connection_url_error() -> ConnectionError {
    loop {}
}
#[test]
fn urls_with_schemes_other_than_mysql_are_errors() {
    loop {}
}
#[test]
fn urls_must_have_zero_or_one_path_segments() {
    loop {}
}
#[test]
fn first_path_segment_is_treated_as_database() {
    loop {}
}
#[test]
fn userinfo_should_be_percent_decode() {
    loop {}
}
#[test]
fn ipv6_host_not_wrapped_in_brackets() {
    loop {}
}
#[test]
fn unix_socket_tests() {
    loop {}
}
#[test]
fn ssl_ca_tests() {
    loop {}
}
#[test]
fn ssl_mode() {
    loop {}
}
