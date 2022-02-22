#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <queue>
#include <regex>
#include <string>
#include <vector>


template<typename T> requires std::integral<T>
struct Coordinate {
    T x;
    T y;
    T z;

    constexpr Coordinate(const T x, const T y, const T z) noexcept
        : x{ x }, y{ y }, z{ z } {
    }

    [[nodiscard]]
    constexpr T distance(const Coordinate& coordinate) const {
        return std::abs(x - coordinate.x) + std::abs(y - coordinate.y) + std::abs(z - coordinate.z);
    }

    auto operator<=>(const Coordinate&) const = default;
};


template<typename T> requires std::integral<T>
class Nanobot {
private:
    Coordinate<T> m_coordinate;
    T m_radius;

public:
    constexpr Nanobot(Coordinate<T> coordinate, const T radius) noexcept
        : m_coordinate{ coordinate }, m_radius{ radius } {
    }

    [[nodiscard]]
    constexpr Coordinate<T> coordinate() const {
        return m_coordinate;
    }

    [[nodiscard]]
    constexpr T radius() const {
        return m_radius;
    }

    [[nodiscard]]
    constexpr int distance(const Nanobot<T>& nanobot) const {
        return m_coordinate.distance(nanobot.m_coordinate);
    }
};


template<typename T> requires std::integral<T>
struct Cube {
    Coordinate<T> top_left_corner;
    Coordinate<T> size;

    constexpr Cube(const Coordinate<T> top_left_corner, const Coordinate<T> size) noexcept
        : top_left_corner{ top_left_corner }, size{ size } {
    }

    [[nodiscard]]
    int intersections(const std::vector<Nanobot<T>>& nanobots) const {
        int intersections{ 0 };
        for (const auto& nanobot : nanobots) {
            if (intersects(nanobot)) {
                ++intersections;
            }
        }
        return intersections;
    }

    [[nodiscard]]
    constexpr bool intersects(const Nanobot<T>& nanobot) const {
        // Calculate manhattan distance from nanobot to the cube
        T distance{ 0 };

        // Walk along the axis
        distance += std::abs(nanobot.coordinate().x - top_left_corner.x) + std::abs(nanobot.coordinate().x - (top_left_corner.x + size.x)) - size.x;
        distance += std::abs(nanobot.coordinate().y - top_left_corner.y) + std::abs(nanobot.coordinate().y - (top_left_corner.y + size.y)) - size.y;
        distance += std::abs(nanobot.coordinate().z - top_left_corner.z) + std::abs(nanobot.coordinate().z - (top_left_corner.z + size.z)) - size.z;

        distance /= 2;
        return distance <= nanobot.radius();
    }

    [[nodiscard]]
    constexpr int distance_to_center() const {
        T distance{ 0 };

        // Walk along the axis
        distance += std::abs(top_left_corner.x) + std::abs(top_left_corner.x + size.x) - size.x;
        distance += std::abs(top_left_corner.y) + std::abs(top_left_corner.y + size.y) - size.y;
        distance += std::abs(top_left_corner.z) + std::abs(top_left_corner.z + size.z) - size.z;

        distance /= 2;
        return distance;
    }
};

template<typename T> requires std::integral<T>
struct HeapItem {
    int intersections;
    Cube<T> cube;


    auto operator<=>(const HeapItem& item) const {
        const auto intersection_ordering{ item.intersections <=> intersections };
        if (intersection_ordering != std::strong_ordering::equivalent) {
            return intersection_ordering;
        } else {
            return cube.distance_to_center() <=> item.cube.distance_to_center();
        }
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
    std::vector<Nanobot<int>> nanobots;

    const std::regex nanobot_regex{ R"(pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+))" };

    while (std::getline(file, line)) {
        if (std::smatch match; std::regex_match(line, match, nanobot_regex)) {
            const int x{ std::stoi(match[1]) };
            const int y{ std::stoi(match[2]) };
            const int z{ std::stoi(match[3]) };

            const int radius{ std::stoi(match[4]) };

            nanobots.emplace_back(Coordinate{ x, y, z }, radius);
        }
    }
    file.close();

    // Part 1
    const auto largest_signal_radius_nanobot{ std::ranges::max(nanobots, {}, [](auto& e) { return e.radius(); }) };
    int in_range{ 0 };
    for (const auto& nanobot : nanobots) {
        if (largest_signal_radius_nanobot.distance(nanobot) <= largest_signal_radius_nanobot.radius()) {
            ++in_range;
        }
    }

    std::cout << "Nanobots in range: " << in_range << '\n';

    // Part 2
    const auto [min_x, max_x] { std::ranges::minmax(nanobots, {}, [](auto& e) { return e.coordinate().x; }) };
    const auto [min_y, max_y] { std::ranges::minmax(nanobots, {}, [](auto& e) { return e.coordinate().y; }) };
    const auto [min_z, max_z] { std::ranges::minmax(nanobots, {}, [](auto& e) { return e.coordinate().z; }) };

    const Cube initial_cube{
        Coordinate{ min_x.coordinate().x, min_y.coordinate().y, min_z.coordinate().z },
        Coordinate{ max_x.coordinate().x - min_x.coordinate().x, max_y.coordinate().y - min_y.coordinate().y, max_z.coordinate().z - min_z.coordinate().z }
    };


    std::priority_queue<HeapItem<int>, std::vector<HeapItem<int>>, std::greater<>> queue;
    queue.emplace(initial_cube.intersections(nanobots), initial_cube);

    while (!queue.empty()) {
        const auto [intersections, cube]{ queue.top() };
        queue.pop();

        if (cube.size == Coordinate{ 0, 0, 0 }) {
            std::cout << "Shortest manhattan distance: " << cube.distance_to_center() << '\n';
            return EXIT_SUCCESS;
        }

        Coordinate new_size{ cube.size.x / 2, cube.size.y / 2, cube.size.z / 2 };

        std::array new_cubes{
            Cube{ cube.top_left_corner, new_size },

            Cube{ Coordinate{ cube.top_left_corner.x + new_size.x, cube.top_left_corner.y, cube.top_left_corner.z }, new_size },
            Cube{ Coordinate{ cube.top_left_corner.x, cube.top_left_corner.y + new_size.y, cube.top_left_corner.z }, new_size },
            Cube{ Coordinate{ cube.top_left_corner.x, cube.top_left_corner.y, cube.top_left_corner.z + new_size.z }, new_size },

            Cube{ Coordinate{ cube.top_left_corner.x + new_size.x, cube.top_left_corner.y + new_size.y, cube.top_left_corner.z }, new_size },
            Cube{ Coordinate{ cube.top_left_corner.x + new_size.x, cube.top_left_corner.y, cube.top_left_corner.z + new_size.z }, new_size },
            Cube{ Coordinate{ cube.top_left_corner.x, cube.top_left_corner.y + new_size.y, cube.top_left_corner.z + new_size.z }, new_size },

            Cube{ Coordinate{ cube.top_left_corner.x + new_size.x, cube.top_left_corner.y + new_size.y, cube.top_left_corner.z + new_size.z }, new_size },
        };

        for (const auto& new_cube : new_cubes) {
            queue.emplace(new_cube.intersections(nanobots), new_cube);
        }
    }

    return EXIT_FAILURE;
}
