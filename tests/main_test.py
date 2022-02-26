#!/usr/bin/env python
"""Tests for `cal-opt` package."""
from cal_opt import main


def test_no_arguments() -> None:
    """It exits with no return and no error code."""
    result = main.cli()
    assert result is None
