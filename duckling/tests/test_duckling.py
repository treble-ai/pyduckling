# -*- coding: utf-8 -*-
# -----------------------------------------------------------------------------
# Copyright (c) Treble.ai
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# -----------------------------------------------------------------------------

"""Tests for pyduckling library."""

# Pytest imports
import pytest

# Third-party imports
import pendulum

# Local imports
from duckling import (load_time_zones, get_current_ref_time, parse_ref_time,
                      parse_lang, default_locale_lang, parse_locale,
                      parse_dimensions, parse, Context)


@pytest.fixture
def time_zones():
    tzdb = load_time_zones("/usr/share/zoneinfo")
    return tzdb


def test_load_time_zones():
    tzdb = load_time_zones("/usr/share/zoneinfo")
    assert tzdb is not None


def test_get_current_ref_time(time_zones):
    # Remove timezone information
    bog_now = pendulum.now('America/Bogota').naive().replace(microsecond=0)
    ref_time = get_current_ref_time(time_zones, 'America/Bogota')
    # UTC conversion in required to recover the actual datetime
    this_ref_time = pendulum.parse(ref_time.iso8601).in_tz('UTC').naive()
    this_ref_time = this_ref_time.replace(microsecond=0)
    assert bog_now == this_ref_time

    # Function should fallback to UTC if the timezone does not exist
    utc_now = pendulum.now('UTC').naive().replace(microsecond=0)
    ref_time = get_current_ref_time(time_zones, 'Continent/Country')
    this_ref_time = pendulum.parse(ref_time.iso8601).in_tz('UTC').naive()
    this_ref_time = this_ref_time.replace(microsecond=0)
    assert this_ref_time == utc_now


def test_parse_ref_time(time_zones):
    bog_now = pendulum.now('America/Bogota').replace(microsecond=0)
    ref_time = parse_ref_time(
        time_zones, 'America/Bogota', bog_now.int_timestamp)
    # UTC conversion in required to recover the actual datetime
    this_ref_time = pendulum.parse(ref_time.iso8601).in_tz('UTC').naive()
    this_ref_time = this_ref_time.replace(microsecond=0)
    assert bog_now.naive() == this_ref_time

    # Initialize any date
    dt = pendulum.datetime(1996, 2, 22, 9, 22, 3, 0, tz="Europe/Madrid")
    # bog_dt = dt.in_tz('America/Bogota')
    ref_time = parse_ref_time(
        time_zones, 'Europe/Madrid', dt.int_timestamp)
    # UTC conversion in required to recover the actual datetime
    this_ref_time = pendulum.parse(ref_time.iso8601).in_tz('UTC').naive()
    this_ref_time = this_ref_time.replace(microsecond=0)
    assert dt.naive() == this_ref_time

    # Function should fallback to UTC if the timezone does not exist
    pst_now = pendulum.now('America/Los_Angeles').replace(microsecond=0)
    ref_time = parse_ref_time(
        time_zones, 'Continent/Country', pst_now.int_timestamp)
    # UTC conversion in required to recover the actual datetime
    this_ref_time = pendulum.parse(ref_time.iso8601).in_tz('UTC').naive()
    this_ref_time = this_ref_time.replace(microsecond=0)
    assert pst_now.in_tz('UTC').naive() == this_ref_time


def test_parse_lang():
    # Function call should be case-insensitive
    lang_es = parse_lang('es')
    assert lang_es.name == 'ES'

    lang_pt = parse_lang('PT')
    assert lang_pt.name == 'PT'

    # Function should default to EN, when the language does not exists
    lang_any = parse_lang('UU')
    assert lang_any.name == 'EN'


def test_default_locale_lang():
    lang_es = parse_lang('ES')
    default_locale = default_locale_lang(lang_es)
    assert default_locale.name == 'ES_XX'


def test_parse_locale():
    lang_es = parse_lang('ES')
    default_locale = default_locale_lang(lang_es)

    # Parse Language + Country locale
    locale = parse_locale('ES_CO', default_locale)
    assert locale.name == 'ES_CO'

    # Parse Country locale
    locale = parse_locale('CO', default_locale)
    assert locale.name == 'ES_XX'


def test_parse_dimensions():
    valid_dimensions = ["amount-of-money", "credit-card-number", "distance",
                        "duration", "email", "number", "ordinal",
                        "phone-number", "quantity", "temperature",
                        "time", "time-grain", "url", "volume"]

    # All dimensions should be parsed
    output_dims = parse_dimensions(valid_dimensions)
    assert len(output_dims) == len(valid_dimensions)

    invalid_dimensions = ["amount-of-money", "dim1", "credit-card-number",
                          "dim2", "distance", "dim3"]

    # Valid-only dimensions should be parsed
    output_dims = parse_dimensions(invalid_dimensions)
    assert len(output_dims) == len(invalid_dimensions) - 3


def test_parse(time_zones):
    bog_now = pendulum.now('America/Bogota').replace(microsecond=0)
    ref_time = parse_ref_time(
        time_zones, 'America/Bogota', bog_now.int_timestamp)
    lang_es = parse_lang('ES')
    default_locale = default_locale_lang(lang_es)
    locale = parse_locale('ES_CO', default_locale)

    context = Context(ref_time, locale)
    dimensions = ['time', 'duration']
    dims = parse_dimensions(dimensions)

    # Test time periods
    result = parse('En dos semanas', context, dims, False)
    next_time = result[0]['value']['value']
    next_time = pendulum.parse(next_time)
    assert next_time == bog_now.add(weeks=2).start_of('day')

    # Test distance units
    dimensions = ['distance']
    dims = parse_dimensions(dimensions)
    result = parse('3 km', context, dims, False)
    info = result[0]['value']
    value = info['value']
    unit = info['unit']
    assert value == 3
    assert unit == 'kilometre'

    # Test volume units
    dimensions = ['volume']
    dims = parse_dimensions(dimensions)
    result = parse('5 litros de leche', context, dims, False)
    info = result[0]['value']
    value = info['value']
    unit = info['unit']
    assert value == 5
    assert unit == 'litre'
