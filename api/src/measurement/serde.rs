use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::fmt;
use super::Measurement;

//-----------------------------------------------------------------------------
// Deserialize
//-----------------------------------------------------------------------------
impl<'de> Deserialize<'de> for Measurement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Value, Unit };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`value` `or `unit`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "value" => Ok(Field::Value),
                            "unit" => Ok(Field::Unit),
                            _ => Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MeasurementVisitor;

        impl<'de> Visitor<'de> for MeasurementVisitor {
            type Value = Measurement;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Measurement")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Measurement, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let unit = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Measurement { value, unit })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Measurement, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value = None;
                let mut unit = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Unit => {
                            if unit.is_some() {
                                return Err(de::Error::duplicate_field("unit"));
                            }
                            unit = Some(map.next_value()?);
                        }
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                let unit = unit.ok_or_else(|| de::Error::missing_field("unit"))?;
                Ok(Measurement { value, unit })
            }
        }

        const FIELDS: &'static [&'static str] = &["value", "unit"];

        deserializer.deserialize_struct("Measurement", FIELDS, MeasurementVisitor)
    }
}

//-----------------------------------------------------------------------------
// Serialize
//-----------------------------------------------------------------------------
impl Serialize for Measurement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // There are 2 fields in the struct.
        let mut state = serializer.serialize_struct("Measurement", 2)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("unit", &self.unit.to_string())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::Measurement;
    use serde_json;

    fn expected_measurement() -> Measurement {
        Measurement::new(432.1, "100cm456{stuff}/g4").unwrap()
    }

    fn validate_measurement(expected_measurement: &Measurement, expected_json: &str) {
        let json = serde_json::to_string(&expected_measurement).expect("Couldn't convert Measurement to JSON String");
        assert_eq!(expected_json, json);
    }

    fn validate_json(expected_json: &str, expected_measurement: &Measurement) {
        let measurement: Measurement = serde_json::from_str(expected_json).unwrap();
        assert_eq!(&measurement, expected_measurement);
    }

    #[test]
    fn validate_serde_json_full_unit() {
        let expected_measurement = expected_measurement();
        let expected_json = r#"{"value":432.1,"unit":"100cm456{stuff}/g4"}"#;
        validate_measurement(&expected_measurement, expected_json);
        validate_json(expected_json, &expected_measurement)
    }

    #[test]
    fn validate_serde_json_empty_unit_terms() {
        let expected_measurement = Measurement::new(2.0, "1").unwrap();
        let expected_json = r#"{"value":2.0,"unit":"1"}"#;
        validate_measurement(&expected_measurement, expected_json);
        validate_json(expected_json, &expected_measurement)
    }

    #[test]
    fn validate_deserialize_json_integer_value() {
        let expected_json = r#"{"value":2,"unit":"m"}"#;
        let expected_measurement = Measurement::new(2.0, "m").unwrap();
        validate_json(expected_json, &expected_measurement)
    }

    #[test]
    fn validate_deserialize_json_errors() {
        let expected_json = r#"{"value":2.0,"unit":""}"#;
        let measurement: Result<Measurement, serde_json::Error> = serde_json::from_str(expected_json);
        assert!(measurement.is_err());

        let expected_json = r#"{"value":"adsf","unit":"m"}"#;
        let measurement: Result<Measurement, serde_json::Error> = serde_json::from_str(expected_json);
        assert!(measurement.is_err());
    }

    #[allow(box_pointers)]
    #[test]
    fn validate_bincode_serde() {
        let expected_measurement = Measurement::new(123.4, "100cm456{stuff}/g4").unwrap();
        let encoded: Vec<u8> = bincode::serialize(&expected_measurement).unwrap();
        let decoded: Measurement = bincode::deserialize(&encoded).unwrap();

        assert_eq!(expected_measurement, decoded);
    }

    #[test]
    fn validate_message_pack_serde() {
        use rmp_serde::{Deserializer, Serializer};
        use serde::{Deserialize, Serialize};

        let expected_measurement = Measurement::new(123.4, "100cm456{stuff}/g4").unwrap();
        let mut buf = Vec::new();
        expected_measurement.serialize(&mut Serializer::new(&mut buf)).unwrap();

        assert_eq!(buf.len(), 29);

        let mut de = Deserializer::new(&buf[..]);
        assert_eq!(expected_measurement, Deserialize::deserialize(&mut de).unwrap());
    }
}
