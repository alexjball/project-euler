# This is about 30 times slower than the non-str solution
cpdef compute_with_str(d, max_n=float("inf"), max_s=1e12):
    d = str(d)
    cdef long n = 0
    cdef long f = 0
    cdef long s = 0
    while True:
        if n > max_n or s > max_s:
            break

        f += str(n).count(d)
        if n == f:
            s += f
        
        n += 1

    return (n, f, s)

cdef class CountingDigits:
    cdef public long f
    cdef public long s
    cdef public long n
    cdef public int d
    cdef observer

    def __init__(self, d, observer = None):
        self.d = d
        self.observer = observer

    cpdef compute(self, long max_s = -1, long max_n = -1):
        while True:
            if (max_s != -1 and self.s > max_s) or (max_n != -1 and self.n > max_n):
                break
            self.update_f()
            self.update_s()
            self.n += 1

    cdef update_f(self):
        cdef long digits = self.n
        cdef int digit
        while digits > 0:
            digit = digits % 10
            digits /= 10
            if digit == self.d:
                self.f += 1

    cdef update_s(self):
        if self.f == self.n:
            self.s += self.f
            if self.observer:
                self.observer(self.f, self.s, self.n)
    