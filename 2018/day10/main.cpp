#include <algorithm>
#include <deque>
#include <execution>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>
#include <string>


class Star {
public:
    int x;
    int y;

    int vx;
    int vy;


    Star(const int x, const int y, const int vx, const int vy)
        : x(x), y(y), vx(vx), vy(vy) {
    }

    static Star parse(const std::string_view line) {
        const auto x{ std::stoi(line.substr(10, 6).data()) };
        const auto y{ std::stoi(line.substr(18, 6).data()) };

        const auto vx{ std::stoi(line.substr(36, 2).data()) };
        const auto vy{ std::stoi(line.substr(40, 2).data()) };

        return { x, y, vx, vy };
    }

    void update_position() {
        x += vx;
        y += vy;
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
    std::vector<Star> input;
    while (std::getline(file, line)) {
        input.push_back(Star::parse(line));
    }
    file.close();

    int seconds{ 0 };
    auto [min_y, max_y] { std::ranges::minmax(input, {}, [](auto e) { return e.y; }) };
    // Update the position of the stars until they are close enough
    while (std::abs(max_y.y - min_y.y) > 11) {
        for (auto& value : input) {
            value.update_position();
        }

        min_y.update_position();
        max_y.update_position();

        seconds += 1;
    }
    const auto [min_x, max_x] { std::ranges::minmax(input, {}, [](auto e) { return e.x; }) };

    // Create a canvas for the stars
    std::vector<std::vector<char>> canvas;
    canvas.resize(max_y.y - min_y.y + 1, std::vector(max_x.x - min_x.x + 1, ' '));

    // Fill the canvas
    for (const auto& star : input) {
        canvas[star.y - min_y.y][star.x - min_x.x] = '#';
    }

    // Print everything
    std::cout << "Sky after " << seconds << " seconds:\n";
    for (const auto& row : canvas) {
        std::cout << std::string(row.begin(), row.end()) << '\n';
    }

    return EXIT_SUCCESS;
}
