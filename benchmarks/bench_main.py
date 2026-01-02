from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

from saal import DuplicateKeyMode, KeyMode, MainInterface


@pytest.fixture(scope="module")
def main_iface() -> MainInterface:
    return MainInterface()


def test_bench_main_info(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    benchmark(lambda: main_iface.info)


def test_bench_main_get_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    benchmark(lambda: main_iface.key_mode)


def test_bench_main_set_duplicate_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    def run() -> None:
        main_iface.duplicate_key_mode = DuplicateKeyMode.ReturnKey
        main_iface.duplicate_key_mode = DuplicateKeyMode.ReturnZero
        main_iface.reset_key_mode()

    benchmark(run)


def test_bench_main_set_key_mode(benchmark: BenchmarkFixture, main_iface: MainInterface) -> None:
    def run() -> None:
        main_iface.key_mode = KeyMode.NoDuplicates
        main_iface.key_mode = KeyMode.DirectMemoryAccess
        main_iface.reset_key_mode()

    benchmark(run)
