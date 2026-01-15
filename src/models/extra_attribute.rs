//! Defines the behavior for adding custom, not-to-spec, attributes to SCIM
//! models.

use serde::{Deserialize, Serialize};

use crate::models::user::{Email, Entitlement, Im, PhoneNumber, Photo, Role, X509Certificate};

/// Enables extra attributes to be added to SCIM types even if they don't fully
/// match the spec.
#[derive(Debug, Serialize, Deserialize)]
pub enum ExtraAttributeValue {
    PrimitiveString(String),
    MultiValued(Vec<DefaultMultiValue>),
}

/// The default schema used for [multi-valued attributes]
///
/// [multi-valued attributes]: https://datatracker.ietf.org/doc/html/rfc7643#section-2.4
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DefaultMultiValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum TypeConversionError {
    #[error("invalid schema for field: {field_name}")]
    IncorrectSchema { field_name: String },
}

impl From<DefaultMultiValue> for Email {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<Email> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "emails".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for PhoneNumber {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<PhoneNumber> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "phone_numbers".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for Im {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<Im> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "ims".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for Photo {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<Photo> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "photos".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for Entitlement {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<Entitlement> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "entitlements".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for Role {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<Role> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "roles".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<DefaultMultiValue> for X509Certificate {
    fn from(value: DefaultMultiValue) -> Self {
        let DefaultMultiValue {
            value,
            display,
            r#type,
            primary,
            r#ref: _,
        } = value;

        Self {
            value,
            display,
            r#type,
            primary,
        }
    }
}

impl TryFrom<ExtraAttributeValue> for Vec<X509Certificate> {
    type Error = TypeConversionError;

    fn try_from(value: ExtraAttributeValue) -> Result<Self, Self::Error> {
        match value {
            ExtraAttributeValue::PrimitiveString(_) => Err(Self::Error::IncorrectSchema {
                field_name: "x509_certificates".to_owned(),
            }),
            ExtraAttributeValue::MultiValued(default_multi_values) => {
                Ok(default_multi_values.into_iter().map(Into::into).collect())
            }
        }
    }
}
