import bkmath


def test_add():
    assert bkmath.add(8, 19) == 27
    assert bkmath.add(10, -20) == -10
    assert bkmath.add(0, 0) == 0


def test_multiply():
    assert bkmath.multiply(5, 6) == 30
