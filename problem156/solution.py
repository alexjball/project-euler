from compute import CountingDigits
from time import time


class Logger:

    start = time()
    section_start = start
    section_start_n = 0

    def log(self, f, s, n):
        section_end = time()
        total_elapsed = section_end - self.start
        section_elapsed = section_end - self.section_start

        print(f"n = {n}, s = {s}, f = {f}")
        print(f"Total: {round(total_elapsed, 2)} s, {n / total_elapsed} n/s")
        print(
            f"Section: {round(section_elapsed, 2)} s, {(n - self.section_start_n) / section_elapsed} n/s"
        )

        self.section_start = section_end
        self.section_start_n = n


small_s = 3e6
medium_s = 646663393
# 1 less than s(1)
large_s = 22786974070
max_s = large_s
max_n = 10 ** 11

logger = Logger()

s = [0] * 10
for d in range(1, 10):
    print("===")
    print(f"d = {d}")
    solution = CountingDigits(d, logger.log)
    solution.compute(max_n=max_n)
    s[d] = solution.s
    print(f"s({d}) = {s[d]}")

total = sum(s)
print(f"sum of s(d) for 1 <= d <= 9: {total}")
