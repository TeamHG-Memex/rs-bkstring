import os.path
from setuptools import setup
from rust_ext import build_rust_cmdclass, install_lib_including_rust


setup(
    name = 'bktree',
    version = '0.1.0',
    cmdclass = {
        'build_rust': build_rust_cmdclass('../Cargo.toml'),
        'install_lib': install_lib_including_rust,
    },
    packages = ['bktree'],
    zip_safe=False
)
