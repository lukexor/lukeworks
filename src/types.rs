//! Custom library types.

use serde::Deserialize;

/// Image attributes.
#[derive(Deserialize, Debug, Clone)]
pub struct ImgAttrs {
    pub src: String,
    pub title: String,
    pub alt: String,
}

/// ISO-8601 formatted date [str].
#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct DateStr(pub String);

impl std::fmt::Display for DateStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// URL [str].
#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Url(pub String);

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
