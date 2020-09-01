# -*- coding: utf-8 -*-
# -----------------------------------------------------------------------------
# Copyright (c) Treble.ai
#
# Licensed under the terms of the MIT License
# (see LICENSE.txt for details)
# -----------------------------------------------------------------------------

"""Setup script for dbsession."""

# yapf: disable

# Standard library imports
import re
import os

# Third party imports
import toml
from setuptools import find_packages, setup
from setuptools_rust import Binding, RustExtension


HERE = os.path.abspath(os.path.dirname(__file__))
AUTHOR_REGEX = re.compile(r'(.*) <(.*@.*[.].*)>')


def get_metadata():
    """Get version from text file and avoids importing the module."""
    with open(os.path.join(HERE, 'Cargo.toml'), 'r') as f:
        data = toml.load(f)
    # version = data['package']['version']
    return data['package']


def get_description():
    """Get long description."""
    with open(os.path.join(HERE, 'README.md'), 'r') as f:
        data = f.read()
    return data


def get_author(metadata):
    author = metadata['authors'][0]
    match = AUTHOR_REGEX.match(author)
    name = match.group(1)
    email = match.group(2)
    return name, email


REQUIREMENTS = [
    'pendulum'
]

metadata = get_metadata()
name, email = get_author(metadata)

setup(
    name=metadata['name'],
    version=metadata['version'],
    license=metadata['license'],
    description=metadata['description'],
    long_description=get_description(),
    long_description_content_type='text/markdown',
    author=name,
    author_email=email,
    url=metadata['repository'],
    keywords=metadata['keywords'],
    packages=find_packages(exclude=['contrib', 'docs', 'tests*']),
    rust_extensions=[RustExtension("duckling.duckling", binding=Binding.PyO3)],
    package_data={
        'duckling': ['*.dll', '*.dylib', '*.so']
    },
    zip_safe=False,
    install_requires=REQUIREMENTS,
    include_package_data=True,
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8'
    ],
)
