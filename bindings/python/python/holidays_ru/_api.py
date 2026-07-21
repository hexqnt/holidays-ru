"""Public, Python-native API for :mod:`holidays_ru`."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import date, datetime
from enum import IntEnum, IntFlag, StrEnum
from typing import Generic, TypeVar

from . import _holidays_ru as _native


class DataSource(StrEnum):
    """Origin of calendar data."""

    OFFICIAL = "official"
    PREDICTED = "predicted"


class Region(StrEnum):
    """A Russian regional calendar combined with the federal calendar."""

    ADYGEA = "adygea"
    ALTAI_REPUBLIC = "altai_republic"
    BASHKORTOSTAN = "bashkortostan"
    BURYATIA = "buryatia"
    DAGESTAN = "dagestan"
    INGUSHETIA = "ingushetia"
    KABARDINO_BALKARIA = "kabardino_balkaria"
    KALMYKIA = "kalmykia"
    KARACHAY_CHERKESSIA = "karachay_cherkessia"
    CRIMEA = "crimea"
    MORDOVIA = "mordovia"
    NORTH_OSSETIA_ALANIA = "north_ossetia_alania"
    TATARSTAN = "tatarstan"
    TUVA = "tuva"
    CHECHNYA = "chechnya"
    CHUVASHIA = "chuvashia"
    ZABAYKALSKY_KRAI = "zabaykalsky_krai"
    KRASNODAR_KRAI = "krasnodar_krai"
    STAVROPOL_KRAI = "stavropol_krai"
    BELGOROD_OBLAST = "belgorod_oblast"
    BRYANSK_OBLAST = "bryansk_oblast"
    IRKUTSK_OBLAST = "irkutsk_oblast"
    OMSK_OBLAST = "omsk_oblast"
    PENZA_OBLAST = "penza_oblast"
    SARATOV_OBLAST = "saratov_oblast"


class WorkWeek(IntEnum):
    """Weekly working-hours norm."""

    FORTY_HOURS = 40
    THIRTY_SIX_HOURS = 36
    TWENTY_FOUR_HOURS = 24


class DayFlags(IntFlag):
    """Properties assigned to a calendar day."""

    EMPTY = 0
    WEEKEND = 1 << 0
    HOLIDAY = 1 << 1
    DAY_OFF = 1 << 2
    WORKING_DAY = 1 << 3
    SHORT_DAY = 1 << 4
    TRANSFERRED = 1 << 5


T = TypeVar("T")


@dataclass(frozen=True, slots=True)
class Resolved(Generic[T]):
    """A value together with the origin of the underlying calendar data."""

    value: T
    source: DataSource

    @property
    def is_official(self) -> bool:
        """Whether the value is based entirely on official data."""

        return self.source is DataSource.OFFICIAL

    @property
    def is_predicted(self) -> bool:
        """Whether any part of the value is a prediction."""

        return self.source is DataSource.PREDICTED


@dataclass(frozen=True, slots=True)
class DayInfo:
    """Resolved properties of one calendar day."""

    date: date
    flags: DayFlags
    source: DataSource

    @property
    def is_weekend(self) -> bool:
        """Whether the day falls on Saturday or Sunday."""

        return bool(self.flags & DayFlags.WEEKEND)

    @property
    def is_holiday(self) -> bool:
        """Whether the day is a non-working public holiday."""

        return bool(self.flags & DayFlags.HOLIDAY)

    @property
    def is_day_off(self) -> bool:
        """Whether the day is non-working."""

        return bool(self.flags & DayFlags.DAY_OFF)

    @property
    def is_working_day(self) -> bool:
        """Whether the day is working."""

        return bool(self.flags & DayFlags.WORKING_DAY)

    @property
    def is_short_day(self) -> bool:
        """Whether the day is a shortened working day."""

        return bool(self.flags & DayFlags.SHORT_DAY)

    @property
    def is_transferred(self) -> bool:
        """Whether the day was affected by a day-off transfer."""

        return bool(self.flags & DayFlags.TRANSFERRED)

    @property
    def is_official(self) -> bool:
        """Whether the flags are based on official data."""

        return self.source is DataSource.OFFICIAL

    @property
    def is_predicted(self) -> bool:
        """Whether the flags are predicted."""

        return self.source is DataSource.PREDICTED


RegionLike = Region | str | None


def _require_date(value: date, argument: str) -> date:
    if not isinstance(value, date) or isinstance(value, datetime):
        raise TypeError(f"{argument} must be a datetime.date")
    return value


def _region_value(region: RegionLike) -> str | None:
    if region is None:
        return None
    if isinstance(region, Region):
        return region.value
    if isinstance(region, str):
        try:
            return Region(region).value
        except ValueError:
            raise ValueError(f"unsupported region: {region}") from None
    raise TypeError("region must be a Region, string, or None")


def _source(is_official: bool) -> DataSource:
    return DataSource.OFFICIAL if is_official else DataSource.PREDICTED


def day_info(day: date, *, region: RegionLike = None) -> DayInfo:
    """Return calendar properties for ``day``.

    With no region, the federal calendar is used. A regional calendar is
    always combined with the federal calendar.
    """

    day = _require_date(day, "day")
    bits, is_official = _native._day_info(
        day.year,
        day.month,
        day.day,
        _region_value(region),
    )
    return DayInfo(day, DayFlags(bits), _source(is_official))


def is_day_off(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` is non-working.

    Use :func:`day_info` when the official/predicted distinction is needed.
    """

    return day_info(day, region=region).is_day_off


