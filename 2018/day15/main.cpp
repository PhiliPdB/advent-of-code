#include <algorithm>
#include <array>
#include <deque>
#include <fstream>
#include <iostream>
#include <ostream>
#include <queue>
#include <vector>
#include <string>
#include <unordered_set>


enum class Type {
    Wall,
    Open,
    Goblin,
    Elf,
};

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

struct QueueItem {
    int distance;
    Coordinate current;
    Coordinate start;

    constexpr QueueItem(const int distance, Coordinate current, Coordinate start) noexcept
        : distance{distance}, current{current}, start{start} {
    }

    auto operator<=>(const QueueItem&) const = default;
};


class Place {
private:
    Type m_type;

    int m_attack_power{ 3 };
    int m_hp{ 200 };

public:
    explicit Place(const Type type) noexcept
        : m_type(type) {
    }

    [[nodiscard]]
    constexpr Type type() const {
        return m_type;
    }

    [[nodiscard]]
    constexpr int attack_power() const {
        return m_attack_power;
    }

    constexpr void set_attack_power(const int attack_power) {
        m_attack_power = attack_power;
    }

    [[nodiscard]]
    constexpr int hp() const {
        return m_hp;
    }

    [[nodiscard]]
    constexpr bool hit(const int damage) {
        m_hp -= damage;
        return m_hp <= 0;
    }

    [[nodiscard]]
    constexpr bool is_enemy(const Type type) const {
        switch (m_type) {
            case Type::Wall:
            case Type::Open:
                return false;
            case Type::Goblin:
                return type == Type::Elf;
            case Type::Elf:
                return type == Type::Goblin;
        }

        throw std::runtime_error("Unreachable!");
    }

    friend std::ostream& operator<<(std::ostream& os, const Place& obj) {
        char type_char{ ' ' };
        switch (obj.m_type) {
            case Type::Wall:
                type_char = '#';
                break;
            case Type::Open:
                type_char = '.';
                break;
            case Type::Goblin:
                type_char = 'G';
                break;
            case Type::Elf:
                type_char = 'E';
                break;
        }

        return os << type_char;
    }
};

class Map {
private:
    std::vector<std::vector<Place>> m_map{};
    int m_elfs_left{ 0 };
    int m_goblins_left{ 0 };
    

    explicit Map(std::vector<std::vector<Place>> map, const int elfs_left, const int goblins_left) noexcept
        : m_map{ std::move(map) }, m_elfs_left{ elfs_left }, m_goblins_left{ goblins_left } {
    }

public:
    static Map parse_from_file(std::ifstream& file) {
        std::vector<std::vector<Place>> map;
        int elfs{ 0 };
        int goblins{ 0 };

        int y{ 0 };
        std::string line;
        while (std::getline(file, line)) {
            map.emplace_back(line.size(), Place{ Type::Wall });
            for (int x{ 0 }; x < static_cast<int>(line.size()); ++x) {
                if (line[x] == 'G') {
                    ++goblins;
                    map[y][x] = Place{ Type::Goblin };
                } else if (line[x] == 'E') {
                    ++elfs;
                    map[y][x] = Place{ Type::Elf };
                } else if (line[x] == '.') {
                    map[y][x] = Place{ Type::Open };
                }
            }

            ++y;
        }

        return Map{ std::move(map), elfs, goblins };
    }

    [[nodiscard]]
    bool next_turn() {
        std::unordered_set<Coordinate> moved_to;

        for (size_t y = 0; y < m_map.size(); ++y) {
            for (size_t x = 0; x < m_map[0].size(); ++x) {
                if (m_map[y][x].type() == Type::Wall || m_map[y][x].type() == Type::Open
                    || moved_to.contains(Coordinate{ static_cast<int>(x), static_cast<int>(y) })
                ) {
                    continue;
                }

                const auto current_type = m_map[y][x].type();
                switch (current_type) {
                    case Type::Elf:
                        if (m_goblins_left == 0) {
                            return false;
                        }
                        break;
                    case Type::Goblin:
                        if (m_elfs_left == 0) {
                            return false;
                        }
                        break;
                    default:
                        break;
                }

                // Check if we can attack
                if (try_attack(x, y)) {
                    continue;
                }
                

                // Otherwise try to move

                std::priority_queue<QueueItem, std::vector<QueueItem>, std::greater<>> queue;
                std::unordered_set<Coordinate> visited;
                for (const auto& [nx, ny] : adjacent_squares(static_cast<int>(x), static_cast<int>(y))) {
                    if (m_map[ny][nx].type() == Type::Open) {
                        queue.emplace(1, Coordinate(nx, ny), Coordinate(nx, ny));
                    }
                }

                while (!queue.empty()) {
                    const QueueItem current_item = queue.top();
                    queue.pop();
                    if (!visited.insert(current_item.current).second) {
                        continue;
                    }

                    for (const auto& [nx, ny] : adjacent_squares(current_item.current.x, current_item.current.y)) {
                        if (m_map[ny][nx].type() == Type::Open) {
                            queue.emplace(current_item.distance + 1, Coordinate{ nx, ny }, current_item.start);
                        } else if (m_map[ny][nx].is_enemy(current_type)) {
                            // Move towards this enemy
                            m_map[current_item.start.y][current_item.start.x] = m_map[y][x];
                            m_map[y][x] = Place{ Type::Open };
                            moved_to.insert(Coordinate{ current_item.start.x, current_item.start.y });

                            try_attack(static_cast<size_t>(current_item.start.x), static_cast<size_t>(current_item.start.y));

                            goto end_current_place;
                        }
                    }
                }
                end_current_place: continue;
            }
        }

        return true;
    }

