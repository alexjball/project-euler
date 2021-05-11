import csv


class Solution:
    def combinations(self, values, k, start_index=0):
        """Enumerate combinations recursively

        For a list v with len(v) = n, the k combinations can be enumerated by
        looping over the position of the leftmost element in the set. For each
        position i of the leftmost element, enumerate that element joined to all
        combinations of v[i+1:end] choose k-1. when k is 0, enumerate a single empty
        array.

        Computing 4 choose 2

        [1, 2, 3, 4]
        *  *
        *     *
        *        *
            *  *
            *     *
            *  *
        """

        if k == 0:
            return [[]]
        all = []
        for i in range(start_index, len(values) - k + 1):
            combinations = self.combinations(values, k - 1, i + 1)
            for c in combinations:
                c.append(values[i])
            all.extend(combinations)
        return all

    def subset_pairs(self, values):
        ts = lambda x: tuple(sorted(x))
        pairs = set()
        for pair_size in range(2, len(values) + 1):
            for pair_union in self.combinations(values, pair_size):
                pair_union_set = set(pair_union)
                for subset_size in range(1, pair_size):
                    for b in [
                        ts(x) for x in self.combinations(pair_union, subset_size)
                    ]:
                        c = ts(pair_union_set.difference(b))
                        pairs.add(ts([b, c]))
        return list(pairs)

    def is_special_sum_set(self, values):
        unequal_sums = lambda b, c: sum(b) != sum(c)

        def larger_sets_larger_sums(b, c):
            if len(b) > len(c):
                return sum(b) > sum(c)
            elif len(c) > len(b):
                return sum(c) > sum(b)
            return True

        subset_pairs = self.subset_pairs(values)
        return all(
            [
                unequal_sums(b, c) and larger_sets_larger_sums(b, c)
                for b, c in subset_pairs
            ]
        )

    def load_sets(self):
        with open("./p105_sets.txt") as f:
            sets = [[int(x) for x in s] for s in csv.reader(f)]
        return sets

    def sum_of_special_sum_sets(self):
        total = 0
        for s in self.load_sets():
            if self.is_special_sum_set(s):
                print(True, s)
                total += sum(s)
            else:
                print(False, s)

        print(total)

    def equality_test_required(self, n):
        # Equality test is not required if
        # - sets are different size
        # - sets of equal sizes can be zipped up so that every element of one
        #   set is larger than the corresponding element in the other set

        equal_length = lambda b, c: len(b) == len(c)

        def larger_by_inequality(b, c):
            direction = None
            for v1, v2 in zip(b, c):
                new_direction = v2 > v1
                if direction is not None and new_direction != direction:
                    return False
                direction = new_direction
            return True

        pairs = self.subset_pairs(range(1, n + 1))
        equality_test_required = [
            equal_length(b, c) and not larger_by_inequality(b, c) for b, c in pairs
        ]
        total = sum(equality_test_required)
        print(f"n = {n}, {total} require equality test")


a = range(1, 5)
k = 2
s = Solution()
# c = s.combinations(a, k)
p = s.subset_pairs(a)
print(p)

not_a_special_sum_set = [81, 88, 75, 42, 87, 84, 86, 65]
special_sum_set = [157, 150, 164, 119, 79, 159, 161, 139, 158]

assert not s.is_special_sum_set(not_a_special_sum_set)
assert s.is_special_sum_set(special_sum_set)

# s.sum_of_special_sum_sets()
s.equality_test_required(4)
s.equality_test_required(7)
s.equality_test_required(12)
