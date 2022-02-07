#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>

constexpr int SAFE_DISTANCE = 10'000;

class Coordinate {
public:
    int x;
    int y;

    Coordinate(const int x, const int y)
        : x(x), y(y) {
    }

    static Coordinate parse(const std::string_view s) {
        const auto sep_loc = s.find(", ");

        const auto x = std::stoi(s.substr(0, sep_loc).data());
        const auto y = std::stoi(s.substr(sep_loc + 2, s.size() - sep_loc - 2).data());

        return { x, y };
    }

    [[nodiscard]]
    int distance(const Coordinate& other) const {
        return abs(x - other.x) + abs(y - other.y);
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::vector<Coordinate> input;
    while (std::getline(file, line)) {
        input.push_back(Coordinate::parse(line));
    }
    file.close();


    const auto [min_x, max_x] { std::ranges::minmax(input, {}, [](auto e) { return e.x; }) };
    const auto [min_y, max_y] { std::ranges::minmax(input, {}, [](auto e) { return e.y; }) };

    std::vector<int> areas;
    areas.resize(input.size(), 0);
    std::vector<int> excluded_areas;

    int safe_area_size = 0;

    for (int y = min_y.y; y <= max_y.y; ++y) {
        for (int x = min_x.x; x <= max_x.x; ++x) {
            // Check to which area this points belongs
            int closest_point = -1;
            int closest_distance = INT32_MAX;
            int amount = 1;

            int total_distance = 0;
            for (int i = 0; i < static_cast<int>(input.size()); ++i) {
                const auto distance = input[i].distance({x, y});
                if (distance < closest_distance) {
                    closest_point = i;
                    closest_distance = distance;
                    amount = 1;
                } else if (distance == closest_distance) {
                    ++amount;
                }

                total_distance += distance;
            }

            if (total_distance <= SAFE_DISTANCE) {
                safe_area_size += 1;
            }

            if (amount == 1) {
                ++areas[closest_point];
            }

            if (y == min_y.y || y == max_y.y || x == min_x.x || x == max_x.x) {
                // Point is on the border
                excluded_areas.push_back(closest_point);

                if (total_distance <= SAFE_DISTANCE) {
                    std::cerr << "Safe on the border...\n";
                    return EXIT_FAILURE;
                }
            }
        }
    }

    // Reset excluded areas
    for (const auto excluded_area : excluded_areas) {
        areas[excluded_area] = -1;
    }

    // Find max area
    const auto largest_area = std::ranges::max(areas);

    std::cout << "Largest area: " << largest_area << '\n';
    std::cout << "Safe area: " << safe_area_size << '\n';


    return EXIT_SUCCESS;
}
