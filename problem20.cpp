#include <algorithm>
#include <cmath>
#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

using namespace std;

struct BigInteger {
  vector<int> digits;

  BigInteger(int initialValue) : digits(1, 0) { add(initialValue); }

  static BigInteger factorial(int n) {
    BigInteger f(1);
    for (int i = 2; i <= n; i++) {
      f.multiply(i);
    }
    return f;
  }

  void add(int n) {
    int remainder = 0;
    int i = 0;
    while (n > 0 || remainder > 0) {
      int digit = n % 10;
      n /= 10;
      if (i == digits.size()) {
        digits.push_back(0);
      }
      int sum = digits[i] + digit + remainder;
      digits[i] = sum % 10;
      remainder = sum / 10;
      i++;
    }
  }

  void multiply(int n) {
    int remainder = 0, i = 0, width = digits.size();
    while (i < width || remainder > 0) {
      if (i == digits.size()) {
        digits.push_back(0);
      }
      int step = n * digits[i] + remainder;
      digits[i] = step % 10;
      remainder = step / 10;
      i++;
    }
  }

  string toString() {
    string s;
    for (auto it = digits.rbegin(); it != digits.rend(); ++it) {
      s.append(to_string(*it));
    }
    return s;
  }

  long sumOfDigits() {
    long sum = 0;
    for (int d : digits) {
      sum += d;
    }
    return sum;
  }
};

void do_add() {
  BigInteger a(0);
  a.add(5000);
  a.add(5000);
  cout << "a " << a.toString() << endl;
}

void do_multiply() {
  BigInteger a(1);
  a.multiply(1);
  a.multiply(123456);
  a.multiply(123);
  cout << "a " << a.toString() << endl;
}

void do_factorial() {
  BigInteger f = BigInteger::factorial(100);
  cout << "100! = " << f.toString() << endl;
  cout << "sum of digits of 100! = " << f.sumOfDigits() << endl;
}

int main() {
  do_add();
  do_multiply();
  do_factorial();
}