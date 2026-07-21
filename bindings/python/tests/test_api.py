from datetime import date, datetime

import holidays_ru
import pytest
from holidays_ru import _holidays_ru as _native


def test_official_federal_day() -> None:
    info = holidays_ru.day_info(date(2026, 1, 9))

    assert info.is_day_off
    assert info.is_transferred
    assert not info.is_holiday
    assert info.source is holidays_ru.DataSource.OFFICIAL


def test_prediction_keeps_its_source() -> None:
    info = holidays_ru.day_info(date(2027, 1, 1))

    assert info.is_holiday
    assert info.is_predicted


def test_region_is_combined_with_federal_calendar() -> None:
    federal = holidays_ru.day_info(date(2026, 11, 6))
    tatarstan = holidays_ru.day_info(
        date(2026, 11, 6),
        region=holidays_ru.Region.TATARSTAN,
    )

    assert federal.is_working_day
    assert tatarstan.is_day_off
    assert tatarstan.is_holiday


def test_every_public_region_is_supported_by_native_module() -> None:
    day = date(2026, 1, 12)

    assert {region.value for region in holidays_ru.Region} == set(_native._REGION_NAMES)
    for region in holidays_ru.Region:
        assert holidays_ru.day_info(day, region=region).date == day


def test_regional_ranges_use_the_full_calendar() -> None:
    calendar = holidays_ru.Calendar("tatarstan")
    start = date(2026, 11, 6)
    end = date(2026, 11, 7)

    assert calendar.non_working_days_between(start, end).value == 1
    assert calendar.working_minutes_between(start, end).value == 0

    federal_holiday = date(2026, 1, 1)
    day_after = date(2026, 1, 2)
    assert calendar.non_working_days_between(federal_holiday, day_after).value == 1

    predicted = calendar.non_working_days_between(date(2027, 1, 1), date(2027, 1, 2))
    assert predicted.is_predicted


def test_working_hours_preserve_resolution() -> None:
    result = holidays_ru.working_hours_between(
        date(2026, 1, 12),
        date(2026, 1, 13),
        week=holidays_ru.WorkWeek.THIRTY_SIX_HOURS,
    )

    assert result.value == 7.2
    assert result.is_official


def test_boolean_shortcuts() -> None:
    day = date(2026, 11, 3)

    assert holidays_ru.is_working_day(day)
    assert holidays_ru.is_short_day(day)
    assert not holidays_ru.is_day_off(day)


@pytest.mark.parametrize(
    ("value", "message"),
    [
        (date(1899, 12, 31), "date must be valid"),
        (date(2101, 1, 1), "date must be valid"),
    ],
)
def test_unsupported_day(value: date, message: str) -> None:
    with pytest.raises(ValueError, match=message):
        holidays_ru.day_info(value)


def test_invalid_range() -> None:
    with pytest.raises(ValueError, match="invalid or unsupported date range"):
        holidays_ru.non_working_days_between(date(2026, 1, 2), date(2026, 1, 1))


def test_datetime_is_not_silently_truncated() -> None:
    with pytest.raises(TypeError, match=r"datetime\.date"):
        holidays_ru.day_info(datetime(2026, 1, 1))


def test_unknown_region() -> None:
    with pytest.raises(ValueError, match="unsupported region"):
        holidays_ru.Calendar("unknown")


def test_work_week_requires_enum() -> None:
    with pytest.raises(TypeError, match="WorkWeek"):
        holidays_ru.working_minutes_between(
            date(2026, 1, 12),
            date(2026, 1, 13),
            week=40,  # type: ignore[arg-type]
        )
