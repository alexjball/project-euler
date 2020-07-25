#include <cassert>
#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

#define MAX_PRIME 1000

/**
 * Find the lowest sum of 5 primes such that any concatination of primes is also
 * prime. Primes with this property form a k-clique.
 *
 * The approach:
 * 1) generate a list of all primes below some max greater than the largest
 * prime in the solution set 2) Find all 2-cliques with brute force (O(n^2)) 3)
 * Find k+1-cliques by iterating over k-cliques and checking for primes (above
 * largest in clique) that form a k+1-clique 4) Proceed from 2 to 5-clique. If
 * there are no solutions, increase the max and start again.
 *
 * There is a clique data structure that stores the set of primes
 * The 2-cliques are stored as an adjacency list.
 *
 * Unverified Assumptions:
 * - We only need to store edges from smaller to larger primes
 * - We only need to check primes that are larger than the largest prime in a
 * clique. If there is a k-clique that can be expanded by adding a smaller
 * prime, then there is an overall-smaller k-clique which will be evaluated
 * later on in the k-iteration
 */
struct Clique {
  vector<int> el_index;
};

struct Solution {
  vector<int> primes;
  map<int, int> pairs_by_min;
  vector<vector<int>> concat_primes;
  int maxPrime;

  Solution(int maxPrime) { this->maxPrime = maxPrime; }

  void solve() {
    primes = primes_below(maxPrime);
    compute_2_cliques();
  }

  void compute_2_cliques() {
    for (int i = 0; i < primes.size(); i++) {
      for (int j = i + 1; j < primes.size(); j++) {
        if (is_concat_prime(primes[i], primes[j])) {
          // primes.push_back();
          // cliques.push_back(new vector({i, j}));
        }
      }
    }
  }

  static bool is_concat_prime(int a, int b) {
    return is_prime(stoi(to_string(a) + to_string(b))) &&
           is_prime(stoi(to_string(b) + to_string(a)));
  }

  static vector<int> primes_below(int max) {
    vector<int> primes;
    vector<bool> is_composite(max, false);
    for (int i = 2; i < max; i++) {
      if (!is_composite[i]) {
        primes.push_back(i);
      }
      int composite = i * 2;

      while (composite < max) {
        is_composite[composite] = true;
        composite += i;
      }
    }
    return primes;
  }

  static bool is_prime(int n) {
    int limit = (int)sqrt(n);
    for (int i = 2; i <= limit; i++) {
      if (n % i == 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  Solution solution(MAX_PRIME);

  assert(solution.is_prime(11));
  assert(!solution.is_prime(10));
}