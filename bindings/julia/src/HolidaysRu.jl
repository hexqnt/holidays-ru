module HolidaysRu

using Dates: Date, day, month, year
using JlrsCore.Wrap: @initjlrs, @wrapmodule
import Libdl

export FIRST_FACT_YEAR, LAST_FACT_YEAR, MAX_YEAR, MIN_YEAR
export Calendar, DataSource, DayFlags, DayInfo, Region, Resolved, WorkWeek
export Official, Predicted
export FortyHours, ThirtySixHours, TwentyFourHours
export Adygea, AltaiRepublic, Bashkortostan, Buryatia, Dagestan, Ingushetia
export KabardinoBalkaria, Kalmykia, KarachayCherkessia, Crimea, Mordovia
export NorthOssetiaAlania, Tatarstan, Tuva, Chechnya, Chuvashia
export ZabaykalskyKrai, KrasnodarKrai, StavropolKrai, BelgorodOblast
export BryanskOblast, IrkutskOblast, OmskOblast, PenzaOblast, SaratovOblast
export day_info, is_day_off, is_holiday, is_official, is_predicted
export is_short_day, is_transferred, is_weekend, is_working_day
export non_working_days_between, working_hours_between, working_minutes_between

function _default_library_path()
    library_name = if Sys.iswindows()
        "holidays_ru_julia.dll"
    else
        "libholidays_ru_julia.$(Libdl.dlext)"
    end

    joinpath(@__DIR__, "..", "native", "target", "debug", library_name)
end

const _NATIVE_LIBRARY = get(ENV, "HOLIDAYS_RU_JULIA_LIB", _default_library_path())

@wrapmodule(_NATIVE_LIBRARY, :holidays_ru_julia_init)

function __init__()
    @initjlrs
end

"""Origin of resolved calendar data."""
@enum DataSource::UInt8 begin
    Official = 0
    Predicted = 1
end

"""A regional calendar combined with the federal production calendar."""
@enum Region::UInt8 begin
    Adygea = _REGION_ADYGEA
    AltaiRepublic = _REGION_ALTAI_REPUBLIC
    Bashkortostan = _REGION_BASHKORTOSTAN
    Buryatia = _REGION_BURYATIA
    Dagestan = _REGION_DAGESTAN
    Ingushetia = _REGION_INGUSHETIA
    KabardinoBalkaria = _REGION_KABARDINO_BALKARIA
    Kalmykia = _REGION_KALMYKIA
    KarachayCherkessia = _REGION_KARACHAY_CHERKESSIA
    Crimea = _REGION_CRIMEA
    Mordovia = _REGION_MORDOVIA
    NorthOssetiaAlania = _REGION_NORTH_OSSETIA_ALANIA
    Tatarstan = _REGION_TATARSTAN
    Tuva = _REGION_TUVA
    Chechnya = _REGION_CHECHNYA
    Chuvashia = _REGION_CHUVASHIA
    ZabaykalskyKrai = _REGION_ZABAYKALSKY_KRAI
    KrasnodarKrai = _REGION_KRASNODAR_KRAI
    StavropolKrai = _REGION_STAVROPOL_KRAI
    BelgorodOblast = _REGION_BELGOROD_OBLAST
    BryanskOblast = _REGION_BRYANSK_OBLAST
    IrkutskOblast = _REGION_IRKUTSK_OBLAST
    OmskOblast = _REGION_OMSK_OBLAST
    PenzaOblast = _REGION_PENZA_OBLAST
    SaratovOblast = _REGION_SARATOV_OBLAST
end

"""Weekly working-hours norm."""
@enum WorkWeek::UInt8 begin
    FortyHours = 40
    ThirtySixHours = 36
    TwentyFourHours = 24
end

"""Bit flags assigned to a calendar day."""
struct DayFlags
    bits::UInt8
end

"""
A value together with the origin of its calendar data. Range results are
`Predicted` if at least one day in the range uses predicted data.
"""
struct Resolved{T}
    value::T
    source::DataSource
end

"""Resolved calendar properties for one date."""
struct DayInfo
    date::Date
    flags::DayFlags
    source::DataSource
end

"""
    Calendar()
    Calendar(region::Region)

A reusable federal or federal-plus-regional production calendar.
"""
struct Calendar
    region::Union{Nothing,Region}
end

Calendar() = Calendar(nothing)

@inline _has_flag(flags::DayFlags, flag::UInt8) = !iszero(flags.bits & flag)
@inline _calendar_code(::Nothing) = _CALENDAR_FEDERAL_CODE
@inline _calendar_code(region::Region) = UInt8(region)
@inline _source(raw::UInt64) = iszero(raw & _OFFICIAL_BIT) ? Predicted : Official
@inline _value(raw::UInt64) = UInt32(raw & _VALUE_MASK)

function _require_valid(raw::UInt64, message::String)
    iszero(raw & _VALID_BIT) && throw(ArgumentError(message))
    nothing
end

function _parts(value::Date)
    y = year(value)
    typemin(Int32) <= y <= typemax(Int32) || throw(ArgumentError("date year is outside Int32"))
    (Int32(y), UInt8(month(value)), UInt8(day(value)))
end

"""Return whether a result is based entirely on official data."""
is_official(value::DayInfo) = value.source == Official
is_official(value::Resolved) = value.source == Official

"""Return whether any part of a result is predicted."""
is_predicted(value::DayInfo) = value.source == Predicted
is_predicted(value::Resolved) = value.source == Predicted

"""Return whether the flags describe a Saturday or Sunday."""
is_weekend(flags::DayFlags) = _has_flag(flags, _FLAG_WEEKEND)

"""Return whether the flags describe a non-working public holiday."""
is_holiday(flags::DayFlags) = _has_flag(flags, _FLAG_HOLIDAY)

