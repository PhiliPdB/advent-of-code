#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>


inline int plant_sum(const std::deque<bool>& state, const int start) {
    auto plant_containing_pots_sum{ 0 };
    for (int i = 0; i < static_cast<int>(state.size()); ++i) {
        if (state[i]) {
            plant_containing_pots_sum += start + i;
        }
    }
    return plant_containing_pots_sum;
}


int main() {
    std::deque<bool> state;
    std::vector<bool> rules(0b11111 + 1);

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::getline(file, line);
    for (int i = 15; i < static_cast<int>(line.length()); ++i) {
        state.push_back(line[i] == '#');
    }

    std::getline(file, line);
    while (std::getline(file, line)) {
        int rule{ 0 };
        for (const char c : line.substr(0, 5)) {
            rule <<= 1;
            rule |= c == '#';
        }

        rules[rule] = line[9] == '#';
    }
    file.close();

    // Start the iterations

    auto start{ 0 };
    auto pot_sum{ plant_sum(state, start) };
    auto generation{ 0 };

    auto last_increase{ 0 };
    auto current_increase{ pot_sum };


    std::deque<bool> new_state(state.size());
    while (last_increase != current_increase) {
        if (state[0]) {
            start -= 1;

            state.push_front(false);

            new_state.push_front(false);
        }
        if (state[state.size() - 1]) {
            state.push_back(false);

            new_state.push_back(false);
        }

        // Perform one plant generation
        for (int p = 0; p < static_cast<int>(state.size()); ++p) {
            int key{ 0 };
            for (int n = 0; n < 5; ++n) {
                if (p - 2 + n >= 0 && p - 2 + n < static_cast<int>(state.size())) {
                    key |= state[p - 2 + n] << (4 - n);
                }
            }
            
            new_state[p] = rules[key];
        }

        // Swap the new state with the current one
        std::swap(state, new_state);


        last_increase = current_increase;
        current_increase = plant_sum(state, start) - pot_sum;
        pot_sum += current_increase;

        ++generation;
        if (generation == 20) {
            // Part 1 answer
            std::cout << "Plant pot sum after 20 generations: " << pot_sum << '\n';
        }
    }

    // Calculations for part 2
    constexpr auto iterations = 50'000'000'000;

    const auto sum_after_iterations{ pot_sum + last_increase * (iterations - generation)};
    std::cout << "Plant pot sum after " << iterations << " generations: " << sum_after_iterations << '\n';


    return EXIT_SUCCESS;
}
