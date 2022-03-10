import math

divfloor = lambda a, b: divmod(a, b)[0]
sign = lambda a: -1 if a < 0 else 1 if a > 0 else 0
mid = lambda a, b: math.floor((a + b) / 2)
samesign = lambda a, b: a * b >= 0


def f(n, d, k=None):
    if k is None:
        num_digits = math.floor(math.log10(n)) + 1 if n > 0 else 1
        return sum([f(n, d, i) for i in range(num_digits)])

    n_k = divfloor(n, 10 ** k) % 10

    complete_wraps = divfloor(n, 10 ** (k + 1)) * 10 ** k
    if n_k < d:
        current_wrap = 0
    elif n_k > d:
        current_wrap = 10 ** k
    else:
        current_wrap = n % 10 ** k + 1

    total = complete_wraps + current_wrap
    return total


def find_zero_int(min, max, f):
    assert sign(f(min)) != sign(f(max))

    start = (min, f(min))
    end = (max, f(max))

    if start[1] == 0:
        return start[0]
    elif end[1] == 0:
        return end[0]

    while start[0] < end[0] - 1:
        x = mid(start[0], end[0])
        f_x = f(x)
        if f_x == 0:
            return x
        elif not samesign(start[1], f_x):
            end = (x, f_x)
        else:
            start = (x, f_x)

    raise NoIntegerSolutionError(start, end)


class NoIntegerSolutionError(Exception):
    def __init__(self, left, right):
        self.left = left
        self.right = right