"""Return whether the flags describe a non-working day."""
is_day_off(flags::DayFlags) = _has_flag(flags, _FLAG_DAY_OFF)

"""Return whether the flags describe a working day."""
is_working_day(flags::DayFlags) = _has_flag(flags, _FLAG_WORKING_DAY)

"""Return whether the flags describe a shortened working day."""
is_short_day(flags::DayFlags) = _has_flag(flags, _FLAG_SHORT_DAY)

"""Return whether the flags describe a transferred day off."""
is_transferred(flags::DayFlags) = _has_flag(flags, _FLAG_TRANSFERRED)

is_weekend(info::DayInfo) = is_weekend(info.flags)
is_holiday(info::DayInfo) = is_holiday(info.flags)
is_day_off(info::DayInfo) = is_day_off(info.flags)
is_working_day(info::DayInfo) = is_working_day(info.flags)
is_short_day(info::DayInfo) = is_short_day(info.flags)
is_transferred(info::DayInfo) = is_transferred(info.flags)

"""
    day_info(date::Date; region=nothing) -> DayInfo
    day_info(calendar::Calendar, date::Date) -> DayInfo

Return resolved calendar properties for `date`. With no region, the federal
production calendar is used. Regional calendars always include federal data.
"""
function day_info(date::Date; region::Union{Nothing,Region}=nothing)
    y, m, d = _parts(date)
    raw = _native_day_info(y, m, d, _calendar_code(region))
    _require_valid(raw, "date must be valid and between $(MIN_YEAR)-01-01 and $(MAX_YEAR)-12-31")
    DayInfo(date, DayFlags(UInt8(_value(raw))), _source(raw))
end

day_info(calendar::Calendar, date::Date) = day_info(date; region=calendar.region)

is_weekend(date::Date; region::Union{Nothing,Region}=nothing) =
    is_weekend(day_info(date; region=region))
is_holiday(date::Date; region::Union{Nothing,Region}=nothing) =
    is_holiday(day_info(date; region=region))
is_day_off(date::Date; region::Union{Nothing,Region}=nothing) =
    is_day_off(day_info(date; region=region))
is_working_day(date::Date; region::Union{Nothing,Region}=nothing) =
    is_working_day(day_info(date; region=region))
is_short_day(date::Date; region::Union{Nothing,Region}=nothing) =
    is_short_day(day_info(date; region=region))
is_transferred(date::Date; region::Union{Nothing,Region}=nothing) =
    is_transferred(day_info(date; region=region))

is_weekend(calendar::Calendar, date::Date) = is_weekend(date; region=calendar.region)
is_holiday(calendar::Calendar, date::Date) = is_holiday(date; region=calendar.region)
is_day_off(calendar::Calendar, date::Date) = is_day_off(date; region=calendar.region)
is_working_day(calendar::Calendar, date::Date) = is_working_day(date; region=calendar.region)
is_short_day(calendar::Calendar, date::Date) = is_short_day(date; region=calendar.region)
is_transferred(calendar::Calendar, date::Date) = is_transferred(date; region=calendar.region)

"""
    non_working_days_between(start::Date, stop::Date; region=nothing)
    non_working_days_between(calendar::Calendar, start::Date, stop::Date)

Count non-working days in the half-open interval `[start, stop)`.
"""
function non_working_days_between(
    start::Date,
    stop::Date;
    region::Union{Nothing,Region}=nothing,
)
    sy, sm, sd = _parts(start)
    ey, em, ed = _parts(stop)
    raw = _native_non_working_days_between(sy, sm, sd, ey, em, ed, _calendar_code(region))
    _require_valid(raw, "invalid or unsupported date range")
    Resolved(Int(_value(raw)), _source(raw))
end

function non_working_days_between(calendar::Calendar, start::Date, stop::Date)
    non_working_days_between(start, stop; region=calendar.region)
end

"""
    working_minutes_between(start::Date, stop::Date; week=FortyHours, region=nothing)
    working_minutes_between(calendar::Calendar, start::Date, stop::Date; week=FortyHours)

Count working minutes in the half-open interval `[start, stop)`. Shortened
working days reduce the selected daily norm by 60 minutes.
"""
function working_minutes_between(
    start::Date,
    stop::Date;
    week::WorkWeek=FortyHours,
    region::Union{Nothing,Region}=nothing,
)
    sy, sm, sd = _parts(start)
    ey, em, ed = _parts(stop)
    raw = _native_working_minutes_between(
        sy,
        sm,
        sd,
        ey,
        em,
        ed,
        UInt8(week),
        _calendar_code(region),
    )
    _require_valid(raw, "invalid or unsupported date range")
    Resolved(Int(_value(raw)), _source(raw))
end

function working_minutes_between(
    calendar::Calendar,
    start::Date,
    stop::Date;
    week::WorkWeek=FortyHours,
)
    working_minutes_between(start, stop; week=week, region=calendar.region)
end

"""
    working_hours_between(start::Date, stop::Date; week=FortyHours, region=nothing)
    working_hours_between(calendar::Calendar, start::Date, stop::Date; week=FortyHours)

Count working hours in the half-open interval `[start, stop)`. The returned
value can be fractional.
"""
function working_hours_between(
    start::Date,
    stop::Date;
    week::WorkWeek=FortyHours,
    region::Union{Nothing,Region}=nothing,
)
    minutes = working_minutes_between(start, stop; week=week, region=region)
    Resolved(minutes.value / 60, minutes.source)
end

function working_hours_between(
    calendar::Calendar,
    start::Date,
    stop::Date;
    week::WorkWeek=FortyHours,
)
    minutes = working_minutes_between(calendar, start, stop; week=week)
    Resolved(minutes.value / 60, minutes.source)
end

end