def is_working_day(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` is working.

    Use :func:`day_info` when the official/predicted distinction is needed.
    """

    return day_info(day, region=region).is_working_day


def is_holiday(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` is a non-working holiday."""

    return day_info(day, region=region).is_holiday


def is_short_day(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` is a shortened working day."""

    return day_info(day, region=region).is_short_day


def is_weekend(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` is Saturday or Sunday."""

    return day_info(day, region=region).is_weekend


def is_transferred(day: date, *, region: RegionLike = None) -> bool:
    """Return whether ``day`` was affected by a day-off transfer."""

    return day_info(day, region=region).is_transferred


def non_working_days_between(
    start: date,
    end: date,
    *,
    region: RegionLike = None,
) -> Resolved[int]:
    """Count non-working days in the half-open interval ``[start, end)``."""

    start = _require_date(start, "start")
    end = _require_date(end, "end")
    value, is_official = _native._non_working_days_between(
        (start.year, start.month, start.day),
        (end.year, end.month, end.day),
        _region_value(region),
    )
    return Resolved(value, _source(is_official))


def working_minutes_between(
    start: date,
    end: date,
    *,
    week: WorkWeek = WorkWeek.FORTY_HOURS,
    region: RegionLike = None,
) -> Resolved[int]:
    """Count working minutes in the half-open interval ``[start, end)``."""

    start = _require_date(start, "start")
    end = _require_date(end, "end")
    if not isinstance(week, WorkWeek):
        raise TypeError("week must be a WorkWeek")
    value, is_official = _native._working_minutes_between(
        (start.year, start.month, start.day),
        (end.year, end.month, end.day),
        int(week),
        _region_value(region),
    )
    return Resolved(value, _source(is_official))


def working_hours_between(
    start: date,
    end: date,
    *,
    week: WorkWeek = WorkWeek.FORTY_HOURS,
    region: RegionLike = None,
) -> Resolved[float]:
    """Count working hours in the half-open interval ``[start, end)``."""

    minutes = working_minutes_between(start, end, week=week, region=region)
    return Resolved(minutes.value / 60.0, minutes.source)


@dataclass(frozen=True, slots=True, init=False)
class Calendar:
    """A reusable federal or federal-plus-regional production calendar."""

    region: Region | None

    def __init__(self, region: RegionLike = None) -> None:
        value = _region_value(region)
        object.__setattr__(self, "region", Region(value) if value is not None else None)

    def day(self, day: date) -> DayInfo:
        """Return resolved properties for one day."""

        return day_info(day, region=self.region)

    def is_day_off(self, day: date) -> bool:
        """Return whether ``day`` is non-working."""

        return is_day_off(day, region=self.region)

    def is_working_day(self, day: date) -> bool:
        """Return whether ``day`` is working."""

        return is_working_day(day, region=self.region)

    def is_holiday(self, day: date) -> bool:
        """Return whether ``day`` is a non-working holiday."""

        return is_holiday(day, region=self.region)

    def is_short_day(self, day: date) -> bool:
        """Return whether ``day`` is a shortened working day."""

        return is_short_day(day, region=self.region)

    def is_weekend(self, day: date) -> bool:
        """Return whether ``day`` falls on Saturday or Sunday."""

        return is_weekend(day, region=self.region)

    def is_transferred(self, day: date) -> bool:
        """Return whether ``day`` was affected by a day-off transfer."""

        return is_transferred(day, region=self.region)

    def non_working_days_between(self, start: date, end: date) -> Resolved[int]:
        """Count non-working days in the half-open interval ``[start, end)``."""

        return non_working_days_between(start, end, region=self.region)

    def working_minutes_between(
        self,
        start: date,
        end: date,
        *,
        week: WorkWeek = WorkWeek.FORTY_HOURS,
    ) -> Resolved[int]:
        """Count working minutes in the half-open interval ``[start, end)``."""

        return working_minutes_between(start, end, week=week, region=self.region)

    def working_hours_between(
        self,
        start: date,
        end: date,
        *,
        week: WorkWeek = WorkWeek.FORTY_HOURS,
    ) -> Resolved[float]:
        """Count working hours in the half-open interval ``[start, end)``."""

        return working_hours_between(start, end, week=week, region=self.region)
