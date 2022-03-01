#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <unordered_set>


template<size_t N>
struct std::hash<std::array<int, N>> {
    std::size_t operator()(const std::array<int, N>& banks) const noexcept {
        size_t hash{ 0 };
        for (const auto& bank : banks) {
            hash ^= std::hash<int>()(bank);
        }
        return hash;
    }
};

template<size_t N>
void redistribute(std::array<int, N>& banks) {
    const auto most_blocks_index{ std::ranges::max_element(banks) - banks.begin() };

    int blocks_left{ banks[most_blocks_index] };
    banks[most_blocks_index] = 0;
    size_t current_index{ (most_blocks_index + 1) % banks.size() };
    while (blocks_left > 0) {
        ++banks[current_index];
        --blocks_left;

        ++current_index;
        current_index %= banks.size();
    }
}

int main() {
    std::array<int, 16> banks{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    int index{ 0 };
    int blocks;
    while (file >> blocks) {
        banks[index] = blocks;
        ++index;
    }
    file.close();


    std::unordered_set<std::array<int, 16>> seen_banks;

    int redistributions{ 0 };
    while (seen_banks.insert(banks).second) {
        redistribute(banks);
        ++redistributions;
    }
    std::cout << "Completed redistribution cycles: " << redistributions << '\n';

    const std::array goal{ banks };
    redistribute(banks);

    int cycles{ 1 };
    while (banks != goal) {
        redistribute(banks);
        ++cycles;
    }
    std::cout << "Cycle size: " << cycles << '\n';


    return EXIT_SUCCESS;
}
