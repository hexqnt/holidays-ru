/// Результат запроса к календарю: факт или прогноз.
///
/// # Семантика
///
/// - `Fact(T)` — данные присутствуют в официальной таблице производственного календаря.
/// - `Predict(T)` — официальной таблицы нет, результат получен алгоритмическим прогнозом.
///
/// # Примеры
///
/// ```rust
/// use holidays_ru::Resolved;
///
/// let r: Resolved<i32> = Resolved::Fact(42);
/// assert!(r.is_fact());
/// assert_eq!(r.value(), 42);
///
/// let mapped = r.map(|v| v * 2);
/// assert_eq!(mapped, Resolved::Fact(84));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resolved<T> {
    /// Результат основан на официальных данных производственного календаря.
    Fact(T),

    /// Результат получен алгоритмическим прогнозом.
    Predict(T),
}

impl<T> Resolved<T> {
    /// Извлекает значение, отбрасывая информацию о происхождении.
    ///
    /// ```rust
    /// use holidays_ru::Resolved;
    ///
    /// assert_eq!(Resolved::Fact(10).value(), 10);
    /// assert_eq!(Resolved::Predict(20).value(), 20);
    /// ```
    #[inline]
    pub fn value(self) -> T {
        match self {
            Self::Fact(value) | Self::Predict(value) => value,
        }
    }

    /// Возвращает `true`, если результат основан на официальных данных.
    #[inline]
    pub const fn is_fact(&self) -> bool {
        matches!(self, Self::Fact(_))
    }

    /// Возвращает `true`, если результат получен прогнозом.
    #[inline]
    pub const fn is_predict(&self) -> bool {
        matches!(self, Self::Predict(_))
    }

    /// Применяет функцию к внутреннему значению, сохраняя обёртку `Fact`/`Predict`.
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Resolved<U> {
        match self {
            Self::Fact(value) => Resolved::Fact(f(value)),
            Self::Predict(value) => Resolved::Predict(f(value)),
        }
    }

    /// Возвращает внутреннее значение как ссылку.
    #[inline]
    pub const fn as_ref(&self) -> Resolved<&T> {
        match self {
            Self::Fact(value) => Resolved::Fact(value),
            Self::Predict(value) => Resolved::Predict(value),
        }
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Resolved<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("Resolved", 2)?;
        match self {
            Self::Fact(value) => {
                s.serialize_field("kind", "Fact")?;
                s.serialize_field("value", value)?;
            }
            Self::Predict(value) => {
                s.serialize_field("kind", "Predict")?;
                s.serialize_field("value", value)?;
            }
        }
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Resolved<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Kind,
            Value,
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Resolved<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("struct Resolved")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut kind: Option<String> = None;
                let mut value: Option<T> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Kind => {
                            if kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }

                let kind = kind.ok_or_else(|| serde::de::Error::missing_field("kind"))?;
                let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;

                match kind.as_str() {
                    "Fact" => Ok(Resolved::Fact(value)),
                    "Predict" => Ok(Resolved::Predict(value)),
                    other => Err(serde::de::Error::unknown_variant(
                        other,
                        &["Fact", "Predict"],
                    )),
                }
            }
        }

        deserializer.deserialize_struct(
            "Resolved",
            &["kind", "value"],
            Visitor(std::marker::PhantomData),
        )
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_json_fact_format() {
        let value = Resolved::Fact(42u8);
        let json = serde_json::to_string(&value).unwrap();

        assert_eq!(json, r#"{"kind":"Fact","value":42}"#);
        assert_eq!(serde_json::from_str::<Resolved<u8>>(&json).unwrap(), value);
    }

    #[test]
    fn test_serde_json_predict_format() {
        let value = Resolved::Predict(false);
        let json = serde_json::to_string(&value).unwrap();

        assert_eq!(json, r#"{"kind":"Predict","value":false}"#);
        assert_eq!(
            serde_json::from_str::<Resolved<bool>>(&json).unwrap(),
            value
        );
    }
}
