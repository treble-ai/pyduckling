# -*- coding: utf-8 -*-
# -----------------------------------------------------------------------------
# Copyright (c) Treble.ai
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# -----------------------------------------------------------------------------

"""Python bindings for Haskell's duckling library."""


# Standard library imports
import json
from typing import List

# Local imports
from .duckling import (init, stop, load_time_zones,
                       get_current_ref_time, parse_ref_time,
                       parse_lang, default_locale_lang, parse_locale,
                       parse_dimensions, parse_text, Context, Dimension,
                       Locale, __version__, GHC_VERSION)

__version__
GHC_VERSION
init
stop
load_time_zones
parse_ref_time
parse_locale
parse_lang
parse_text
parse_dimensions
default_locale_lang
get_current_ref_time
Context
Locale

# Start Haskell runtime
init()


def parse(text: str, context: Context, dimensions: List[Dimension],
          with_latent: bool = False) -> dict:
    """
    Parse a text into a structured format.

    Parameters
    ----------
    text: str
        Text to parse.
    context: Context
        Reference time and locale information
    dimensions: List[Dimension]
        List of dimensions to parse
    with_latent: bool
        When set, includes less certain parses, e.g. "7" as an hour of the day

    Returns
    -------
    result: dict
        Dictionary that contains the parsed information.
    """
    result = parse_text(text, context, dimensions, with_latent)
    return json.loads(result)
