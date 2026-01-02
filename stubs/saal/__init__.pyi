# flake8: noqa
"""Type hints for saal Python bindings.

Units:
    - ds50 values are days since 1950-01-01 00:00:00 in the given time scale.
    - Angles are radians unless noted otherwise.

Example:
    ```python
    from saal import TimeInterface

    ti = TimeInterface()
    ds50 = ti.ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0)
    print(f"{ds50:.1f}")
    ```

    Output:
    ```bash
    2192.0
    ```
"""

from __future__ import annotations

class KeyMode:
    """Global key mode for all keys.

    Controls whether duplicate keys are rejected or stored with DMA.

    Example:
        ```python
        from saal import KeyMode

        mode = int(KeyMode.DirectMemoryAccess)
        print(mode)
        ```

        Output:
        ```bash
        1
        ```
    """

    NoDuplicates: KeyMode
    DirectMemoryAccess: KeyMode

    def __int__(self) -> int:
        """Return the underlying integer value.

        Example:
            ```python
            from saal import KeyMode

            value = int(KeyMode.NoDuplicates)
            print(value)
            ```

            Output:
            ```bash
            0
            ```
        """
        ...

class KeyOrder:
    """Ordering options for retrieved keys."""

    Ascending: KeyOrder
    Descending: KeyOrder
    Fastest: KeyOrder
    LoadTime: KeyOrder

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class DuplicateKeyMode:
    """Duplicate key handling behavior.

    Controls whether duplicates yield 0 or the existing key.

    Example:
        ```python
        from saal import DuplicateKeyMode

        value = int(DuplicateKeyMode.ReturnZero)
        print(value)
        ```

        Output:
        ```bash
        0
        ```
    """

    ReturnZero: DuplicateKeyMode
    ReturnKey: DuplicateKeyMode

    def __int__(self) -> int:
        """Return the underlying integer value.

        Example:
            ```python
            from saal import DuplicateKeyMode

            value = int(DuplicateKeyMode.ReturnKey)
            print(value)
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

class ElsetKeyMode:
    """Key mode for ELSET keys.

    Controls duplicate handling for ELSET keys only.

    Example:
        ```python
        from saal import ElsetKeyMode

        value = int(ElsetKeyMode.NoDuplicates)
        print(value)
        ```

        Output:
        ```bash
        0
        ```
    """

    NoDuplicates: ElsetKeyMode
    DirectMemoryAccess: ElsetKeyMode

    def __int__(self) -> int:
        """Return the underlying integer value.

        Example:
            ```python
            from saal import ElsetKeyMode

            value = int(ElsetKeyMode.DirectMemoryAccess)
            print(value)
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

class ElementType:
    """Orbital element type identifiers."""

    TwoLineSGP: ElementType
    TwoLineSGP4: ElementType
    TwoLineSP: ElementType
    SPVector: ElementType
    VCM: ElementType
    Ephemeris: ElementType
    TwoLineXP: ElementType

    def __int__(self) -> int:
        """Return the underlying integer value.

        Example:
            ```python
            from saal import ElementType

            value = int(ElementType.TwoLineSGP4)
            print(value)
            ```

            Output:
            ```bash
            2
            ```
        """
        ...

class TLEType:
    """TLE ephemeris type identifiers."""

    SGP: TLEType
    SGP4: TLEType
    SP: TLEType
    XP: TLEType

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class Classification:
    """TLE classification codes."""

    Unclassified: Classification
    Confidential: Classification
    Secret: Classification

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class B3Type:
    """Observation B3 record types."""

    Zero: B3Type
    One: B3Type
    Two: B3Type
    Three: B3Type
    Four: B3Type
    Five: B3Type
    Six: B3Type
    Eight: B3Type
    Nine: B3Type

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class PositionInTrack:
    """Observation track position indicators."""

    Beginning: PositionInTrack
    Middle: PositionInTrack
    End: PositionInTrack

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class AssociationStatus:
    """Observation association status indicators."""

    Statistical: AssociationStatus
    High: AssociationStatus
    Medium: AssociationStatus
    Low: AssociationStatus
    None: AssociationStatus

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class FundamentalCatalog:
    """Fundamental catalog selection (FK4 or FK5)."""

    Four: FundamentalCatalog
    Five: FundamentalCatalog

    def __int__(self) -> int:
        """Return the underlying integer value.

        Example:
            ```python
            from saal import FundamentalCatalog

            value = int(FundamentalCatalog.Five)
            print(value)
            ```

            Output:
            ```bash
            5
            ```
        """
        ...

class SGP4OutputEphemerisFrame:
    """SGP4 output ephemeris frames."""

    TEME: SGP4OutputEphemerisFrame
    J2000: SGP4OutputEphemerisFrame

    def __int__(self) -> int:
        """Return the underlying integer value."""
        ...

class MainInterface:
    """Access DllMain settings, messages, and key modes."""

    DLL_VERSION: str

    def __init__(self, file_name: str | None = None) -> None:
        """Create an interface and optionally load an input file."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import MainInterface

            main = MainInterface()
            print(MainInterface.DLL_VERSION in main.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def last_error_message(self) -> str:
        """Last error message reported by the DLL."""
        ...

    @property
    def last_info_message(self) -> str:
        """Last informational message reported by the DLL."""
        ...

    @property
    def key_mode(self) -> KeyMode:
        """Global key mode for all keys.

        Example:
            ```python
            from saal import MainInterface

            main = MainInterface()
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @key_mode.setter
    def key_mode(self, mode: KeyMode) -> None:
        """Set the global key mode for all keys.

        Example:
            ```python
            from saal import KeyMode, MainInterface

            main = MainInterface()
            main.key_mode = KeyMode.NoDuplicates
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            0
            ```
        """
        ...

    def reset_key_mode(self) -> None:
        """Reset global key mode and duplicate key mode to defaults.

        Example:
            ```python
            from saal import KeyMode, MainInterface

            main = MainInterface()
            main.key_mode = KeyMode.NoDuplicates
            main.reset_key_mode()
            print(int(main.key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @property
    def elset_key_mode(self) -> ElsetKeyMode:
        """ELSET key mode.

        Example:
            ```python
            from saal import ElsetKeyMode, MainInterface

            main = MainInterface()
            main.elset_key_mode = ElsetKeyMode.NoDuplicates
            print(int(main.elset_key_mode))
            ```

            Output:
            ```bash
            0
            ```
        """
        ...

    @elset_key_mode.setter
    def elset_key_mode(self, mode: ElsetKeyMode) -> None:
        """Set the ELSET key mode.

        Example:
            ```python
            from saal import ElsetKeyMode, MainInterface

            main = MainInterface()
            main.elset_key_mode = ElsetKeyMode.DirectMemoryAccess
            print(int(main.elset_key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

    @property
    def duplicate_key_mode(self) -> DuplicateKeyMode:
        """Duplicate key mode behavior."""
        ...

    @duplicate_key_mode.setter
    def duplicate_key_mode(self, mode: DuplicateKeyMode) -> None:
        """Set the behavior of returned keys when a duplicate is loaded in NoDuplicates mode.

        Check DuplicateKeyMode for return options.

        Example:
            ```python
            from saal import DuplicateKeyMode, MainInterface

            main = MainInterface()
            main.duplicate_key_mode = DuplicateKeyMode.ReturnKey
            print(int(main.duplicate_key_mode))
            ```

            Output:
            ```bash
            1
            ```
        """
        ...

class AstroInterface:
    """Astronomical conversion utilities."""

    def __init__(self) -> None: ...
    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform)."""
        ...

    def keplerian_to_equinoctial(self, kep: list[float]) -> list[float]: ...
    def equinoctial_to_keplerian(self, eqnx: list[float]) -> list[float]: ...
    def keplerian_to_cartesian(self, kep: list[float]) -> list[float]: ...
    def cartesian_to_keplerian(self, posvel: list[float]) -> list[float]: ...
    def mean_motion_to_sma(self, mean_motion: float) -> float: ...
    def sma_to_mean_motion(self, semi_major_axis: float) -> float: ...
    def kozai_to_brouwer(self, eccentricity: float, inclination: float, mean_motion: float) -> float: ...
    def brouwer_to_kozai(self, eccentricity: float, inclination: float, mean_motion: float) -> float: ...
    def osculating_to_mean(self, osc: list[float]) -> list[float]: ...
    def gst_teme_to_lla(self, gst: float, teme_pos: list[float]) -> list[float]: ...
    def efg_to_lla(self, efg_pos: list[float]) -> list[float]: ...

class EnvironmentInterface:
    """Access Earth constants and fundamental catalog settings."""

    def __init__(self, file_name: str | None = None) -> None:
        """Create an interface and optionally load an input file."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import EnvironmentInterface, MainInterface

            env = EnvironmentInterface()
            print(MainInterface.DLL_VERSION in env.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def earth_radius(self) -> float:
        """Earth radius from the current GEO model.

        Units: kilometers.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_radius:.3f}")
            ```

            Output:
            ```bash
            6378.135
            ```
        """
        ...

    @property
    def earth_rotation_rate(self) -> float:
        """Earth rotation rate from the current FK model.

        Units: radians/day.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_rotation_rate:.18f}")
            ```

            Output:
            ```bash
            0.017202791694070362
            ```
        """
        ...

    @property
    def earth_rotation_acceleration(self) -> float:
        """Earth rotation acceleration from the current FK model.

        Units: radians/day^2.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_rotation_acceleration:.15e}")
            ```

            Output:
            ```bash
            5.075514194322695e-15
            ```
        """
        ...

    @property
    def earth_mu(self) -> float:
        """Earth gravitational parameter from the current GEO model.

        Units: km^3/s^2.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_mu:.1f}")
            ```

            Output:
            ```bash
            398600.8
            ```
        """
        ...

    @property
    def earth_flattening(self) -> float:
        """Earth flattening (reciprocal) from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.earth_flattening:.15f}")
            ```

            Output:
            ```bash
            0.003352779454168
            ```
        """
        ...

    @property
    def j2(self) -> float:
        """J2 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j2:.9f}")
            ```

            Output:
            ```bash
            0.001082616
            ```
        """
        ...

    @property
    def j3(self) -> float:
        """J3 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j3:.11f}")
            ```

            Output:
            ```bash
            -0.00000253881
            ```
        """
        ...

    @property
    def j4(self) -> float:
        """J4 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j4:.11f}")
            ```

            Output:
            ```bash
            -0.00000165597
            ```
        """
        ...

    @property
    def j5(self) -> float:
        """J5 coefficient from the current GEO model.

        Units: unitless.

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(f"{env.j5:.7e}")
            ```

            Output:
            ```bash
            -2.1848270e-07
            ```
        """
        ...

    @property
    def fundamental_catalog(self) -> FundamentalCatalog:
        """Fundamental catalog selection (FK4 or FK5).

        Example:
            ```python
            from saal import EnvironmentInterface

            env = EnvironmentInterface()
            print(int(env.fundamental_catalog))
            ```

            Output:
            ```bash
            5
            ```
        """
        ...

    @fundamental_catalog.setter
    def fundamental_catalog(self, catalog: FundamentalCatalog) -> None:
        """Set the fundamental catalog selection.

        Example:
            ```python
            from saal import EnvironmentInterface, FundamentalCatalog

            env = EnvironmentInterface()
            env.fundamental_catalog = FundamentalCatalog.Four
            print(int(env.fundamental_catalog))
            ```

            Output:
            ```bash
            4
            ```
        """
        ...

class TimeInterface:
    """Time conversion utilities and time constants access.

    ds50 values are days since 1950-01-01 00:00:00 in the given time scale.
    """

    def __init__(self, file_name: str | None = None) -> None:
        """Create an interface and optionally load time constants."""
        ...

    @property
    def info(self) -> str:
        """DLL info string (version, build date, platform).

        Example:
            ```python
            from saal import MainInterface, TimeInterface

            ti = TimeInterface()
            print(MainInterface.DLL_VERSION in ti.info)
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    def ymd_components_to_ds50(
        self,
        year: int,
        month: int,
        day: int,
        hour: int,
        minute: int,
        second: float,
    ) -> float:
        """Convert UTC date/time components to ds50 UTC.

        Units: returns days since 1950-01-01 00:00:00 UTC.

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            ds50 = ti.ymd_components_to_ds50(1956, 1, 1, 0, 0, 0.0)
            print(f"{ds50:.1f}")
            ```

            Output:
            ```bash
            2192.0
            ```
        """
        ...

    def ds50_to_ymd_components(self, ds50_utc: float) -> tuple[int, int, int, int, int, float]:
        """Convert ds50 UTC to (year, month, day, hour, minute, second)."""
        ...

    def dtg_to_ds50(self, dtg: str) -> float:
        """Convert a DTG string to ds50 UTC."""
        ...

    def ds50_to_dtg20(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 20 characters."""
        ...

    def ds50_to_dtg19(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 19 characters."""
        ...

    def ds50_to_dtg17(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 17 characters."""
        ...

    def ds50_to_dtg15(self, ds50_utc: float) -> str:
        """Format ds50 UTC as a DTG string with 15 characters."""
        ...

    def year_doy_to_ds50(self, year: int, doy: float) -> float:
        """Convert (year, day-of-year) to ds50 UTC."""
        ...

    def ds50_to_year_doy(self, ds50_utc: float) -> tuple[int, float]:
        """Convert ds50 UTC to (year, day-of-year)."""
        ...

    def utc_to_ut1(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 UT1 using loaded time constants.

        If no timing constants are loaded, the input is returned unchanged.
        Units: days since 1950-01-01 00:00:00 (UTC in, UT1 out).

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            ut1 = ti.utc_to_ut1(2192.0)
            print(f"{ut1:.1f}")
            ```

            Output:
            ```bash
            2192.0
            ```
        """
        ...

    def utc_to_tai(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 TAI using loaded time constants."""
        ...

    def tai_to_utc(self, ds50_tai: float) -> float:
        """Convert ds50 TAI to ds50 UTC using loaded time constants."""
        ...

    def utc_to_tt(self, ds50_utc: float) -> float:
        """Convert ds50 UTC to ds50 TT using loaded time constants."""
        ...

    def tai_to_ut1(self, ds50_tai: float) -> float:
        """Convert ds50 TAI to ds50 UT1 using loaded time constants."""
        ...

    def load_constants(self, path: str) -> None:
        """Load timing constants from a file."""
        ...

    def load_time_constants(self, path: str) -> None:
        """Load timing constants from a file."""
        ...

    def get_fk4_greenwich_angle(self, ds50_ut1: float) -> float:
        """Compute Greenwich right ascension using the FK4 catalog.

        Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.

        Example:
            ```python
            import math
            from saal import TimeInterface

            ti = TimeInterface()
            ang = ti.get_fk4_greenwich_angle(2192.0)
            print(math.isfinite(ang))
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    def get_fk5_greenwich_angle(self, ds50_ut1: float) -> float:
        """Compute Greenwich right ascension using the FK5 catalog.

        Units: returns radians; input is days since 1950-01-01 00:00:00 UT1.

        Example:
            ```python
            import math
            from saal import TimeInterface

            ti = TimeInterface()
            ang = ti.get_fk5_greenwich_angle(2192.0)
            print(math.isfinite(ang))
            ```

            Output:
            ```bash
            True
            ```
        """
        ...

    @property
    def constants_loaded(self) -> bool:
        """Return whether timing constants are loaded."""
        ...

    def time_constants_loaded(self) -> bool:
        """Return whether timing constants are loaded.

        Example:
            ```python
            from saal import TimeInterface

            ti = TimeInterface()
            print(ti.time_constants_loaded())
            ```

            Output:
            ```bash
            False
            ```
        """
        ...

class SGP4Interface:
    """Access SGP4 propagation helpers."""

    def __init__(self) -> None: ...
    @property
    def info(self) -> str: ...
    def load(self, sat_key: int) -> None: ...
    def remove(self, sat_key: int) -> None: ...
    def clear(self) -> None: ...
    def get_count(self) -> int: ...
    def get_position_velocity_lla(
        self, sat_key: int, ds50_utc: float
    ) -> tuple[float, list[float], list[float], list[float]]: ...
    def get_position_velocity(self, sat_key: int, ds50_utc: float) -> tuple[list[float], list[float]]: ...
    def get_lla(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_position(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_full_state(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_equinoctial(self, sat_key: int, ds50_utc: float) -> list[float]: ...
    def get_ephemeris(
        self,
        sat_key: int,
        start: float,
        stop: float,
        step: float,
        frame: SGP4OutputEphemerisFrame,
    ) -> list[float]: ...
    def array_to_ephemeris(
        self,
        xa_tle: list[float],
        start: float,
        stop: float,
        step: float,
        frame: SGP4OutputEphemerisFrame,
    ) -> list[float]: ...
    def fit_xp_array(
        self,
        epoch: float,
        posvel: list[float],
        ballistic_coefficient: float | None,
        srp_coefficient: float | None,
    ) -> list[float]: ...
    def fit_sgp4_array(
        self,
        epoch: float,
        posvel: list[float],
        b_star: float | None,
    ) -> list[float]: ...
    def get_positions_velocities(self, sat_keys: list[int], ds50_utc: float) -> list[float]: ...
    def set_license_directory(self, lic_file_path: str) -> None: ...
    def get_license_directory(self) -> str: ...
    def reepoch_tle(self, sat_key: int, re_epoch_ds50_utc: float) -> tuple[str, str]: ...

class ParsedB3:
    """Parsed representation of a B3 observation."""

    def __init__(self) -> None: ...

    classification: Classification
    norad_id: int
    sensor_number: int
    epoch: float
    elevation: float | None
    declination: float | None
    azimuth: float | None
    right_ascension: float | None
    range: float | None
    range_rate: float | None
    year_of_equinox: float | None
    elevation_rate: float | None
    azimuth_rate: float | None
    range_acceleration: float | None
    observation_type: B3Type
    track_position: PositionInTrack
    association_status: AssociationStatus
    site_tag: int
    spadoc_tag: int
    position: list[float] | None

    def get_line(self) -> str: ...

class ObsInterface:
    """Access observation parsing helpers."""

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def parse_line(self, line: str) -> ParsedB3: ...

class ParsedSensor:
    """Parsed representation of a sensor."""

    key: int
    number: int
    minimum_range: float | None
    maximum_range: float | None
    range_rate_limit: float | None
    apply_range_limits: bool
    mobile: bool
    latitude: float | None
    longitude: float | None
    altitude: float | None
    astronomical_latitude: float
    astronomical_longitude: float
    azimuth_noise: float | None
    elevation_noise: float | None
    range_noise: float | None
    range_rate_noise: float | None
    azimuth_rate_noise: float | None
    elevation_rate_noise: float | None
    description: str | None

class SensorInterface:
    """Access sensor parsing and storage helpers."""

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def parse_key(self, sen_key: int) -> ParsedSensor: ...
    def parse_all(self) -> list[ParsedSensor]: ...
    def prune_missing_locations(self) -> None: ...
    def get_astronomical_ll(self, sen_key: int) -> list[float]: ...
    def get_lla(self, sen_key: int) -> list[float] | None: ...
    def get_keys(self, order: KeyOrder) -> list[int]: ...
    def load_card(self, card: str) -> None: ...
    def remove(self, sen_key: int) -> None: ...
    def get_count(self) -> int: ...
    def load_file(self, file_path: str) -> None: ...
    def clear(self) -> None: ...
    def get_arrays(self, sen_key: int) -> tuple[list[float], str]: ...

class ParsedTLE:
    """Parsed representation of a TLE."""

    def __init__(self) -> None: ...

    epoch: float
    norad_id: int
    inclination: float
    raan: float
    eccentricity: float
    argument_of_perigee: float
    mean_anomaly: float
    mean_motion: float
    ephemeris_type: TLEType
    element_set_number: int
    revolution_number: int
    designator: str | None
    classification: Classification
    mean_motion_1st_derivative: float | None
    mean_motion_2nd_derivative: float | None
    b_star: float | None
    ballistic_coefficient: float | None
    srp_coefficient: float | None

    def get_lines(self, remove_nulls: bool = False) -> tuple[str, str]: ...

class TLEInterface:
    """Access TLE parsing and storage helpers."""

    def __init__(self) -> None: ...

    @property
    def info(self) -> str: ...
    def fix_blank_exponent_sign(self, line_1: str) -> str: ...
    def add_check_sums(self, line_1: str, line_2: str) -> tuple[str, str]: ...
    def lines_to_arrays(self, line_1: str, line_2: str) -> tuple[list[float], str]: ...
    def arrays_to_lines(self, xa_tle: list[float], xs_tle: str) -> tuple[str, str]: ...
    def get_check_sums(self, line_1: str, line_2: str) -> tuple[int, int]: ...
    def load_lines(self, line_1: str, line_2: str) -> int: ...
    def load_arrays(self, xa_tle: list[float], xs_tle: str) -> int: ...
    def load_file(self, file_path: str) -> int: ...
    def clear(self) -> None: ...
    def remove(self, sat_key: int) -> None: ...
    def get_count(self) -> int: ...
    def get_keys(self, order: KeyOrder) -> list[int]: ...
    def get_lines(self, sat_key: int) -> tuple[str, str]: ...
    def get_arrays(self, sat_key: int) -> tuple[list[float], str]: ...
    def parse_lines(self, line_1: str, line_2: str) -> ParsedTLE: ...

__all__ = [
    "MainInterface",
    "AstroInterface",
    "DuplicateKeyMode",
    "KeyMode",
    "KeyOrder",
    "ElsetKeyMode",
    "ElementType",
    "TLEType",
    "Classification",
    "B3Type",
    "PositionInTrack",
    "AssociationStatus",
    "FundamentalCatalog",
    "SGP4OutputEphemerisFrame",
    "EnvironmentInterface",
    "SGP4Interface",
    "ObsInterface",
    "ParsedB3",
    "SensorInterface",
    "ParsedSensor",
    "TimeInterface",
    "TLEInterface",
    "ParsedTLE",
]
