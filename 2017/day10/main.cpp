#include <array>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

template<int N>
constexpr std::array<int, N> build_list() {
    std::array<int, N> list{};
    for (auto i{ 0 }; i < N; ++i) {
        list[i] = i;
    }
    return list;
}

constexpr auto N{ 256 };
constexpr auto ROUNDS{ 64 };

const std::vector PART1_INPUT{ 183, 0, 31, 146, 254, 240, 223, 150, 2, 206, 161, 1, 255, 232, 199, 88 };
const std::string INPUT_STRING{ "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88" };

template<int N>
constexpr void reverse(std::array<int, N>& list, const int start, const int length) {
    const int end_index = (start + length - 1) % N;
    for (int i = 0; i < length/2; ++i) {
        int end = end_index - i;
        if (end < 0) {
            end += N;
        }

        std::swap(list[(start + i) % N], list[end]);
    }
}

template<int N>
constexpr std::pair<int, int> knot_hash_round(
    std::array<int, N>& list, const std::vector<int>& input,
    const int initial_current_position = 0,
    const int initial_skip_size = 0
) {
    auto current_position{ initial_current_position };
    auto skip_size{ initial_skip_size };
    for (const int length : input) {
        reverse<N>(list, current_position, length);

        current_position += length + skip_size;
        current_position %= N;
        ++skip_size;
    }

    return { current_position, skip_size };
}

template<int N, int M>
constexpr std::array<int, N / M> densify(const std::array<int, N>& list) {
    std::array<int, N / M> result{};
    for (std::size_t i = 0; i < result.size(); ++i) {
        int hash{ 0 };
        for (int j = 0; j < M; ++j) {
            hash ^= list[i * M + j];
        }
        result[i] = hash;
    }
    return result;
}

int main() {
    std::array list{ build_list<N>() };

    // Part 1
    knot_hash_round<N>(list, PART1_INPUT);
    std::cout << "[Part 1] Result: " << list[0] * list[1] << '\n';

    // Part 2

    // Reset list
    list = build_list<N>();

    // Build input
    std::vector<int> input{};
    for (const char c : INPUT_STRING) {
        input.push_back(c);
    }
    for (const int l : { 17, 31, 73, 47, 23 }) {
        input.push_back(l);
    }

    // Run the hashing rounds
    int current_position{ 0 };
    int skip_size{ 0 };
    for (int i = 0; i < ROUNDS; ++i) {
        const auto [res_cp, res_ss] = knot_hash_round<N>(list, input, current_position, skip_size);

        current_position = res_cp;
        skip_size = res_ss;
    }

    // Make it dense
    const auto dense_hash{ densify<N, 16>(list) };
    // Print in hex
    std::cout << "[Part 2] Hash: ";
    for (const int c : dense_hash) {
        std::cout << std::setw(2) << std::setfill('0') << std::hex << c;
    }
    std::cout << '\n';

    return EXIT_SUCCESS;
}
