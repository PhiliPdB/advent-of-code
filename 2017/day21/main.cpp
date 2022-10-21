#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

template<const int N>
int pattern_to_int(std::array<std::string, N> pattern) {
    int result{ 0 };

    for (int row = 0; row < N; ++row) {
        for (int col = 0; col < N; ++col) {
            if (pattern[row][col] == '#') {
                result |= (1 << (col + row * N));
            }
        }
    }

    return result;
}

template<const int N>
int pattern_to_int(std::array<std::array<bool, N>, N> pattern) {
    int result{ 0 };

    for (int row = 0; row < N; ++row) {
        for (int col = 0; col < N; ++col) {
            if (pattern[row][col]) {
                result |= (1 << (col + row * N));
            }
        }
    }

    return result;
}

template<const int N>
std::array<std::array<bool, N>, N> int_to_pattern(const int input) {
    std::array<std::array<bool, N>, N> pattern;

    for (int row = 0; row < N; ++row) {
        for (int col = 0; col < N; ++col) {
            pattern[row][col] = (input >> (col + row * N)) & 1;
        }
    }

    return pattern;
}

template<const int N>
std::vector<std::pair<int, int>> generate_transformations(std::array<std::string, N>& from, std::array<std::string, N + 1>& to) {
    std::vector<std::pair<int, int>> tranformations;

    const auto output{ pattern_to_int(to) };

    // Rotate
    for (int i = 0; i < 4; ++i) {
        const auto tmp{ from[0][0] };
        from[0][0] = from[N - 1][0];
        from[N - 1][0] = from[N - 1][N - 1];
        from[N - 1][N - 1] = from[0][N - 1];
        from[0][N - 1] = tmp;

        if constexpr (N == 3) {
            const auto tmp2{ from[0][1] };
            from[0][1] = from[1][0];
            from[1][0] = from[2][1];
            from[2][1] = from[1][2];
            from[1][2] = tmp2;
        }

        tranformations.emplace_back(pattern_to_int(from), output);

        // Horizontal flip
        for (std::string& row : from) {
            std::ranges::reverse(row);
        }
        tranformations.emplace_back(pattern_to_int(from), output);
        // Revert
        for (std::string& row : from) {
            std::ranges::reverse(row);
        }

        // Vertical flip
        for (int r = 0; r < N / 2; ++r) {
            std::swap(from[r], from[N - 1 - r]);
        }
        tranformations.emplace_back(pattern_to_int(from), output);
        // Revert
        for (int r = 0; r < N / 2; ++r) {
            std::swap(from[r], from[N - 1 - r]);
        }
    }

    return tranformations;
}

constexpr int on_lamps(std::vector<std::vector<bool>>& grid) {
    auto on_lamps{ 0 };
    for (int y = 0; y < grid.size(); ++y) {
        for (int x = 0; x < grid.size(); ++x) {
            if (grid[y][x]) {
                ++on_lamps;
            }
        }
    }
    return on_lamps;
}

int main() {
    std::unordered_map<int, int> twoByTwoTransformations{};
    std::unordered_map<int, int> threeByThreeTransformations{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    std::string line;
    for (int i = 0; i < 6; ++i) {
        std::getline(file, line);

        // Break into from - to
        auto from{ std::array{ line.substr(0, 2), line.substr(3, 2) } };
        auto to{ std::array{ line.substr(9, 3), line.substr(13, 3), line.substr(17, 3) } };
        
        // Generate all transformations
        for (const auto& transformation : generate_transformations(from, to)) {
            twoByTwoTransformations[transformation.first] = transformation.second;
        }
    }
    for (int i = 0; i < 102; ++i) {
        std::getline(file, line);

        // Break into from - to
        auto from{ std::array{ line.substr(0, 3), line.substr(4, 3), line.substr(8, 3) } };
        auto to{ std::array{ line.substr(15, 4), line.substr(20, 4), line.substr(25, 4), line.substr(30, 4) } };

        // Generate all transformations
        for (const auto& transformation : generate_transformations(from, to)) {
            threeByThreeTransformations[transformation.first] = transformation.second;
        }
    }
    file.close();

    // Starting grid
    std::vector grid{
        std::vector{ false, true,  false },
        std::vector{ false, false, true  },
        std::vector{ true,  true,  true  },
    };

    for (int iteration = 0; iteration < 18; ++iteration) {
        if (grid.size() % 2 == 0) {
            const auto new_size{ grid.size() + grid.size() / 2 };
            std::vector<std::vector<bool>> new_grid;
            new_grid.resize(new_size, std::vector(new_size, false));

            for (int y = 0; y < grid.size(); y += 2) {
                for (int x = 0; x < grid.size(); x += 2) {
                    // Get pattern
                    const std::array<std::array<bool, 2>, 2> pattern{
                        std::array<bool, 2>{ grid[y][x],     grid[y][x + 1] },
                        std::array<bool, 2>{ grid[y + 1][x], grid[y + 1][x + 1] },
                    };

                    // Get output
                    const auto output{ int_to_pattern<3>(twoByTwoTransformations[pattern_to_int(pattern)]) };

                    // Save output
                    const auto new_y{ y / 2 * 3 };
                    const auto new_x{ x / 2 * 3 };
                    for (int r = 0; r < output.size(); ++r) {
                        for (int c = 0; c < output.size(); ++c) {
                            new_grid[new_y + r][new_x + c] = output[r][c];
                        }
                    }
                }
            }
            std::swap(grid, new_grid);
        } else {
            const auto new_size{ grid.size() + grid.size() / 3 };
            std::vector<std::vector<bool>> new_grid;
            new_grid.resize(new_size, std::vector(new_size, false));

            for (int y = 0; y < grid.size(); y += 3) {
                for (int x = 0; x < grid.size(); x += 3) {
                    // Get pattern
                    const std::array<std::array<bool, 3>, 3> pattern{
                        std::array<bool, 3>{ grid[y][x],     grid[y][x + 1],     grid[y][x + 2] },
                        std::array<bool, 3>{ grid[y + 1][x], grid[y + 1][x + 1], grid[y + 1][x + 2] },
                        std::array<bool, 3>{ grid[y + 2][x], grid[y + 2][x + 1], grid[y + 2][x + 2] },
                    };

                    // Get output
                    const auto output{ int_to_pattern<4>(threeByThreeTransformations[pattern_to_int(pattern)]) };

                    // Save output
                    const auto new_y{ y / 3 * 4 };
                    const auto new_x{ x / 3 * 4 };
                    for (int r = 0; r < output.size(); ++r) {
                        for (int c = 0; c < output.size(); ++c) {
                            new_grid[new_y + r][new_x + c] = output[r][c];
                        }
                    }
                }
            }
            std::swap(grid, new_grid);
        }

        if (iteration == 4) {
            std::cout << "[Part 1] Total 'on' lamps: " << on_lamps(grid) << '\n';
        }
    }
    
    std::cout << "[Part 2] Total 'on' lamps: " << on_lamps(grid) << '\n';
    
    return EXIT_SUCCESS;
}
