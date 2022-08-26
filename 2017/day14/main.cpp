#include <array>
#include <iostream>
#include <string>
#include <vector>


template<int N, int M>
class KnotHash {
private:
    constexpr static int ROUNDS = 64;

public:
    constexpr static std::array<std::uint8_t, N / M> hash(const std::string_view input_string) {
        std::vector<int> input{};
        for (const char c : input_string) {
            input.push_back(c);
        }
        for (const int l : { 17, 31, 73, 47, 23 }) {
            input.push_back(l);
        }

        // Run the hashing rounds
        auto list{ build_list() };
        int current_position{ 0 };
        int skip_size{ 0 };
        for (int i = 0; i < ROUNDS; ++i) {
            const auto [res_cp, res_ss] = single_round(list, input, current_position, skip_size);

            current_position = res_cp;
            skip_size = res_ss;
        }

        // Make it dense
        const auto dense_hash{ densify(list) };

        return dense_hash;
    }

private:
    constexpr static std::pair<int, int> single_round(
        std::array<std::uint8_t, N>& list, const std::vector<int>& input,
        const int initial_current_position = 0,
        const int initial_skip_size = 0
    ) {
        auto current_position{ initial_current_position };
        auto skip_size{ initial_skip_size };
        for (const int length : input) {
            reverse(list, current_position, length);

            current_position += length + skip_size;
            current_position %= N;
            ++skip_size;
        }

        return { current_position, skip_size };
    }

    constexpr static void reverse(std::array<std::uint8_t, N>& list, const int start, const int length) {
        const int end_index = (start + length - 1) % N;
        for (int i = 0; i < length / 2; ++i) {
            int end = end_index - i;
            if (end < 0) {
                end += N;
            }

            std::swap(list[(start + i) % N], list[end]);
        }
    }

    constexpr static std::array<std::uint8_t, N> build_list() {
        std::array<std::uint8_t, N> list{};
        for (int i{ 0 }; i < N; ++i) {
            list[i] = static_cast<std::int8_t>(i);
        }
        return list;
    }

    constexpr static std::array<std::uint8_t, N / M> densify(const std::array<std::uint8_t, N>& list) {
        std::array<std::uint8_t, N / M> result{};
        for (std::size_t i = 0; i < result.size(); ++i) {
            std::int8_t hash{ 0 };
            for (int j = 0; j < M; ++j) {
                hash ^= list[i * M + j];
            }
            result[i] = hash;
        }
        return result;
    }
};


int dfs(const std::array<std::array<bool, 128>, 128>& grid, std::vector<bool>& visited, const std::pair<int, int> start) {
    int nodes{ 0 };
    std::vector stack{ start };

    while (!stack.empty()) {
        const auto [y, x] { stack.back() };
        stack.pop_back();

        if (visited[static_cast<std::size_t>(128) * y + x] || !grid[y][x]) {
            continue;
        }

        visited[static_cast<std::size_t>(128) * y + x] = true;
        ++nodes;

        if (x > 0)   { stack.emplace_back(y, x - 1); }
        if (x < 127) { stack.emplace_back(y, x + 1); }

        if (y > 0)   { stack.emplace_back(y - 1, x); }
        if (y < 127) { stack.emplace_back(y + 1, x); }
    }

    return nodes;
}


int main() {
    const std::string_view input{ "vbqugkhl" };

    std::array<std::array<bool, 128>, 128> grid{};

    int used_squares{ 0 };
    for (int i = 0; i < 128; ++i) {
        std::string round_input{ input };
        round_input.push_back('-');
        round_input.append(std::to_string(i));

        const auto hash{ KnotHash<256, 16>::hash(round_input) };
        for (int j = 0; j < 16; ++j) {
            std::uint8_t hash_part{ hash[j] };

            int bit{ 0 };
            while (hash_part != 0) {
                if ((hash_part & 1) == 1) {
                    ++used_squares;

                    grid[i][(j + 1) * 8 - bit - 1] = true;
                }

                ++bit;
                hash_part >>= 1;
            }
        }
    }

    // Part 1
    std::cout << "[Part 1] Used squares: " << used_squares << '\n';

    // Part 2
    int regions{ 0 };
    std::vector visited(static_cast<std::size_t>(128) * 128, false);
    for (int y = 0; y < 128; ++y) {
        for (int x = 0; x < 128; ++x) {
            if (dfs(grid, visited, { y, x }) != 0) {
                ++regions;
            }
        }
    }

    std::cout << "[Part 2] Regions: " << regions << '\n';

    return EXIT_SUCCESS;
}
