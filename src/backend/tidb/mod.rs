pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Mysql query builder.
#[derive(Debug)]
pub struct TiDBQueryBuilder;

pub type tidbQueryBuilder = TiDBQueryBuilder;

impl Default for TiDBQueryBuilder {
    fn default() -> Self {
        Self
    }
}

impl GenericBuilder for TiDBQueryBuilder {}

impl SchemaBuilder for TiDBQueryBuilder {}

impl QuotedBuilder for TiDBQueryBuilder {
    fn quote(&self) -> char {
        '`'
    }
}

impl EscapeBuilder for TiDBQueryBuilder {}
