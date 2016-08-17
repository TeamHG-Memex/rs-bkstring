# Installation

This library is written in Rust and it has two build dependencies:

1. Rust nightly
2. Setuptools helper for Rust

Either download the [Rust
nightly](https://www.rust-lang.org/en-US/downloads.html#nightly) or run the
following command on Linux or OS X:
`curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly`.

Then, install the [setuptools
helper](https://github.com/novocaine/rust-python-ext): `pip install rust-ext`.

After installing dependencies, you can install this package:

`python setup.py install`

(Installing with `pip` doesn't work.)

# Examples

    >>> from bktree import BkTree
    >>> t = BkTree()
    >>> t.add_list(['foo1', 'foo2', 'bar1', 'bar2')
    >>> t.search('foo', 1)
    ['foo1', 'foo2']
    >>> t.search('bar', 1)
    ['bar1', 'bar2']
