#include <algorithm>
#include <fstream>
#include <iostream>
#include <optional>
#include <vector>
#include <string>


enum class PathType {
    Empty,
    Straight,
    Turn,
    TurnAlt,
    Intersection,
};

enum class Direction {
    Up,
    Right,
    Down,
    Left,
};

struct Coordinate {
    int y;
    int x;

    constexpr Coordinate(const int x, const int y): y{ y }, x{ x } {
    }

    auto operator<=>(const Coordinate&) const = default;
};

struct MineCart {
    Coordinate location;
    Direction direction;

private:
    int m_intersections = 0;

public:
    constexpr MineCart(const Coordinate location, const Direction direction)
        : location{ location }, direction{ direction } {
    }

    [[nodiscard]]
    constexpr Coordinate next_location() const {
        switch (direction) {
            case Direction::Up:
                return Coordinate{ location.x, location.y - 1 };
            case Direction::Right:
                return Coordinate{ location.x + 1, location.y };
            case Direction::Down: 
                return Coordinate{ location.x, location.y + 1 };
            case Direction::Left:
                return Coordinate{ location.x - 1, location.y };
            default:
                throw std::runtime_error("Unreachable");
        }
    }

    constexpr void turn_left() {
        direction = static_cast<Direction>((static_cast<int>(direction) + 3) % 4);
    }

    constexpr void turn_right() {
        direction = static_cast<Direction>((static_cast<int>(direction) + 1) % 4);
    }

    constexpr void turn_intersection() {
        switch (m_intersections % 3) {
            case 0:
                turn_left();
                break;
            case 2:
                turn_right();
                break;
            case 1:
            default:
                break;
        }
        ++m_intersections;
    }
};

class Map {
private:
    std::vector<MineCart> m_carts;
    std::vector<std::vector<PathType>> m_map;


    constexpr Map(std::vector<MineCart> carts, std::vector<std::vector<PathType>> map)
        : m_carts{ std::move(carts) }, m_map{ std::move(map) } {
    }

public:
    static Map parse_from_file(std::ifstream& file) {
        std::vector<MineCart> carts;
        std::vector<std::vector<PathType>> map;

        std::string line;
        auto y{ 0 };
        while (std::getline(file, line)) {
            map.emplace_back(line.size(), PathType::Empty);
            for (int x = 0; x < static_cast<int>(line.size()); ++x) {
                switch (line[x]) {
                    case '|':
                    case '-':
                        map[y][x] = PathType::Straight;
                        break;
                    case '/':
                        map[y][x] = PathType::Turn;
                        break;
                    case '\\':
                        map[y][x] = PathType::TurnAlt;
                        break;
                    case '+':
                        map[y][x] = PathType::Intersection;
                        break;
                    case '^':
                        map[y][x] = PathType::Straight;
                        carts.push_back(MineCart{ Coordinate{ x, y }, Direction::Up });
                        break;
                    case '>':
                        map[y][x] = PathType::Straight;
                        carts.push_back(MineCart{ Coordinate{ x, y }, Direction::Right });
                        break;
                    case 'v':
                        map[y][x] = PathType::Straight;
                        carts.push_back(MineCart{ Coordinate{ x, y }, Direction::Down });
                        break;
                    case '<':
                        map[y][x] = PathType::Straight;
                        carts.push_back(MineCart{ Coordinate{ x, y }, Direction::Left });
                        break;
                    case ' ':
                        break;
                    default:
                        throw std::runtime_error("Unexpected character when parsing the map");
                }
            }

            ++y;
        }

        return Map{ std::move(carts), std::move(map) };
    }

    [[nodiscard]]
    constexpr Coordinate step() {
        std::optional<Coordinate> first_crash;

        while (m_carts.size() > 1) {
            std::ranges::sort(m_carts, {}, [](auto c) { return c.location; });

            std::vector<int> to_remove;
            for (int i = 0; i < static_cast<int>(m_carts.size()); ++i) {
                auto& cart = m_carts[i];

                // Update cart position
                const auto new_location = cart.next_location();

                // Detect crash
                if (const auto crash{ std::ranges::find(m_carts, new_location, [](auto c) { return c.location; }) };
                    crash != m_carts.end()
                ) {
                    if (!first_crash.has_value()) {
                        first_crash = new_location;
                    }

                    to_remove.push_back(static_cast<int>(crash - m_carts.begin()));
                    to_remove.push_back(i);
                }

                cart.location = new_location;

                // Detect if the cart should turn
                switch (m_map[new_location.y][new_location.x]) {
                    case PathType::Turn:
                        switch (cart.direction) {
                            case Direction::Up:
                            case Direction::Down:
                                cart.turn_right();
                                break;
                            case Direction::Right:
                            case Direction::Left:
                                cart.turn_left();
                                break;
                        }
                        break;
                    case PathType::TurnAlt:
                        switch (cart.direction) {
                            case Direction::Up:
                            case Direction::Down:
                                cart.turn_left();
                                break;
                            case Direction::Right:
                            case Direction::Left:
                                cart.turn_right();
                                break;
                        }
                        break;
                    case PathType::Intersection:
                        cart.turn_intersection();
                        break;
                    default:
                        break;
                }
            }


            std::ranges::sort(to_remove);
            while (!to_remove.empty()) {
                const auto last = to_remove.back();
                to_remove.pop_back();

                m_carts.erase(m_carts.begin() + last);
            }
        }

        return *first_crash;
    }

    [[nodiscard]]
    constexpr const MineCart& first_cart() const {
        return m_carts[0];
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    auto map = Map::parse_from_file(file);
    file.close();


    const auto first_crash = map.step();

    std::cout << "First crash occurs at: " << first_crash.x << ',' << first_crash.y << '\n';
    const auto& last_cart = map.first_cart();
    std::cout << "Last cart ends up at: " << last_cart.location.x << ',' << last_cart.location.y << '\n';

    return EXIT_SUCCESS;
}
