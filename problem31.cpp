#include <iostream>
#include <vector>
#include <string>

using namespace std;

enum Coin {
    p200 = 0,
    p100 = 1,
    p50 = 2,
    p20 = 3,
    p10 = 4,
    p5 = 5,
    p2 = 6,
    p1 = 7,
};

const int number_of_coin_types = 8;
int coin_values[number_of_coin_types] = {200, 100, 50, 20, 10, 5, 2, 1};

int number_of_ways(int amount, Coin coin)
{
    if (amount == 0 || coin == p1) {
        return 1;
    }

    Coin next_coin = (Coin) (coin + 1);
    int rest = amount;
    int count = 0;
    while (rest >= 0) {
        count += number_of_ways(rest, next_coin);
        rest -= coin_values[coin];
    }
    return count;
}

int main(int argc, char *argv[])
{
    int amount = argc == 2 ? stoi(argv[1]) : 200;
    cout << "Number of ways to make " << amount << "p: " << number_of_ways(amount, p200) << endl;  
}
