use super::Unit;
use serde::{
    de::{self, Deserialize, Deserializer, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};
use std::{fmt, str::FromStr};

struct UnitVisitor;

impl<'de> Visitor<'de> for UnitVisitor {
    type Value = Unit;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Unit string that parses")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Unit::from_str(s).map_err(|_| de::Error::invalid_value(Unexpected::Str(s), &self))
    }
}

//-----------------------------------------------------------------------------
// Deserialize
//-----------------------------------------------------------------------------
impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UnitVisitor)
    }
}

//-----------------------------------------------------------------------------
// Serialize
//-----------------------------------------------------------------------------
impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Unit;
    use std::str::FromStr;

    fn expected_unit() -> Unit {
        Unit::from_str("100cm456{stuff}/g4").unwrap()
    }

    fn validate_unit(expected_unit: &Unit, expected_json: &str) {
        let json =
            serde_json::to_string(&expected_unit).expect("Couldn't convert Unit to JSON String");
        assert_eq!(expected_json, json);
    }

    fn validate_json(expected_json: &str, expected_unit: &Unit) {
        let unit: Unit = serde_json::from_str(expected_json).unwrap();
        assert_eq!(&unit, expected_unit);
    }

    #[test]
    fn validate_serde_json_full_unit() {
        let expected_unit = expected_unit();
        let expected_json = r#""100cm456{stuff}/g4""#;
        validate_unit(&expected_unit, expected_json);
        validate_json(expected_json, &expected_unit)
    }

    #[test]
    fn validate_serde_json_empty_terms() {
        let expected_unit = Unit::new_unity();
        let expected_json = r#""1""#;
        validate_unit(&expected_unit, expected_json);
        validate_json(expected_json, &expected_unit)
    }

    #[test]
    fn validate_deserialize_json_error_empty_unit() {
        let expected_json = r#""""#;
        let unit: Result<Unit, serde_json::Error> = serde_json::from_str(expected_json);
        assert!(unit.is_err());
    }

    #[test]
    fn validate_deserialize_json_error_bad_unit() {
        let expected_json = r#""!@#$""#;
        let unit: Result<Unit, serde_json::Error> = serde_json::from_str(expected_json);
        assert!(unit.is_err());
    }

    #[allow(box_pointers)]
    #[test]
    fn validate_serde_bincode() {
        let expected_unit = expected_unit();
        let encoded: Vec<u8> = bincode::serialize(&expected_unit).unwrap();
        let decoded: Unit = bincode::deserialize(&encoded).unwrap();

        assert_eq!(expected_unit, decoded);
    }

    #[test]
    fn validate_serde_msg_pack() {
        use rmp_serde::{Deserializer, Serializer};
        use serde::{Deserialize, Serialize};

        let expected_unit = expected_unit();
        let mut buf = Vec::new();
        expected_unit
            .serialize(&mut Serializer::new(&mut buf))
            .unwrap();
        assert_eq!(buf.len(), 19);

        let mut de = Deserializer::new(&buf[..]);
        assert_eq!(expected_unit, Deserialize::deserialize(&mut de).unwrap());
    }
}
