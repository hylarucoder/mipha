import platform

from mipha.native.subclassing import Subclassable

PYPY = platform.python_implementation() == "PyPy"


class SomeSubClass(Subclassable):
    pass


def test_subclassing():
    if not PYPY:
        a = SomeSubClass()
        _b = str(a) + repr(a)
