#include <algorithm>
#include <iostream>
#include <list>
#include <numeric>
#include <vector>

constexpr int PLAYERS = 430;
constexpr int LAST_MARBLE_WORTH = 71'588;


long long winning_score(const int last_marble_worth) {
    std::vector<long long> scores;
    scores.resize(PLAYERS, 0);

    std::list marbles{ 0, 1 };
    auto current_marble = std::prev(marbles.end(), 1);
    for (int next_marble = 2; next_marble <= last_marble_worth; ++next_marble) {
        if (next_marble % 23 == 0) {
            for (int i = 0; i < 7; ++i) {
                if (current_marble == marbles.begin()) {
                    current_marble = marbles.end();
                }
                std::advance(current_marble, -1);
            }

            scores[next_marble % PLAYERS] += next_marble + *current_marble;

            current_marble = marbles.erase(current_marble);
        } else {
            if (std::next(current_marble) == marbles.end()) {
                current_marble = std::next(marbles.begin());
            } else {
                std::advance(current_marble, 2);
            }

            current_marble = marbles.insert(current_marble, next_marble);
        }
    }

    return std::ranges::max(scores);
}


int main() {
    std::cout << "[Part 1] Winning score: " << winning_score(LAST_MARBLE_WORTH) << '\n';
    std::cout << "[Part 2] Winning score: " << winning_score(LAST_MARBLE_WORTH * 100) << '\n';

    return EXIT_SUCCESS;
}
