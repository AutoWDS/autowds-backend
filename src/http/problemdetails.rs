#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::unused_self
)]

/// [ProblemDetails RFC-7807](https://datatracker.ietf.org/doc/html/rfc7807)
///
use std::collections::BTreeMap;

use actix_web::http::StatusCode;
use serde::{ser::SerializeMap, Serialize};
use serde_json::Value;

/// Representation of a Problem error to return to the client.
#[allow(dead_code)] // These fields are used by the various features.
#[derive(Clone, Debug)]
pub struct Problem {
    /// The status code of the problem.
    pub status_code: StatusCode,
    /// The actual body of the problem.
    pub body: BTreeMap<String, Value>,
}

/// Create a new `Problem` response to send to the client.

#[must_use]
pub fn new<S>(status_code: S) -> Problem
where
    S: Into<StatusCode>,
{
    Problem {
        status_code: status_code.into(),
        body: BTreeMap::new(),
    }
}

impl Problem {
    /// Specify the "type" to use for the problem.
    ///
    /// # Parameters
    /// - `value` - The value to use for the "type"
    #[must_use]
    pub fn with_type<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.with_value("type", value.into())
    }

    /// Specify the "title" to use for the problem.
    ///
    /// # Parameters
    /// - `value` - The value to use for the "title"
    #[must_use]
    pub fn with_title<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.with_value("title", value.into())
    }

    /// Specify the "detail" to use for the problem.
    ///
    /// # Parameters
    /// - `value` - The value to use for the "detail"
    #[must_use]
    pub fn with_detail<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.with_value("detail", value.into())
    }

    /// Specify the "instance" to use for the problem.
    ///
    /// # Parameters
    /// - `value` - The value to use for the "instance"
    #[must_use]
    pub fn with_instance<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.with_value("instance", value.into())
    }

    /// Specify an arbitrary value to include in the problem.
    ///
    /// # Parameters
    /// - `key` - The key for the value.
    /// - `value` - The value itself.
    #[must_use]
    pub fn with_value<V>(mut self, key: &str, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.body.insert(key.to_owned(), value.into());

        self
    }
}

impl<S> From<S> for Problem
where
    S: Into<StatusCode>,
{
    fn from(status_code: S) -> Self {
        new(status_code.into())
    }
}

impl Serialize for Problem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.body.len() + 1))?;
        map.serialize_entry("status", &self.status_code.as_u16())?;
        for (k, v) in &self.body {
            map.serialize_entry(k.as_str(), &v)?;
        }
        map.end()
    }
}
