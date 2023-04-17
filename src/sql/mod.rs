mod query_builder;

pub use query_builder::*;

use std::fmt;

// See https://clickhouse.tech/docs/en/sql-reference/syntax/#syntax-string-literal
pub(crate) fn string(src: &str, dst: impl fmt::Write) -> fmt::Result {
    escape(src, dst, '\'')
}

// See https://clickhouse.tech/docs/en/sql-reference/syntax/#syntax-identifiers
pub(crate) fn identifier(src: &str, dst: impl fmt::Write) -> fmt::Result {
    escape(src, dst, '`')
}

fn escape(src: &str, mut dst: impl fmt::Write, ch: char) -> fmt::Result {
    dst.write_char(ch)?;

    // TODO: escape newlines?
    for (idx, part) in src.split(ch).enumerate() {
        if idx > 0 {
            dst.write_char('\\')?;
            dst.write_char(ch)?;
        }

        for (idx, part) in part.split('\\').enumerate() {
            if idx > 0 {
                dst.write_str("\\\\")?;
            }

            dst.write_str(part)?;
        }
    }

    dst.write_char(ch)
}
