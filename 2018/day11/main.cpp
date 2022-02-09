#include <algorithm>
#include <iostream>
#include <list>
#include <numeric>
#include <vector>

constexpr int GRID_SERIAL_NUMBER = 9995;

constexpr int power_level(const int x, const int y) {
    const int rack_id = x + 10;
    int power_level{ rack_id * y };
    power_level += GRID_SERIAL_NUMBER;
    power_level *= rack_id;

    // Only keep the hundreds digit
    power_level /= 100;
    power_level %= 10;

    power_level -= 5;

    return power_level;
}

constexpr std::vector<std::vector<int>> build_power_level_lookup() {
    std::vector lookup(301, std::vector(301, 0));
    for (int x = 1; x <= 300; ++x) {
        for (int y = 1; y <= 300; ++y) {
            lookup[x][y] = power_level(x, y);
        }
    }
    return lookup;
}

static const auto POWER_LEVEL_LOOKUP{ build_power_level_lookup() };


int main() {
    auto max_power = 0;
    auto max_power_3 = 0;
    std::tuple<int, int, int> max_power_coordinate;
    std::pair<int, int> max_power_coordinate_3;

    for (int x = 1; x <= 300; ++x) {
        for (int y = 1; y <= 300; ++y) {
            const auto max_size{ 301 - std::max(x, y) };

            auto power{ 0 };
            for (int size = 1; size < max_size; ++size) {
                for (int px = 0; px < size; ++px) {
                    power += POWER_LEVEL_LOOKUP[x + px][y + size - 1];
                }
                for (int py = 0; py < size; ++py) {
                    power += POWER_LEVEL_LOOKUP[x + size - 1][y + py];
                }
                power -= POWER_LEVEL_LOOKUP[x + size - 1][y + size - 1];

                // Check the new power sum
                if (size == 3) {
                    if (power > max_power_3) {
                        max_power_3 = power;
                        max_power_coordinate_3 = { x, y };
                    }
                }

                if (power > max_power) {
                    max_power = power;
                    max_power_coordinate = std::make_tuple(x, y, size);
                }
            }
        }
    }

    std::cout << "[Part 1] Coordinate with maximum power: "
        << max_power_coordinate_3.first << ", " << max_power_coordinate_3.second
        << '\n';
    std::cout << "[Part 2] Coordinate with maximum power: "
        << std::get<0>(max_power_coordinate) << ", " << std::get<1>(max_power_coordinate) << ", " << std::get<2>(max_power_coordinate)
        << '\n';

    return EXIT_SUCCESS;
}
