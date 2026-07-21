__version__: str
FIRST_FACT_YEAR: int
LAST_FACT_YEAR: int
MIN_YEAR: int
MAX_YEAR: int
_REGION_NAMES: tuple[str, ...]

def _day_info(
    year: int,
    month: int,
    day: int,
    region: str | None = None,
) -> tuple[int, bool]: ...
def _non_working_days_between(
    start: tuple[int, int, int],
    end: tuple[int, int, int],
    region: str | None = None,
) -> tuple[int, bool]: ...
def _working_minutes_between(
    start: tuple[int, int, int],
    end: tuple[int, int, int],
    week: int,
    region: str | None = None,
) -> tuple[int, bool]: ...