    constexpr void set_elf_attack_power(const int attack_power) {
        for (auto& row : m_map) {
            for (auto& place : row) {
                if (place.type() == Type::Elf) {
                    place.set_attack_power(attack_power);
                }
            }
        }
    }

    [[nodiscard]]
    constexpr int hp_left() const {
        int total_hp{ 0 };
        for (const auto& row : m_map) {
            for (const auto& p : row) {
                if (p.type() == Type::Elf || p.type() == Type::Goblin) {
                    total_hp += p.hp();
                }
            }
        }
        return total_hp;
    }

    [[nodiscard]]
    constexpr int elfs_left() const {
        return m_elfs_left;
    }

    friend std::ostream& operator<<(std::ostream& os, const Map& map) {
        for (size_t y = 0; y < map.m_map.size(); ++y) {
            std::vector<const Place*> entities;
            for (size_t x = 0; x < map.m_map[0].size(); ++x) {
                os << map.m_map[y][x];

                if (map.m_map[y][x].type() == Type::Elf || map.m_map[y][x].type() == Type::Goblin) {
                    entities.push_back(&map.m_map[y][x]);
                }
            }
            if (!entities.empty()) {
                os << "    ";
                for (const Place* entity : entities) {
                    switch (entity->type()) {
                        case Type::Elf:
                            os << 'E';
                            break;
                        case Type::Goblin:
                            os << 'G';
                            break;
                        default:
                            break;
                    }

                    os << '(' << entity->hp() << "), ";
                }
            }

            os << '\n';
        }
        return os;
    }

private:
    bool try_attack(const size_t x, const size_t y) {
        const auto current_type = m_map[y][x].type();
        const auto attack_power = m_map[y][x].attack_power();

        std::vector<std::pair<Coordinate, Place*>> attackable;
        for (const auto& [nx, ny] : adjacent_squares(x, y)) {
            if (m_map[ny][nx].is_enemy(current_type)) {
                attackable.emplace_back(
                    Coordinate{ static_cast<int>(nx), static_cast<int>(ny) },
                    &m_map[ny][nx]
                );
            }
        }
        if (!attackable.empty()) {
            // Pick the one with lowest health (or first) and hit it.
            if (const auto target = std::ranges::min_element(attackable, {}, [](const auto& e) { return e.second->hp(); });
                target->second->hit(attack_power)
            ) {
                switch (target->second->type()) {
                    case Type::Elf:
                        --m_elfs_left;
                        break;
                    case Type::Goblin:
                        --m_goblins_left;
                        break;
                    default:
                        break;
                }

                m_map[target->first.y][target->first.x] = Place{ Type::Open };
            }
            return true;
        }

        return false;
    }


    template<typename T> requires std::integral<T>
    [[nodiscard]]
    static constexpr std::array<std::pair<T, T>, 4> adjacent_squares(const T x, const T y) {
        return { std::make_pair(x, y - 1), std::make_pair(x - 1, y), std::make_pair(x + 1, y), std::make_pair(x, y + 1) };
    }
};

inline int run_game(Map& map) {
    auto turns{ 0 };
    while (map.next_turn()) {
        ++turns;
    }
    return turns * map.hp_left();
}


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    const auto map = Map::parse_from_file(file);
    file.close();

    Map part1_map{ map };
    const auto part1_outcome{ run_game(part1_map) };
    std::cout << "[Part 1] Outcome: " << part1_outcome << '\n';

    
    const int total_elfs{ map.elfs_left() };

    // Find minimal Elf attack power using binary search
    int min_attack_power{ 3 };
    int max_attack_power{ 50 };
    while (min_attack_power < max_attack_power) {
        const int attack_power{ (min_attack_power + max_attack_power) / 2 };

        Map m{ map };
        m.set_elf_attack_power(attack_power);
        run_game(m);
        if (m.elfs_left() == total_elfs) {
            max_attack_power = attack_power;
        } else {
            min_attack_power = attack_power + 1;
        }
    }

    // Run at minimal attack power to get the outcome
    Map part2_map{ map };
    part2_map.set_elf_attack_power(min_attack_power);
    const auto part2_outcome{ run_game(part2_map) };
    std::cout << "[Part 2] Outcome: " << part2_outcome << '\n';

    return EXIT_SUCCESS;
}
