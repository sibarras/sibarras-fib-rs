#!/usr/bin/env python
from setuptools import dist
dist.Distribution().fetch_build_eggs(["setuptools_rust"])
from setuptools import setup
from setuptools_rust import Binding, RustExtension

# with open("./version.py", 'rt') as vr:
#     version = vr.read().split("=")[1].replace("'", "")

setup(
    name="sibarras-fib-rs",
    version='0.1',
    # author="Samuel Ibarra",
    # author_email="ing.samuelibarra@gmail.com",
    # description="Calculates a fibonacci number",
    # long_description=long_description,
    # long_description_content_type="text/markdown",
    # url="https://github.com/sibarras/sibarras-fib-rs.git",
    rust_extensions=[
        RustExtension(
            ".sibarras_fib_rs.sibarras_fib_rs",
            path="Cargo.toml", binding=Binding.PyO3
    )],
    packages=["sibarras_fib_rs"],
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    zip_safe=False,
    entry_points={
        'console_scripts': [
            'fib-number = flitton_fib_rs.fib_number_command:fib_number_command',
        ],
    },
)