# holidays-ru
[![CI](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml/badge.svg)](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/holidays-ru.svg)](https://crates.io/crates/holidays-ru)
[![docs.rs](https://img.shields.io/docsrs/holidays-ru)](https://docs.rs/holidays-ru)

Russian holidays and working days lookup. Updated annually with the official production calendar.

## Usage

Without dependencies — just year, month, day:

```rust
use holidays_ru::{self, Federal};

let result = holidays_ru::flags_ymd::<Federal>(2026, 1, 9).unwrap();
let flags = result.value();

assert!(flags.is_day_off());
assert!(!flags.is_holiday());
assert!(flags.is_transferred());
```

With `chrono` (feature `chrono`) or `time` (feature `time`):

```rust
use chrono::NaiveDate;
use holidays_ru::{Federal, Resolved, flags};

let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();

match flags::<Federal, _>(date).unwrap() {
    Resolved::Fact(flags) => println!("official: {flags:?}"),
    Resolved::Predict(flags) => println!("prediction: {flags:?}"),
}
```

Bool shortcuts:

```rust
holidays_ru::is_day_off_ymd::<holidays_ru::Federal>(2026, 1, 9).unwrap().value();   // true
holidays_ru::is_holiday_ymd::<holidays_ru::Federal>(2026, 1, 1).unwrap().value();    // true
holidays_ru::is_short_day_ymd::<holidays_ru::Federal>(2026, 11, 3).unwrap().value(); // true
```

Full calendar with regional holidays:

```rust
use holidays_ru::regions::Tatarstan;

let flags = holidays_ru::flags_with_region_ymd::<Tatarstan>(2026, 11, 6)
    .unwrap()
    .value();

assert!(flags.is_day_off()); // Tatarstan Constitution Day
```

The type-level [`FederalWithRegion`](https://docs.rs/holidays-ru/latest/holidays_ru/struct.FederalWithRegion.html)
can be used when a full regional calendar is needed by generic and range APIs:

```rust
use holidays_ru::{FederalWithRegion, WorkWeek, regions};

type Tatarstan = FederalWithRegion<regions::Tatarstan>;

let Some(minutes) = holidays_ru::working_minutes_between_ymd::<Tatarstan>(
    2026,
    11,
    6,
    2026,
    11,
    7,
    WorkWeek::FortyHours,
) else {
    return;
};
let minutes = minutes.value();

assert_eq!(minutes, 0);
```

Date ranges use a half-open interval `[start, end)`:

```rust
use holidays_ru::WorkWeek;

let days_off = holidays_ru::non_working_days_between_ymd::<holidays_ru::Federal>(2026, 1, 1, 2027, 1, 1)
    .unwrap()
    .value();
let minutes = holidays_ru::working_minutes_between_ymd::<holidays_ru::Federal>(
    2026,
    1,
    1,
    2027,
    1,
    1,
    WorkWeek::FortyHours,
)
.unwrap()
.value();

assert_eq!(days_off, 118);
assert_eq!(minutes, 1972 * 60);
```

## Supported years

### Federal calendar (`Federal`)

- **1993–2026** — official production calendar data, returned as `Resolved::Fact`.
- **1900–2100 outside the official-data range** — deterministic prediction based on the Labor Code, returned as `Resolved::Predict`.

### Regional calendars (`calendar::regions::*`)

Overlay calendars with non-working holidays specific to subjects of the Russian Federation
(e.g. `Tatarstan`, `Bashkortostan`, `Crimea`). Each region has its own fact range
and falls back to a fixed-date prediction outside it. Regional calendars must be
combined with the federal calendar for a complete picture.

Prediction is deterministic and conservative. It is not an official production calendar.
Official transfers are defined by yearly government decrees and may differ from the prediction.

## Features

| Feature  | Enables                                                   |
| -------- | --------------------------------------------------------- |
| `chrono` | `flags::<Federal, _>(date)` for `chrono::NaiveDate`       |
| `time`   | `flags::<Federal, _>(date)` for `time::Date`              |
| `serde`  | `Serialize` / `Deserialize` for `DayFlags`, `Resolved<T>` |

No features enabled — use `flags_ymd::<Federal>(year, month, day)`.

## Python bindings

Python bindings live in [`bindings/python`](bindings/python) as a separate
workspace package. PyO3 is not a dependency or feature of the Rust library, so
Rust consumers do not compile or link any Python-specific code.

```python
from datetime import date
from holidays_ru import Calendar, Region

calendar = Calendar(Region.TATARSTAN)
info = calendar.day(date(2026, 11, 6))

assert info.is_day_off
assert info.is_official
```

For local development:

```console
cd bindings/python
python -m pip install -e ".[test]"
python -m pytest
python -m mypy
python -m ruff check .
python -m ruff format --check .
```

## Julia bindings

Julia bindings live in [`bindings/julia`](bindings/julia). The Julia package
uses a native module built with `jlrs`; that module is an isolated Cargo
workspace, so normal Rust and Python builds do not require Julia or `libjulia`.

```julia
using Dates
using HolidaysRu

calendar = Calendar(Tatarstan)
info = day_info(calendar, Date(2026, 11, 6))

@assert is_day_off(info)
@assert is_official(info)
```

See [`bindings/julia/README.md`](bindings/julia/README.md) for native build and
test commands.
