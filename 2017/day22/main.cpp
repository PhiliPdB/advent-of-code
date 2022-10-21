#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

template<typename T1, typename T2>
struct std::hash<std::pair<T1, T2>> {
    std::size_t operator()(const std::pair<T1, T2>& pair) const noexcept {
        const auto h1{ std::hash<T1>()(pair.first) };
        const auto h2{ std::hash<T2>()(pair.second) };
        return h1 ^ (h2 << 1);
    }
};

enum class Direction {
    Up, Right, Down, Left,
};

class VirusCarrier {
private:
    int m_x;
    int m_y;
    Direction m_direction;

public:
    constexpr VirusCarrier(const int x, const int y, const Direction d) :
        m_x{ x }, m_y{ y }, m_direction{ d } {
    }

    [[nodiscard]]
    constexpr std::pair<int, int> location() const {
        return { m_y, m_x };
    }

    constexpr void move() {
        switch (m_direction) {
            case Direction::Up:
                --m_y;
                break;
            case Direction::Right:
                ++m_x;
                break;
            case Direction::Down:
                ++m_y;
                break;
            case Direction::Left:
                --m_x;
                break;
        }
    }

    constexpr void turn_right() {
        m_direction = static_cast<Direction>((static_cast<int>(m_direction) + 1) % 4);
    }

    constexpr void turn_left() {
        m_direction = static_cast<Direction>((static_cast<int>(m_direction) + 3) % 4);
    }

    constexpr void reverse_direction() {
        m_direction = static_cast<Direction>((static_cast<int>(m_direction) + 2) % 4);
    }
};

int main() {
    std::unordered_set<std::pair<int, int>> infected_nodes{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    int width{ 0 };
    int y{ 0 };
    std::string line;
    while (std::getline(file, line)) {
        width = static_cast<int>(line.size());
        for (int x = 0; x < line.size(); ++x) {
            if (line[x] == '#') {
                infected_nodes.emplace(y, x);
            }
        }

        ++y;
    }

    file.close();

    // Part 1
    auto part1_infected_nodes{ infected_nodes };
    VirusCarrier carrier{ y / 2, width / 2, Direction::Up };
    int part1_newly_infected{ 0 };

    for (int burst = 0; burst < 10'000; ++burst) {
        if (part1_infected_nodes.contains(carrier.location())) {
            // Turn
            carrier.turn_right();

            // Then remove the infected node
            part1_infected_nodes.erase(carrier.location());
        } else {
            // Turn
            carrier.turn_left();

            // Infect the node
            part1_infected_nodes.insert(carrier.location());
            ++part1_newly_infected;
        }

        carrier.move();
    }

    std::cout << "[Part 1] Newly infected nodes: " << part1_newly_infected << '\n';

    // Part 2
    carrier = VirusCarrier{ y / 2, width / 2, Direction::Up };
    int part2_newly_infected{ 0 };

    std::unordered_set<std::pair<int, int>> flagged_nodes{};
    std::unordered_set<std::pair<int, int>> weakened_nodes{};

    for (int burst = 0; burst < 10'000'000; ++burst) {
        const auto location{ carrier.location() };

        if (infected_nodes.contains(location)) { // Infected
            carrier.turn_right();

            // Then remove the infected node
            infected_nodes.erase(location);
            flagged_nodes.insert(location);
        } else if (flagged_nodes.contains(location)) {
            carrier.reverse_direction();

            // Make the node clean
            flagged_nodes.erase(location);
        } else if (weakened_nodes.contains(location)) {
            weakened_nodes.erase(location);
            infected_nodes.insert(location);
            ++part2_newly_infected;
        } else {
            carrier.turn_left();

            // Infect the node
            weakened_nodes.insert(location);
        }

        carrier.move();
    }

    std::cout << "[Part 2] Newly infected nodes: " << part2_newly_infected << '\n';
    
    return EXIT_SUCCESS;
}
