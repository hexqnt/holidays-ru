use holidays_ru::Resolved;
use holidays_ru_bindings_common::{
    CalendarSelection, DateParts, REGION_NAMES, work_week_from_hours,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn parse_calendar(region: Option<&str>) -> PyResult<CalendarSelection> {
    CalendarSelection::parse(region).map_err(|error| PyValueError::new_err(error.to_string()))
}

#[inline]
fn into_native<T>(resolved: Resolved<T>) -> (T, bool) {
    let is_official = resolved.is_fact();
    (resolved.value(), is_official)
}

#[pyfunction(signature = (year, month, day, region=None))]
fn _day_info(year: i32, month: u8, day: u8, region: Option<&str>) -> PyResult<(u8, bool)> {
    let calendar = parse_calendar(region)?;
    let resolved = calendar.flags_ymd(year, month, day).ok_or_else(|| {
        PyValueError::new_err(format!(
            "date must be valid and between {}-01-01 and {}-12-31",
            holidays_ru::MIN_YEAR,
            holidays_ru::MAX_YEAR,
        ))
    })?;
    let (flags, is_official) = into_native(resolved);

    Ok((flags.bits(), is_official))
}

#[pyfunction(signature = (start, end, region=None))]
fn _non_working_days_between(
    start: DateParts,
    end: DateParts,
    region: Option<&str>,
) -> PyResult<(u32, bool)> {
    let calendar = parse_calendar(region)?;
    let resolved = calendar
        .non_working_days_between(start, end)
        .ok_or_else(|| PyValueError::new_err("invalid or unsupported date range"))?;

    Ok(into_native(resolved))
}

#[pyfunction(signature = (start, end, week, region=None))]
fn _working_minutes_between(
    start: DateParts,
    end: DateParts,
    week: u8,
    region: Option<&str>,
) -> PyResult<(u32, bool)> {
    let week = work_week_from_hours(week)
        .ok_or_else(|| PyValueError::new_err("work week must be 40, 36, or 24 hours"))?;
    let calendar = parse_calendar(region)?;
    let resolved = calendar
        .working_minutes_between(start, end, week)
        .ok_or_else(|| PyValueError::new_err("invalid or unsupported date range"))?;

    Ok(into_native(resolved))
}

#[pymodule]
fn _holidays_ru(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let region_names = PyTuple::new(module.py(), REGION_NAMES)?;

    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("FIRST_FACT_YEAR", holidays_ru::FIRST_FACT_YEAR)?;
    module.add("LAST_FACT_YEAR", holidays_ru::LAST_FACT_YEAR)?;
    module.add("MIN_YEAR", holidays_ru::MIN_YEAR)?;
    module.add("MAX_YEAR", holidays_ru::MAX_YEAR)?;
    module.add("_REGION_NAMES", region_names)?;
    module.add_function(wrap_pyfunction!(_day_info, module)?)?;
    module.add_function(wrap_pyfunction!(_non_working_days_between, module)?)?;
    module.add_function(wrap_pyfunction!(_working_minutes_between, module)?)?;

    Ok(())
}
