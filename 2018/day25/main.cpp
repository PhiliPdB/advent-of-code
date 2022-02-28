#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>


struct Point {
    int w;
    int x;
    int y;
    int z;

    constexpr Point(const int w, const int x, const int y, const int z) noexcept
        : w{w}, x{x}, y{y}, z{z} {
    }

    static Point parse(const std::string& line) {
        static const std::regex regex{ R"((-?\d+),(-?\d+),(-?\d+),(-?\d+))" };

        std::smatch match;
        std::regex_match(line, match, regex);

        return Point{ std::stoi(match[1]), std::stoi(match[2]), std::stoi(match[3]), std::stoi(match[4]) };
    }

    [[nodiscard]]
    int distance(const Point& p) const {
        return std::abs(w - p.w) + std::abs(x - p.x) + std::abs(y - p.y) + std::abs(z - p.z);
    }

    [[nodiscard]]
    int distance(const std::vector<Point>& ps) const {
        const auto min{ std::ranges::min(ps, {}, [&](const auto& p) { return distance(p); }) };

        return distance(min);
    }
};

int main() {
    std::vector<Point> points;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        points.push_back(Point::parse(line));
    }
    file.close();

    std::ranges::sort(points, {}, [](const auto& e) { return e.distance(Point{ 0, 0, 0, 0 }); });

    std::vector<std::vector<Point>> constellations;
    for (const auto& point : points) {
        // Try to add to existing constellation
        for (auto& constellation : constellations) {
            for (const auto& cp : constellation) {
                if (cp.distance(point) <= 3) {
                    constellation.push_back(point);
                    goto next;
                }
            }
        }

        // Add new constellation
        constellations.emplace_back(1, point);
    next:;
    }

    // Merge constellations
    bool changes_made{ false };
    do {
        changes_made = false;

        std::vector<std::vector<Point>> new_constellations;
        new_constellations.reserve(constellations.size());
        new_constellations.push_back(constellations[0]);

        for (auto c = constellations.begin() + 1; c != constellations.end(); ++c) {
            for (auto& new_constellation : new_constellations) {
                int distance = INT32_MAX;
                for (auto& p_i : *c) {
                    for (auto& p_j : new_constellation) {
                        if (const int d{ p_i.distance(p_j) }; d < distance) {
                            distance = d;
                        }
                    }
                }

                if (distance <= 3) {
                    for (const auto& point : *c) {
                        new_constellation.push_back(point);
                    }
                    changes_made = true;
                    goto next_constellation;
                }
            }

            new_constellations.push_back(*c);

        next_constellation:;
        }

        std::swap(constellations, new_constellations);
    } while (changes_made);


    std::cout << "Constellations: " << constellations.size() << '\n';

    return EXIT_SUCCESS;
}
