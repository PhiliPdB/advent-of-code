#include <array>
#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

enum class Direction {
    NorthWest,
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
};

// Hex grid:
// - https://www.redblobgames.com/grids/hexagons
// - https://www.redblobgames.com/grids/hexagons/implementation.html
// With q pointing to the north
class Hex {
private:
    int m_q;
    int m_r;
    int m_s;

public:
    constexpr Hex(const int q, const int r, const int s)
        : m_q{ q }, m_r{ r }, m_s{ s } {
        assert(q + r + s == 0);
    }

    [[nodiscard]]
    int length() const {
        return (abs(m_q) + abs(m_r) + abs(m_s)) / 2;
    }

    [[nodiscard]]
    int distance(const Hex& b) const {
        return (*this - b).length();
    }

    constexpr void step(Direction d);


    constexpr Hex operator+(const Hex& b) const {
        return { m_q + b.m_q, m_r + b.m_r, m_s + b.m_s };
    }

    constexpr Hex& operator+=(const Hex& b) {
        return *this = *this + b;
    }

    constexpr Hex operator-(const Hex& b) const {
        return { m_q - b.m_q, m_r - b.m_r, m_s - b.m_s };
    }
};

constexpr std::array DIRECTIONS{
    Hex(-1, 0,  1), Hex(0, -1,  1), Hex( 1, -1, 0),
    Hex( 1, 0, -1), Hex(0,  1, -1), Hex(-1,  1, 0)
};

constexpr void Hex::step(Direction d) {
    *this += DIRECTIONS[static_cast<int>(d)];
}


int main() {
    std::vector<Direction> child_path{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string step;
    while (std::getline(file, step, ',')) {
        if (std::size_t pos; (pos = step.find('\n')) != std::string::npos) {
            step.replace(pos, 1, "");
        }

        Direction d;
        if (step == "n") {
            d = Direction::North;
        } else if (step == "ne") {
            d = Direction::NorthEast;
        } else if (step == "nw") {
            d = Direction::NorthWest;
        } else if (step == "s") {
            d = Direction::South;
        } else if (step == "se") {
            d = Direction::SouthEast;
        } else if (step == "sw") {
            d = Direction::SouthWest;
        } else {
            throw std::runtime_error("Unreachable");
        }

        child_path.push_back(d);
    }
    file.close();

    // Part 2:
    int furthest_distance{ 0 };

    // Part 1: Find the end coordinate
    Hex child_location{ 0, 0, 0 };
    for (const auto& direction : child_path) {
        child_location.step(direction);

        if (child_location.length() > furthest_distance) {
            furthest_distance = child_location.length();
        }
    }
    std::cout << "[Part 1] Child is " << child_location.length() << " steps away\n";
    std::cout << "[Part 2] Furthest encountered distance: " << furthest_distance << '\n';

    return EXIT_SUCCESS;
}
