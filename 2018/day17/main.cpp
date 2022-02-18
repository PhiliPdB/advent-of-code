#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>


enum class State {
    Sand,
    Clay,
    FlowingWater,
    RestingWater,
};


struct Coordinate {
    int y;
    int x;

    constexpr Coordinate(const int x, const int y) noexcept
        : y{ y }, x{ x } {
    }

    auto operator<=>(const Coordinate&) const = default;
};

void print_map(std::ostream& out, const std::vector<std::vector<State>>& map) {
    for (const auto& row : map) {
        for (const auto& state : row) {
            char state_char{ ' ' };
            switch (state) {
                case State::Sand:
                    state_char = ' ';
                    break;
                case State::Clay:
                    state_char = '#';
                    break;
                case State::FlowingWater:
                    state_char = '|';
                    break;
                case State::RestingWater:
                    state_char = '~';
                    break;
            }
            out << state_char;
        }
        out << '\n';
    }
}

int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::vector<Coordinate> clay_coordinates;

    const std::regex match_x_first{ R"(^x=(\d+),\ y=(\d+)\.\.(\d+)$)" };
    const std::regex match_y_first{ R"(^y=(\d+),\ x=(\d+)\.\.(\d+)$)" };

    while (std::getline(file, line)) {
        // Parse and add to coordinates.
        if (std::smatch match; std::regex_match(line, match, match_x_first)) {
            const int x{ std::stoi(match[1]) };
            const int y_min{ std::stoi(match[2]) };
            const int y_max{ std::stoi(match[3]) };

            for (int y = y_min; y <= y_max; ++y) {
                clay_coordinates.emplace_back(x, y);
            }
        } else if (std::regex_match(line, match, match_y_first)) {
            const int y{ std::stoi(match[1]) };
            const int x_min{ std::stoi(match[2]) };
            const int x_max{ std::stoi(match[3]) };

            for (int x = x_min; x <= x_max; ++x) {
                clay_coordinates.emplace_back(x, y);
            }
        }
    }

    file.close();


    // Create map for the current ground state

    constexpr Coordinate water_spring{ 500, 0 };

    const auto [x_min, x_max] { std::ranges::minmax(clay_coordinates, {}, [](auto& e) { return e.x; }) };
    const auto [y_min, y_max] { std::ranges::minmax(clay_coordinates, {}, [](auto& e) { return e.y; }) };

    const auto width{ x_max.x - x_min.x + 3 };
    const auto height{ y_max.y - y_min.y + 1 };

    std::vector map(height, std::vector(width, State::Sand));
    for (const auto& clay_coordinate : clay_coordinates) {
        map[static_cast<size_t>(clay_coordinate.y) - y_min.y][static_cast<size_t>(clay_coordinate.x) - x_min.x + 1] = State::Clay;
    }
    assert(y_min.y >= water_spring.y);
    map[0][static_cast<size_t>(water_spring.x) - x_min.x + 1] = State::FlowingWater;


    // Let the water flow

    std::vector<Coordinate> stack;
    stack.emplace_back(water_spring.x - x_min.x + 1, 0);

    while (!stack.empty()) {
        Coordinate current{ stack.back() };
        stack.pop_back();

        // Flow down till bottom or clay
        while (current.y + 1 < height && map[static_cast<size_t>(current.y) + 1][current.x] == State::Sand) {
            map[static_cast<size_t>(current.y) + 1][current.x] = State::FlowingWater;
            current.y += 1;
        }

        // Check if we can fill a reservoir
        if (current.y + 1 < height
            && (map[static_cast<size_t>(current.y) + 1][current.x] == State::Clay || map[static_cast<size_t>(current.y) + 1][current.x] == State::RestingWater)
        ) {
            // Let the water flow out till it hits clay or is not supported by clay or resting water anymore.
            while (true) { // While the water is contained, spread it sidewards
                // Make the current spot is flowing water
                map[current.y][current.x] = State::FlowingWater;

                // Go left
                Coordinate left{ current.x - 1, current.y };
                while ((map[static_cast<size_t>(left.y) + 1][left.x] == State::Clay || map[static_cast<size_t>(left.y) + 1][left.x] == State::RestingWater)
                    && map[left.y][left.x] != State::Clay
                ) {
                    map[left.y][left.x] = State::FlowingWater;
                    left.x -= 1;
                }

                // Go right
                Coordinate right{ current.x + 1, current.y };
                while ((map[static_cast<size_t>(right.y) + 1][right.x] == State::Clay || map[static_cast<size_t>(right.y) + 1][right.x] == State::RestingWater)
                    && map[right.y][right.x] != State::Clay
                ) {
                    map[right.y][right.x] = State::FlowingWater;
                    right.x += 1;
                }

                if (map[left.y][left.x] == State::Clay && map[right.y][right.x] == State::Clay) {
                    // Hit both left and right

                    // Fill with resting water
                    for (int x = left.x + 1; x < right.x; ++x) {
                        map[left.y][x] = State::RestingWater;
                    }

                    // Move up by one
                    current.y -= 1;
                } else {
                    if (map[left.y][left.x] != State::Clay) {
                        stack.emplace_back(left.x, left.y - 1);
                    }
                    if (map[right.y][right.x] != State::Clay) {
                        stack.emplace_back(right.x, right.y - 1);
                    }
                    break;
                }
            }
        }
    }

    int total_water{ 0 };
    int resting_water{ 0 };
    for (const auto& row : map) {
        for (const auto& state : row) {
            switch (state) {
                case State::RestingWater:
                    ++resting_water;
                    [[fallthrough]];
                case State::FlowingWater:
                    ++total_water;
                    break;
                default:
                    break;
            }
        }
    }

    std::cout << "[Part 1] Total water: " << total_water << '\n';
    std::cout << "[Part 2] Resting water: " << resting_water << '\n';

    // Print the map to an output file
    std::ofstream out{ "out.txt" };
    print_map(out, map);
    out.close();

    return EXIT_SUCCESS;
}
