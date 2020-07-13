#include <iostream>
#include <vector>
#include <string>
#include <cmath>
#include <utility>
#include <algorithm>
#include <set>

using namespace std;

typedef std::pair<int, int> DevisorSum;

int sum_of_divisors(int n) {
    int sum = 1;
    int max_divisor = (int) ceil(sqrt(n));
    for (int i = 2; i <= max_divisor; i++) {
        if (n % i == 0) {
            sum += i + n / i;
        }
    }
    return sum;
}


int sum_of_amicable_below(int max_number) {
    int sum = 0;

    for (int n = 1; n < max_number; n++) {
        int divisor_sum = sum_of_divisors(n);
        if (divisor_sum < max_number && divisor_sum > n && n == sum_of_divisors(divisor_sum)) {
            sum += divisor_sum + n;
        }
    }
    return sum;
}

int main(int argc, char *argv[])
{
    int max = argc > 1 ? stoi(argv[1]) : 10000;
    cout << "d(284): " << sum_of_divisors(284) << endl;
    cout << "d(220): " << sum_of_divisors(220) << endl;
    cout << "sum_of_amicable_below(" << max << "): " << sum_of_amicable_below(max) << endl;
}
