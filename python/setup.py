import os.path
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name = 'bktree',
    version = '0.1.0',
    rust_extensions=[RustExtension(
        name='bktree',
        path='../Cargo.toml',
        binding=Binding.PyO3)],
    install_requires=[
        'setuptools_rust'
    ],
    zip_safe=False
)
