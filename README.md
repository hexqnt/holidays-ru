# holidays-ru
[![CI](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml/badge.svg)](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/holidays-ru.svg)](https://crates.io/crates/holidays-ru)
[![docs.rs](https://img.shields.io/docsrs/holidays-ru)](https://docs.rs/holidays-ru)

Russian holidays and working days lookup. Updated annually with the official production calendar.

## Usage

Without dependencies — just year, month, day:

```rust
use holidays_ru;

let result = holidays_ru::flags_ymd(2026, 1, 9).unwrap();
let flags = result.value();

assert!(flags.is_day_off());
assert!(!flags.is_holiday());
assert!(flags.is_transferred());
```

With `chrono` (feature `chrono`) or `time` (feature `time`):

```rust
use chrono::NaiveDate;
use holidays_ru::{flags, Resolved};

let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();

match flags(date) {
    Resolved::Fact(flags) => println!("official: {flags:?}"),
    Resolved::Predict(flags) => println!("prediction: {flags:?}"),
}
```

Bool shortcuts:

```rust
holidays_ru::is_day_off_ymd(2026, 1, 9).unwrap().value();   // true
holidays_ru::is_holiday_ymd(2026, 1, 1).unwrap().value();    // true
holidays_ru::is_short_day_ymd(2026, 11, 3).unwrap().value(); // true
```

## Supported years

- **2000–2026** — official production calendar data, returned as `Resolved::Fact`.
- **1900–9998 outside the official-data range** — deterministic prediction based on the Labor Code, returned as `Resolved::Predict`.

Prediction is deterministic and conservative. It is not an official production calendar.
Official transfers are defined by yearly government decrees and may differ from the prediction.

## Features

| Feature  | Enables                                          |
|----------|--------------------------------------------------|
| `chrono` | `flags(date)` for `chrono::NaiveDate`            |
| `time`   | `flags(date)` for `time::Date`                   |
| `serde`  | `Serialize` / `Deserialize` for `DayFlags`, `Resolved<T>` |

No features enabled — use `flags_ymd(year, month, day)`.
