#include <fstream>
#include <iostream>
#include <numeric>
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
    std::vector<int> input;
    while (file >> line) {
        input.push_back(std::stoi(line));
    }

    // Part 1
    const auto sum = std::accumulate(input.begin(), input.end(), 0);
    std::cout << "Resulting frequency: " << sum << '\n';

    // Part 2
    std::unordered_set<int> seen{};
    auto current_frequency = 0;

    while (true) {
        for (const auto value : input) {
            if (seen.insert(current_frequency).second) {
                current_frequency += value;
            } else {
                std::cout << "First seen twice: " << current_frequency << '\n';
                return EXIT_SUCCESS;
            }
        }
    }
}
