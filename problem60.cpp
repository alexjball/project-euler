#include <bits/stdc++.h>
#include <cassert>
#include <chrono>
#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <utility>
#include <vector>

using namespace std::chrono;
using namespace std;

/**
 * Find the lowest sum of 5 primes such that any concatination of primes is also
 * prime.
 *
 * Consider a graph formed by a vertex for each prime and an edge between pairs
 * of primes that have this property. Sets of larger primes with the
 * concatenation property form k-cliques in this graph. So we want to find the
 * 5-clique with the smallest sum.
 *
 * The approach:
 * 1) generate a list of all primes below some max (O(n^1.5))
 * 2) Find all 2-cliques with brute force (O(n^2))
 * 3) Expand the k-cliques by checking for higher primes that form pairs with
 *    all members of the k-cliques
 * 4) Repeat until 5 cliques are formed.
 * 5) Return the 5-clique with the smallest sum.
 */

typedef multimap<int, int> MapType;
typedef vector<int> Clique;

void log_time(string name, function<void()> work) {
  auto start = high_resolution_clock::now();
  work();
  auto end = high_resolution_clock::now();

  auto duration = duration_cast<milliseconds>(end - start);

  cout << name << " took " << duration.count() << " ms" << endl;
}

struct Problem60 {
  vector<int> primes;
  MapType pairs_by_min;
  vector<Clique> cliques;

  Clique smallest_clique;

  int maxPrime;
  int maxCliqueSize;

  Problem60(int maxPrime, int maxCliqueSize) {
    this->maxPrime = maxPrime;
    this->maxCliqueSize = maxCliqueSize;
  }

  Clique solve() {
    log_time("generate_primes", [this]() { primes = primes_below(maxPrime); });
    log_time("compute 2-cliques", [this]() { compute_2_cliques(); });
    for (int i = 3; i <= maxCliqueSize; i++) {
      log_time("expand cliques", [this]() { expand_cliques(); });
    }
    log_time("find smallest clique", [this]() { find_smallest_clique(); });

    return smallest_clique;
  }

  void find_smallest_clique() {
    int smallest_sum = INT_MAX;
    for (auto &clique : cliques) {
      int sum = accumulate(clique.begin(), clique.end(), 0);
      if (sum < smallest_sum) {
        smallest_clique = clique;
        smallest_sum = sum;
      }
    }
  }

  void expand_cliques() {
    vector<Clique> expanded_cliques;
    for (auto &clique : cliques) {
      MapType current_el_by_candidate;
      pair<MapType::iterator, MapType::iterator> ret;

      for (int el : clique) {
        ret = pairs_by_min.equal_range(el);
        for (MapType::iterator it = ret.first; it != ret.second; ++it) {
          current_el_by_candidate.insert(pair<int, int>(it->second, it->first));
        }
      }

      for (MapType::iterator it = current_el_by_candidate.begin();
           it != current_el_by_candidate.end();) {
        int candidate = it->first;

        if (current_el_by_candidate.count(candidate) == clique.size()) {
          Clique new_clique(clique);
          new_clique.push_back(candidate);
          expanded_cliques.push_back(new_clique);
        }

        do {
          it++;
        } while (it != current_el_by_candidate.end() && it->first == candidate);
      }
    }

    cliques = expanded_cliques;
  }

  void compute_2_cliques() {
    int p1, p2;
    for (int i = 0; i < primes.size(); i++) {
      for (int j = i + 1; j < primes.size(); j++) {
        p1 = primes[i], p2 = primes[j];
        if (is_concat_prime(p1, p2)) {
          pairs_by_min.insert(pair<int, int>(p1, p2));
          cliques.push_back(Clique({p1, p2}));
        }
      }
    }
  }

  static bool is_concat_prime(int a, int b) {
    // TODO: Use a table to more efficiently check for primeness
    return is_prime(stoull(to_string(a) + to_string(b))) &&
           is_prime(stoull(to_string(b) + to_string(a)));
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

  static bool is_prime(unsigned long long n) {
    unsigned long long limit = (unsigned long long)sqrt(n);
    for (unsigned long long i = 2; i <= limit; i++) {
      if (n % i == 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  Problem60 solution(10000, 5);

  Clique smallest = solution.solve();
  int sum = accumulate(smallest.begin(), smallest.end(), 0);
  cout << "min sum " << sum << endl << "clique [";
  for (int i : smallest) {
    cout << i << " ";
  }
  cout << "]" << endl;
}