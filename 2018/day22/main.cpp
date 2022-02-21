#include <algorithm>
#include <array>
#include <iostream>
#include <queue>
#include <vector>

struct Coordinate {
    int y;
    int x;

    constexpr Coordinate(const int x, const int y) noexcept
        : y{ y }, x{ x } {
    }

    auto operator<=>(const Coordinate&) const = default;
};


constexpr auto DEPTH{ 3'339 };
constexpr Coordinate TARGET{ 10, 715 };

using Map = std::array<std::array<int, TARGET.x + 100>, TARGET.y + 50>;
constexpr Map build_erosion_level() {
    Map map{};
    // Set for y = 0
    for (int x = 1; static_cast<size_t>(x) < map[0].size(); ++x) {
        map[0][x] = x * 16'807;
        map[0][x] += DEPTH;
        map[0][x] %= 20'183;
    }
    // Set for x = 0
    for (int y = 1; static_cast<size_t>(y) < map.size(); ++y) {
        map[y][0] = y * 48'271;
        map[y][0] += DEPTH;
        map[y][0] %= 20'183;
    }

    // Set rest
    for (int x = 1; static_cast<size_t>(x) < map[0].size(); ++x) {
        for (int y = 1; static_cast<size_t>(y) < map.size(); ++y) {
            if (x == TARGET.x && y == TARGET.y) {
                map[y][x] = 0;
            } else {
                map[y][x] = map[static_cast<size_t>(y) - 1][x] * map[y][static_cast<size_t>(x) - 1];
            }

            map[y][x] += DEPTH;
            map[y][x] %= 20'183;
        }
    }

    return map;
}

#ifdef _DEBUG
static Map EROSION_LEVEL{ build_erosion_level() };
#else
static constexpr Map EROSION_LEVEL{ build_erosion_level() };
#endif


constexpr int risk_level(const int width, const int height) {
    int risk_level{ 0 };
    for (int y = 0; y <= height; ++y) {
        for (int x = 0; x <= width; ++x) {
            risk_level += EROSION_LEVEL[y][x] % 3;
        }
    }
    return risk_level;
};


enum class Tool {
    Neither = 0,
    Torch,
    ClimbingGear,
};

struct HeapItem {
    int minutes;
    Tool tool;
    Coordinate position;

    auto operator<=>(const HeapItem&) const = default;
};

int main() {
    // Part 1
    std::cout << "Risk level: " << risk_level(TARGET.x, TARGET.y) << '\n';

    // Part 2
    std::priority_queue<HeapItem, std::vector<HeapItem>, std::greater<>> queue;
    std::vector distance(EROSION_LEVEL.size(), std::vector(EROSION_LEVEL[0].size(), std::array{INT32_MAX, INT32_MAX, INT32_MAX}));

    queue.emplace(0, Tool::Torch, Coordinate{ 0, 0 });
    while (!queue.empty()) {
        const auto [min, tool, pos] = queue.top();
        queue.pop();

        const auto tool_index{ static_cast<int>(tool) };
        if (!(0 <= tool_index && tool_index < 3)) {
            throw std::runtime_error("unreachable");
        }

        if (distance[pos.y][pos.x][tool_index] <= min) {
            continue;
        } else {
            distance[pos.y][pos.x][tool_index] = min;
        }

        if (pos == TARGET) {
            if (tool == Tool::Torch) {
                break;
            } else {
                queue.emplace(min + 7, Tool::Torch, pos);
                continue;
            }
        }

        const int current_region{ EROSION_LEVEL[pos.y][pos.x] % 3 };
        const std::array neighbours{
            Coordinate{ pos.x - 1, pos.y }, Coordinate{ pos.x, pos.y - 1 },
            Coordinate{ pos.x + 1, pos.y }, Coordinate{ pos.x, pos.y + 1 }
        };
        for (const auto& neighbour : neighbours) {
            if (neighbour.x < 0 || static_cast<size_t>(neighbour.x) >= EROSION_LEVEL[0].size()
                || neighbour.y < 0 || static_cast<size_t>(neighbour.y) >= EROSION_LEVEL.size()
            ) {
                continue;
            }

            if (const int next_region{ EROSION_LEVEL[neighbour.y][neighbour.x] % 3 };
                current_region == next_region
            ) {
                queue.emplace(min + 1, tool, neighbour);
            } else {
                switch (tool) {
                    case Tool::Neither:
                        if (current_region == 1 && next_region == 0) {
                            queue.emplace(min + 8, Tool::ClimbingGear, neighbour);
                        } else if (current_region == 2 && next_region == 0) {
                            queue.emplace(min + 8, Tool::Torch, neighbour);
                        } else {
                            queue.emplace(min + 1, tool, neighbour);
                        }
                        break;
                    case Tool::Torch:
                        if (current_region == 0 && next_region == 1) {
                            queue.emplace(min + 8, Tool::ClimbingGear, neighbour);
                        } else if (current_region == 2 && next_region == 1) {
                            queue.emplace(min + 8, Tool::Neither, neighbour);
                        } else {
                            queue.emplace(min + 1, tool, neighbour);
                        }
                        break;
                    case Tool::ClimbingGear:
                        if (current_region == 0 && next_region == 2) {
                            queue.emplace(min + 8, Tool::Torch, neighbour);
                        } else if (current_region == 1 && next_region == 2) {
                            queue.emplace(min + 8, Tool::Neither, neighbour);
                        } else {
                            queue.emplace(min + 1, tool, neighbour);
                        }
                        break;
                }
            }
        }
    }

    std::cout << "It took " << distance[TARGET.y][TARGET.x][static_cast<int>(Tool::Torch)] << " minutes to reach the target.\n";

    return EXIT_SUCCESS;
}
