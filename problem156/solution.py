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
large_s = 22786974071
max_s = medium_s
d = 1

logger = Logger()
solution = CountingDigits(d, logger.log)
solution.compute(max_s=max_s)
