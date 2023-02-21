#![allow(non_camel_case_types)]

pub struct UseMixin {
    id:    bool,
    times: bool,
}

pub enum DBType {
    // -> String
    VARCHAR(usize),
    TEXT,

    BOOL,// -> bool

    SMALLINT,// -> i16
    INT,// -> i32
    BIGINT,// -> i64
    
    SERIAL,// -> i32
    BIGSERIAL,// -> i64
    
    REAL,// -> f32
    DOUBLE_PRECISION,// -> f64

    DATE,// -> self::times::Date
    TIME,// -> self::times::Time
    TIMESTAMP,// -> self::times::TimeStamp
    // TIMESTAMPZ,// -> self::times::TimeStampZ
    INTERVAL,// -> self::times::Interval

    // -> impl JSON
    JSON,
    JSONB,
}

pub enum ColumnConstrain {
    CHECK(any),
    NOT_NULL,
    UNIQUE,
    PRIMARY_KEY,
    REFERENCES(any),
    DEFAULT(any),
}
pub enum TableConstrain {
    CHECK(any),
    UNIQUE(any),
    PRIMARY_KEY(any),
    FOREIGN_KEY(any), REFERENCES(any),
}

/// Internal `&str` is automatically generated from input tokens, so you can input literally **any**thing in `()`.
/// ```no_run
/// schema!{
///     User {
///         name: TEXT where DEFAULT("No name"),
///         age:  SMALLINT where DEFAULT(42)
///          // Of course this is valid  ↑↑
///     }
/// }
/// ```
pub struct any(pub &'static str);
