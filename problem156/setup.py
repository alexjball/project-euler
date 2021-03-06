from setuptools import setup
from Cython.Build import cythonize

setup(
    name="Project Euler",
    ext_modules=cythonize("*.pyx", language_level="3"),
    zip_safe=False,
)
