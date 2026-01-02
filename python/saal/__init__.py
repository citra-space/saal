from __future__ import annotations

import os
import logging
from importlib import resources

_logger = logging.getLogger(__name__)


def _set_asset_directory() -> None:

    asset_dir = os.getenv("SAAL_ASSET_DIRECTORY")
    if asset_dir is None:
        pkg_dir = resources.files("saal") / "assets"
        _logger.debug("Setting SAAL_ASSET_DIRECTORY to %s", pkg_dir)
        os.environ.setdefault("SAAL_ASSET_DIRECTORY", str(pkg_dir))
    elif not os.path.exists(asset_dir):
        raise FileNotFoundError(f"SAAL_ASSET_DIRECTORY '{asset_dir}' does not exist.")


_set_asset_directory()
from ._saal import (  # type: ignore  # noqa: E402
    AstroInterface,
    MainInterface,
    EnvironmentInterface,
    SGP4Interface,
    TimeInterface,
    DuplicateKeyMode,
    KeyMode,
    ElsetKeyMode,
    ElementType,
    FundamentalCatalog,
    SGP4OutputEphemerisFrame,
    TLEInterface,
    ParsedTLE,
    TLEType,
    KeyOrder,
    Classification,
    ObsInterface,
    SensorInterface,
    ParsedB3,
    ParsedSensor,
    B3Type,
    PositionInTrack,
    AssociationStatus,
)

__all__ = [
    "MainInterface",
    "AstroInterface",
    "DuplicateKeyMode",
    "KeyMode",
    "ElsetKeyMode",
    "ElementType",
    "FundamentalCatalog",
    "SGP4OutputEphemerisFrame",
    "EnvironmentInterface",
    "SGP4Interface",
    "TimeInterface",
    "TLEInterface",
    "ParsedTLE",
    "TLEType",
    "KeyOrder",
    "Classification",
    "ObsInterface",
    "ParsedB3",
    "B3Type",
    "PositionInTrack",
    "AssociationStatus",
    "SensorInterface",
    "ParsedSensor",
]
