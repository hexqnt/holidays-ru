# holidays-ru for Python

Python bindings for the [`holidays-ru`](https://crates.io/crates/holidays-ru)
Rust library. The Python package is a separate workspace member: Rust users do
not depend on PyO3 or any Python runtime.

```python
from datetime import date

from holidays_ru import Calendar, Region

calendar = Calendar(Region.TATARSTAN)
info = calendar.day(date(2026, 11, 6))

assert info.is_day_off
assert info.is_official
```

The module-level API uses the federal calendar by default:

```python
from datetime import date

import holidays_ru

info = holidays_ru.day_info(date(2026, 1, 9))
assert info.is_day_off
assert info.is_transferred

days = holidays_ru.non_working_days_between(
    date(2026, 1, 1),
    date(2027, 1, 1),
)
assert days.is_official
```

Date ranges are half-open: the start is included and the end is excluded.
Outside the official-data range, results have
`source == DataSource.PREDICTED`.

## Development

```console
python -m pip install -e ".[test]"
python -m pytest
python -m mypy
python -m ruff check .
python -m ruff format --check .
```
