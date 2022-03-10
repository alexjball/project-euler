import solver
from compute import CountingDigits
from time import time
import pytest

small_s = 3e6
medium_s = 646663393
# 1 less than s(1)
large_s = 22786974070
max_s = large_s
max_n = 10 * 10 ** 9


def test_f():
    assert solver.f(1, 1) == 1


def test_f_agrees_with_count():
    d = 1

    def assert_f(f, s, n):
        assert solver.f(n, d) == f

    c = CountingDigits(d, assert_f)
    c.compute(max_n=10 ** 6)


def test_f_exceeds_n_after_10_digits():
    d = 1
    for n in [10 ** k for k in range(12)]:
        f = solver.f(n, d)
        if n >= 10 ** 10:
            assert f > n
        else:
            assert f <= n


class TestFindZeroInt:
    def test_signs(self):
        f = lambda x: x
        with pytest.raises(AssertionError):
            solver.find_zero_int(2, 3, f)

    def test_endpoints(self):
        f = lambda x: x
        assert solver.find_zero_int(0, 5, f) == 0
        assert solver.find_zero_int(-5, 0, f) == 0

    def test_finds(self):
        f = lambda x: x
        assert solver.find_zero_int(-1, 1, f) == 0
        assert solver.find_zero_int(-10, 1, f) == 0

    def test_no_solution(self):
        f = lambda x: x * x - 2
        with pytest.raises(solver.NoIntegerSolutionError):
            solver.find_zero_int(1, 2, f)
