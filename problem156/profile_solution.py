import pstats, cProfile

from compute import compute_s, s, CountingDigits

cProfile.runctx("compute_s(1, 3e6)", globals(), locals(), "Profile-class.prof")

stats = pstats.Stats("Profile-class.prof")
stats.strip_dirs().sort_stats("time").print_stats()

cProfile.runctx("s(1, max_s=3e6)", globals(), locals(), "Profile-cfun.prof")

stats = pstats.Stats("Profile-cfun.prof")
stats.strip_dirs().sort_stats("time").print_stats()


def bench():
    cd = CountingDigits(1)
    cd.compute(max_s=646663393)


cProfile.runctx("bench()", globals(), locals(), "Profile-cclass.prof")

stats = pstats.Stats("Profile-cclass.prof")
stats.strip_dirs().sort_stats("time").print_stats()
