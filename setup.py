#!/usr/bin/env python
# -*- coding:utf-8 -*-

import setuptools
import os

with open('README.md', 'r') as fh:
    long_description = fh.read()

setuptools.setup(
    name='no_proto',
    version='0.2.0',
    author='yumohc',
    author_email='yumohc@163.com',
    description='no proto serialize tools',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://www.dnnmind.com/',
    packages=["no_proto"],
    package_data={"no_proto": ["*.pyd", "*.so", "*.pyi"]},
    classifiers=[
    ],
    python_requires='>=3.4',
)
