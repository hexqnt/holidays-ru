# holidays-ru

[🇺🇸 English](./README.md) · [🇷🇺 Русский](./README.ru.md)

[![CI](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml/badge.svg)](https://github.com/hexqnt/holidays-ru/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/holidays-ru.svg)](https://crates.io/crates/holidays-ru)
[![docs.rs](https://img.shields.io/docsrs/holidays-ru)](https://docs.rs/holidays-ru)

Библиотека для определения праздничных и рабочих дней в России. Ежегодно
обновляется по официальному производственному календарю.

## Использование

Без библиотек для работы с датами — достаточно года, месяца и дня:

```rust
use holidays_ru::{self, Federal};

let result = holidays_ru::flags_ymd::<Federal>(2026, 1, 9).unwrap();
let flags = result.value();

assert!(flags.is_day_off());
assert!(!flags.is_holiday());
assert!(flags.is_transferred());
```

С `chrono` (фича `chrono`) или `time` (фича `time`):

```rust
use chrono::NaiveDate;
use holidays_ru::{Federal, Resolved, flags};

let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();

match flags::<Federal, _>(date).unwrap() {
    Resolved::Fact(flags) => println!("официальные данные: {flags:?}"),
    Resolved::Predict(flags) => println!("прогноз: {flags:?}"),
}
```

Вспомогательные функции, возвращающие `bool`:

```rust
holidays_ru::is_day_off_ymd::<holidays_ru::Federal>(2026, 1, 9).unwrap().value();   // true
holidays_ru::is_holiday_ymd::<holidays_ru::Federal>(2026, 1, 1).unwrap().value();    // true
holidays_ru::is_short_day_ymd::<holidays_ru::Federal>(2026, 11, 3).unwrap().value(); // true
```

Полный календарь с региональными праздниками:

```rust
use holidays_ru::regions::Tatarstan;

let flags = holidays_ru::flags_with_region_ymd::<Tatarstan>(2026, 11, 6)
    .unwrap()
    .value();

assert!(flags.is_day_off()); // День Конституции Татарстана
```

Тип [`FederalWithRegion`](https://docs.rs/holidays-ru/latest/holidays_ru/struct.FederalWithRegion.html)
позволяет использовать полный региональный календарь в обобщённых функциях и
функциях для диапазонов дат:

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

Диапазоны дат задаются полуинтервалом `[start, end)`:

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

## Поддерживаемые годы

### Федеральный календарь (`Federal`)

- **1993–2027** — данные официального производственного календаря, возвращаемые
  как `Resolved::Fact`.
- **1900–2100 за пределами диапазона официальных данных** — детерминированный
  прогноз на основе Трудового кодекса, возвращаемый как `Resolved::Predict`.

### Региональные календари (`regions::*`)

Дополнительные календари нерабочих праздничных дней субъектов Российской
Федерации (например, `Tatarstan`, `Bashkortostan`, `Crimea`). Для каждого региона
задан свой диапазон официальных данных, за пределами которого используется
прогноз по фиксированным датам. Для получения полного календаря региональный
календарь нужно объединить с федеральным.

Прогноз детерминирован и консервативен, но не является официальным
производственным календарём. Переносы, ежегодно устанавливаемые постановлениями
Правительства, могут отличаться от прогноза.

## Фичи

| Фича     | Возможности                                                |
| -------- | ---------------------------------------------------------- |
| `chrono` | `flags::<Federal, _>(date)` для `chrono::NaiveDate`        |
| `time`   | `flags::<Federal, _>(date)` для `time::Date`               |
| `serde`  | `Serialize` / `Deserialize` для `DayFlags` и `Resolved<T>` |

Если фичи не включены, используйте `flags_ymd::<Federal>(year, month, day)`.

## Биндинги для Python

Биндинги для Python находятся в отдельном пакете рабочего пространства
[`bindings/python`](bindings/python), поэтому при использовании Rust-библиотеки
код PyO3 не собирается и не линкуется.

```python
from datetime import date
from holidays_ru import Calendar, Region

calendar = Calendar(Region.TATARSTAN)
info = calendar.day(date(2026, 11, 6))

assert info.is_day_off
assert info.is_official
```

Для локальной разработки:

```console
cd bindings/python
python -m pip install -e ".[test]"
python -m pytest
python -m mypy
python -m ruff check .
python -m ruff format --check .
```

## Биндинги для Julia

Биндинги для Julia находятся в [`bindings/julia`](bindings/julia). Их нативный
модуль на `jlrs` изолирован в отдельном рабочем пространстве Cargo, поэтому для
сборки Rust- и Python-пакетов не нужны Julia или `libjulia`.

```julia
using Dates
using HolidaysRu

calendar = Calendar(Tatarstan)
info = day_info(calendar, Date(2026, 11, 6))

@assert is_day_off(info)
@assert is_official(info)
```

Команды для сборки нативного модуля и запуска тестов приведены в
[`bindings/julia/README.md`](bindings/julia/README.md).
