use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
/// Error creating BSN
pub enum Error {
    /// The BSN was invalid
    InvalidBsn,
    TooFewDigits,
    NonNumericValue,
    Failed11Trial,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidBsn => write!(f, "Invalid BSN number"),
            Error::TooFewDigits => write!(f, "Too few digits: a valid BSN has nine digits"),
            Error::NonNumericValue => {
                write!(f, "Non-numeric value: a valid BSN contains only numerals")
            }
            Error::Failed11Trial => write!(f, "Failed 11 Trial"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
/// 9 digits. For this exercise, all 9 are included. In practice, leading zeroes
/// may be omitted, and the format is NNNN. NN. NNN.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    /// Try to create a new BSN. Returns `Err` if the passed string
    /// does not represent a valid BSN
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        let bsn_str = bsn.to_string();

        Bsn::validate(&bsn_str)?;

        Ok(Bsn { inner: bsn_str })
    }

    /// Check whether the passed string represents a valid BSN.
    //  Returns `Err` if the passed string does not represent a valid BSN
    pub fn validate(bsn: &str) -> Result<(), Error> {
        if bsn.len() != 9 {
            return Err(Error::TooFewDigits);
        }

        if bsn.chars().any(|ch| !ch.is_digit(10)) {
            return Err(Error::NonNumericValue);
        }

        // 11 trial checksum: https://nl.wikipedia.org/wiki/Burgerservicenummer#11-proef
        let mut check = 0;
        for (i, &digit) in bsn.as_bytes().iter().enumerate() {
            match digit {
                b'0'..=b'9' => {
                    let multiplier = if i == 8 { -1 } else { 9 - i as i32 };
                    check += ((digit - b'0') as i32) * multiplier;
                }
                _ => {
                    return Err(Error::InvalidBsn);
                }
            }
        }
        if check % 11 != 0 {
            return Err(Error::Failed11Trial);
        }

        Ok(())
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A visitor for deserializing strings into `Bns`
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let validation = Bsn::validate(v);
                if validation.is_err() {
                    Err(E::custom(format!("Invalid bsn: {:?}", validation)))
                } else {
                    Ok(Bsn {
                        inner: v.to_string(),
                    })
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let validation = Bsn::validate(&v);
                if validation.is_err() {
                    Err(E::custom(format!("Invalid bsn: {:?}", validation)))
                } else {
                    Ok(Bsn {
                        inner: v.to_string(),
                    })
                }
            }
        }

        deserializer.deserialize_str(BsnVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_valid_bsn() {
        let bsns = include_str!("../valid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_ok(),
                "BSN {bsn} is valid, but did not pass validation"
            )
        });
    }

    #[test]
    fn test_invalid_bn() {
        let bsns = include_str!("../invalid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_err(),
                "BSN {bsn} invalid, but passed validation"
            )
        });
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}
