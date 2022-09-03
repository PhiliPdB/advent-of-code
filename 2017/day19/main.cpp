#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <ostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

enum class Direction {
    North, East, South, West,
};

int main() {
    std::vector<std::string> map;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    std::string line;
    while (std::getline(file, line)) {
        map.push_back(line);
    }
    file.close();


    [[maybe_unused]]
    const int height{ static_cast<int>(map.size()) };
    const int width{ static_cast<int>(map[0].size()) };

    
    std::pair current_position{ 0, 0 };
    auto current_direction{ Direction::South };

    // Find start position
    for (int i = 0; i < width; ++i) {
        if (map[0][i] == '|') {
            current_position.first = i;
            break;
        }
    }

    // Part 1
    std::string letter_order;
    // Part 2
    int steps{ 0 };

    while (true) {
        // Walk
        switch (current_direction) {
            case Direction::North:
                current_position.second -= 1;
                break;
            case Direction::East:
                current_position.first += 1;
                break;
            case Direction::South:
                current_position.second += 1;
                break;
            case Direction::West:
                current_position.first -= 1;
                break;
            default:
                throw std::runtime_error("Unknown direction");
        }
        ++steps;

        const char next_tile{ map[current_position.second][current_position.first] };
        if (next_tile != '+' && next_tile != '|' && next_tile != '-' && next_tile != ' ') {
            letter_order.push_back(next_tile);
        }
        if (next_tile == '+') {
            // Find the new direction

            // Look north and south
            if (current_direction == Direction::East || current_direction == Direction::West) {
                if (map[current_position.second - 1][current_position.first] == '|') {
                    current_direction = Direction::North;
                } else {
                    current_direction = Direction::South;
                }
            } else {
                if (map[current_position.second][current_position.first - 1] == '-') {
                    current_direction = Direction::West;
                } else {
                    current_direction = Direction::East;
                }
            }
        } else if (next_tile == ' ') {
            break;
        }
    }
    
    std::cout << "[Part 1] Found the path: " << letter_order << '\n';
    std::cout << "[Part 2] It took " << steps << " steps\n";

    return EXIT_SUCCESS;
}
