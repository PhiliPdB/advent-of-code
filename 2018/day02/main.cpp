#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <string>


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::vector<std::string> input;
    while (file >> line) {
        input.push_back(line);
    }

    // Part 1

    auto two_letter = 0;
    auto three_letter = 0;

    for (const auto& id: input) {
        std::unordered_map<char, int> counts;

        for (const auto c : id) {
            counts[c] += 1;
        }

        auto counted_two_letter = false;
        auto counted_three_leter = false;
        for (const auto& [_, val] : counts) {
            if (!counted_two_letter && val == 2) {
                two_letter += 1;
                counted_two_letter = true;
            }

            if (!counted_three_leter && val == 3) {
                three_letter += 1;
                counted_three_leter = true;
            }
        }
    }

    std::cout << "Checksum: " << two_letter * three_letter << '\n';

    // Part 2

    for (auto i = 0; i < static_cast<int>(input.size()) - 1; ++i) {
        const auto& id1{ input[i] };

        for (auto j = i + 1; j < static_cast<int>(input.size()); ++j) {
            const auto& id2{ input[j] };

            std::string common;
            auto differences = 0;
            for (auto k = 0; k < static_cast<int>(id1.size()); ++k) {
                if (id1[k] == id2[k]) {
                    common.push_back(id1[k]);
                } else {
                    differences += 1;
                }
            }

            if (differences == 1) {
                std::cout << "Common letters: " << common << '\n';
                return EXIT_SUCCESS;
            }
        }
    }

    return EXIT_FAILURE;
}
