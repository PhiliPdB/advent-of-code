#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>


struct Coordinate {
    int y;
    int x;

    constexpr Coordinate(const int x, const int y) noexcept
        : y{ y }, x{ x } {
    }

    auto operator<=>(const Coordinate&) const = default;
};

template<>
struct std::hash<Coordinate> {
    std::size_t operator()(const Coordinate& coordinate) const noexcept {
        const auto h1 = std::hash<int>()(coordinate.x);
        const auto h2 = std::hash<int>()(coordinate.y);
        return h1 ^ (h2 << 1);
    }
};


struct SearchResult {
    int furthest_room;
    int far_away_rooms;
};

class Grid {
private:
    std::unordered_map<Coordinate, std::vector<Coordinate>> m_adjacency_list;

public:
    Grid() noexcept = default;

    explicit Grid(std::unordered_map<Coordinate, std::vector<Coordinate>> adjacency_list) noexcept
        : m_adjacency_list{ std::move(adjacency_list) } {
    }

    void parse_regex(std::istream& regex_stream, Coordinate start = { 0, 0 }) {
        auto current{ start };
        while (true) {
            switch (static_cast<char>(regex_stream.peek())) {
                case 'N': {
                    Coordinate next{ current.x, current.y - 1 };
                    m_adjacency_list[current].push_back(next);
                    m_adjacency_list[next].push_back(current);
                    current = next;
                    break;
                }
                case 'E': {
                    Coordinate next{ current.x + 1, current.y };
                    m_adjacency_list[current].push_back(next);
                    m_adjacency_list[next].push_back(current);
                    current = next;
                    break;
                }
                case 'S': {
                    Coordinate next{ current.x, current.y + 1 };
                    m_adjacency_list[current].push_back(next);
                    m_adjacency_list[next].push_back(current);
                    current = next;
                    break;
                }
                case 'W': {
                    Coordinate next{ current.x - 1, current.y };
                    m_adjacency_list[current].push_back(next);
                    m_adjacency_list[next].push_back(current);
                    current = next;
                    break;
                }
                case '(':
                    regex_stream.get();
                    while (static_cast<char>(regex_stream.peek()) != ')') {
                        parse_regex(regex_stream, current);
                    }
                    break;
                case '^':
                    regex_stream.get();
                    continue;
                case '$':
                case '|':
                    regex_stream.get();
                    [[fallthrough]];
                case ')':
                default:
                    return;
            }
            // Move forward to next character
            regex_stream.get();
        }
    }

    [[nodiscard]]
    SearchResult search_facility() const {
        std::deque<std::pair<Coordinate, int>> queue{};
        std::unordered_set<Coordinate> visited{};

        queue.emplace_back(Coordinate{ 0, 0 }, 0);

        int max_depth{ 0 };
        int far_away_rooms{ 0 };
        while (!queue.empty()) {
            const auto [coordinate, depth] = queue.front();
            queue.pop_front();

            if (!visited.insert(coordinate).second) {
                continue;
            }

            if (depth > max_depth) {
                max_depth = depth;
            }
            if (depth >= 1'000) {
                ++far_away_rooms;
            }

            for (const auto& neighbour : m_adjacency_list.at(coordinate)) {
                queue.emplace_back(neighbour, depth + 1);
            }
        }

        return SearchResult{ max_depth, far_away_rooms };
    }
};

int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    Grid grid{};
    grid.parse_regex(file);
    file.close();

    const auto [furthest_room, far_away_rooms]{ grid.search_facility() };
    std::cout << "Furthest room:  " << furthest_room << '\n';
    std::cout << "Far away rooms: " << far_away_rooms << '\n';

    return EXIT_SUCCESS;
}
