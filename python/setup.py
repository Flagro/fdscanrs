from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="fdscanrs",
    version="0.1.0",
    packages=["fdscanrs"],
    rust_extensions=[RustExtension("fdscanrs.fdscanrs", binding="pyo3")],
    zip_safe=False,
)
