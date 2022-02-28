#include <array>
#include <iostream>

constexpr auto TARGET{ 347'991 };

using Grid = std::array<std::array<int, 51>, 51>;
constexpr int set_sum(Grid& grid, const size_t x, const size_t y) {
    const auto sum{
        grid[y - 1][x - 1] + grid[y - 1][x] + grid[y - 1][x + 1]
            + grid[y][x - 1] + grid[y][x + 1]
            + grid[y + 1][x - 1] + grid[y + 1][x] + grid[y + 1][x + 1]
    };

    grid[y][x] = sum;
    return sum;
}


int main() {
    // Part 1
    int distance{ 0 };
    int current_square{ 1 };
    int increment{ 1 };
    while (current_square + increment < TARGET) {
        current_square += increment;
        distance += 1;

        increment += 8;
    }
    // Walk along spiral to the target
    const int radius{ distance };
    for (int i = 0; i < 8; ++i) {
        const int factor{ i % 2 == 0 ? 1 : -1 };

        const auto d{ std::min(radius, TARGET - current_square) };
        current_square += d;
        distance += factor * d;

        if (current_square == TARGET) {
            break;
        }
    }
    
    std::cout << "Distance: " << distance << '\n';


    // Part 2
    Grid grid{};
    size_t cur_x{ 25 };
    size_t cur_y{ 25 };
    grid[cur_y][cur_x] = 1;

    size_t step_size{ 1 };
    while (true) {
        // Move right
        for (size_t i = 1; i <= step_size; ++i) {
            ++cur_x;
            if (set_sum(grid, cur_x, cur_y) >= TARGET) {
                goto found_value;
            }
        }
        // Move up
        for (size_t i = 1; i <= step_size; ++i) {
            --cur_y;
            if (set_sum(grid, cur_x, cur_y) >= TARGET) {
                goto found_value;
            }
        }

        step_size += 1;

        // Move left
        for (size_t i = 1; i <= step_size; ++i) {
            --cur_x;
            if (set_sum(grid, cur_x, cur_y) >= TARGET) {
                goto found_value;
            }
        }

        // Move down
        for (size_t i = 1; i <= step_size; ++i) {
            ++cur_y;
            if (set_sum(grid, cur_x, cur_y) >= TARGET) {
                goto found_value;
            }
        }

        step_size += 1;
    }

    found_value:
    std::cout << "First value written: " << grid[cur_y][cur_x] << '\n';

    return EXIT_SUCCESS;
}
