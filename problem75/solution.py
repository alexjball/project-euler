# %%

from math import sqrt, floor, gcd

is_coprime = lambda a, b: gcd(a, b) == 1
perimeter = lambda n, m, k=1: 2 * m * k * (m + n)

l_max = 15 * 10 ** 5
m_max = floor(sqrt(l_max / 2))
n_max = floor(l_max / 2)
count_by_perimeter = [0] * (l_max + 1)

k = 1
results = []
count = 0
for n in range(1, n_max):
    for m in range(n + 1, m_max):
        at_least_one_even = n % 2 == 0 or m % 2 == 0
        l = base = perimeter(n, m)
        if at_least_one_even and is_coprime(n, m) and l < l_max:
            while l < l_max:
                count_by_perimeter[l] += 1
                l += base

print(sum(filter(lambda count: count == 1, count_by_perimeter)))
