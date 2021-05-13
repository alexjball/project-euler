#include <chrono>
#include <iostream>
#include <cstdio>
#include <utility>
#include <functional>

using namespace std::chrono;
using namespace std;

void log_time(string name, function<void()> work) {
  auto start = high_resolution_clock::now();
  work();
  auto end = high_resolution_clock::now();

  auto duration = duration_cast<milliseconds>(end - start);

  cout << name << " took " << duration.count() << " ms" << endl;
}

// This is about twice as slow as the Cython solution
struct DigitCounting {
  long f;
  long s;
  long n;
  int d;

  DigitCounting(int d): f(0), s(0), n(0), d(d) {}

  void compute(long max_s)  {
    while (true) {
      if (s > max_s) {
        break;
      }
      update_f();
      update_s();
      n += 1;
    }
  }

  void update_f() {
    long digits = n;
    int digit;
    while (digits > 0) {
      digit = digits % 10;
      digits /= 10;
      if (digit == d) {
        f += 1;
      }
    }
  }

  void update_s() {
    if (f == n) {
      s += f;
    }
  }
};

int main() {
  DigitCounting dc(1);
  // compute took 22353 ms
  // d = 1, f = 500000000, s = 1146663393, n = 500000001
  long max_s = 646663393;
  log_time("compute", [&]() mutable { dc.compute(max_s); });
  printf("d = %d, f = %ld, s = %ld, n = %ld\n", dc.d, dc.f, dc.s, dc.n);
}