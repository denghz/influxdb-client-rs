use std::fmt::{Debug, Formatter, Write};

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::escape;
use crate::traits::PointSerialize;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl<T: ToString> From<T> for Value {
    default fn from(t: T) -> Self {
        Value::Str(t.to_string())
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Float(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Value {
        Value::Int(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Value {
        Value::Int(v as i64)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Bool(v)
    }
}

impl Value {
    pub fn format(&self) -> String {
        match self {
            Value::Str(s) => {
                format!(
                    "\"{}\"",
                    escape::escape_field_value_string(&s)
                )
            }
            Value::Int(i) => {
                format!("{}", i)
            }
            Value::Float(f) => {
                format!("{}", f)
            }
            Value::Bool(b) => {
                format!("{}", b)
            }
        }
    }
}
#[derive(Clone, PartialEq)]
pub enum Timestamp {
    Str(String),
    Int(i64),
}

impl Debug for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Timestamp::Str(s) => {
                write!(f, "StrTimestamp({})", s)
            }
            Timestamp::Int(n) => {
                let naive = NaiveDateTime::from_timestamp(*n, 0);
                let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                let format_date = datetime.format("%Y-%m-%d %H:%M:%S");
                write!(f, "IntTimestamp({})", format_date)
            }
        }

    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp::Int(0)
    }
}

impl From<&str> for Timestamp {
    fn from(v: &str) -> Timestamp {
        Timestamp::Str(v.to_string())
    }
}

impl From<i64> for Timestamp {
    fn from(v: i64) -> Timestamp {
        Timestamp::Int(v)
    }
}

impl std::string::ToString for Timestamp {
    fn to_string(&self) -> String {
        match self {
            Timestamp::Str(s) => s.to_string(),
            Timestamp::Int(i) => i.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub measurement: String,
    pub timestamp: Option<Timestamp>,
    // tag values are always strings:
    // <https://docs.influxdata.com/influxdb/v1.8/concepts/glossary/#tag-value>
    pub tags: Vec<(String, String)>,
    pub fields: Vec<(String, Value)>,
}

impl Point {
    pub fn new<T: Into<String>>(measurement: T) -> Self {
        Point {
            measurement: measurement.into(),
            tags: Vec::new(),
            fields: Vec::new(),
            timestamp: None,
        }
    }

    pub fn tag<T: Into<String>, V: Into<String>>(mut self, key: T, value: V) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }

    pub fn field<T: Into<String>, V: Into<Value>>(mut self, key: T, value: V) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    pub fn timestamp<T: Into<Timestamp>>(mut self, timestamp: T) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl PointSerialize for Point {
    fn serialize(&self) -> String {
        // format guide: <https://archive.docs.influxdata.com/influxdb/v1.2/write_protocols/line_protocol_reference/>

        let mut builder = String::new();

        // Write measurement
        builder.push_str(&escape::escape_measurement(&self.measurement));

        // Write tags
        if !self.tags.is_empty() {
            builder.push(',');
            let n = self.tags.len();
            for (i, (tag_key, tag_value)) in self.tags.iter().enumerate() {
                builder.push_str(&escape::escape_tag_and_field_keys(tag_key));
                builder.push('=');
                builder.push_str(&escape::escape_tag_and_field_keys(tag_value));
                if i < n - 1 {
                    builder.push(',');
                }
            }
        }

        // Write fields
        if !self.fields.is_empty() {
            builder.push(' ');
            let n = self.fields.len();
            for (i, (field_key, field_value)) in self.fields.iter().enumerate() {
                builder.push_str(&escape::escape_tag_and_field_keys(field_key));
                builder.push('=');
                match field_value {
                    Value::Str(s) => {
                        write!(
                            &mut builder,
                            "\"{}\"",
                            escape::escape_field_value_string(&s)
                        )
                        .unwrap();
                    }
                    Value::Int(i) => {
                        write!(&mut builder, "{}", i).unwrap();
                    }
                    Value::Float(f) => {
                        write!(&mut builder, "{}", f).unwrap();
                    }
                    Value::Bool(b) => {
                        write!(&mut builder, "{}", b).unwrap();
                    }
                };
                if i < n - 1 {
                    builder.push(',');
                }
            }
        }

        builder
    }

    fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String {
        match timestamp {
            Some(timestamp) => format!("{} {}", self.serialize(), timestamp.to_string()),
            None => format!(
                "{} {}",
                self.serialize(),
                self.timestamp
                    .clone()
                    .unwrap_or_else(|| Timestamp::from(0))
                    .to_string()
            ),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InfluxError {
    #[error("Network error: {0:?}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Clone)]
pub enum TimestampOptions {
    None,
    Use(Timestamp),
    FromPoint,
}

#[derive(Debug, Clone, Copy)]
pub enum Precision {
    NS,
    US,
    MS,
    S,
}

impl Precision {
    pub fn to_string(&self) -> &str {
        match self {
            Precision::NS => "ns",
            Precision::US => "us",
            Precision::MS => "ms",
            Precision::S => "s",
        }
    }
}
