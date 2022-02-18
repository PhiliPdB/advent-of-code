#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

enum class State {
    Open,
    Trees,
    Lumberyard,
};

template<const unsigned int Size>
class Lumber {
private:
    std::array<std::array<State, Size>, Size> m_map;

public:
    explicit Lumber(const std::array<std::array<State, Size>, Size> map)
        : m_map{ map } {
    }

    static Lumber parse(std::ifstream& file) {
        std::array<std::array<State, Size>, Size> map{};

        int y{ 0 };
        std::string line;
        while (std::getline(file, line)) {
            for (int x = 0; x < Size; ++x) {
                auto s{ State::Open };
                switch (line[x]) {
                    case '.':
                        s = State::Open;
                        break;
                    case '|':
                        s = State::Trees;
                        break;
                    case '#':
                        s = State::Lumberyard;
                        break;
                    default:
                        throw std::runtime_error("Invalid character");
                }
                map[y][x] = s;
            }
            ++y;
        }

        return Lumber{ map };
    }

    void next_iteration() {
        std::array<std::array<State, Size>, Size> next_map{};
        for (uint32_t y = 0; y < Size; ++y) {
            for (uint32_t x = 0; x < Size; ++x) {
                // Go through the neighbours
                int open_neighbours{ 0 };
                int tree_neighbours{ 0 };
                int lumberyard_neighbours{ 0 };
                // ReSharper disable once CppTooWideScopeInitStatement
                const std::array neighbours{
                    std::make_pair(x - 1, y - 1), std::make_pair(x, y - 1), std::make_pair(x + 1, y - 1),
                    std::make_pair(x - 1, y), std::make_pair(x + 1, y),
                    std::make_pair(x - 1, y + 1), std::make_pair(x, y + 1), std::make_pair(x + 1, y + 1),
                };
                for (const auto& [nx, ny] : neighbours) {
                    if (nx < Size && ny < Size) {
                        // ReSharper disable once CppDefaultCaseNotHandledInSwitchStatement
                        switch (m_map[ny][nx]) {
                            case State::Open:
                                ++open_neighbours;
                                break;
                            case State::Trees:
                                ++tree_neighbours;
                                break;
                            case State::Lumberyard:
                                ++lumberyard_neighbours;
                                break;
                        }
                    }
                }

                // ReSharper disable once CppDefaultCaseNotHandledInSwitchStatement
                switch (m_map[y][x]) {
                    case State::Open:
                        if (tree_neighbours >= 3) {
                            next_map[y][x] = State::Trees;
                        } else {
                            next_map[y][x] = State::Open;
                        }
                        break;
                    case State::Trees:
                        if (lumberyard_neighbours >= 3) {
                            next_map[y][x] = State::Lumberyard;
                        }
                        else {
                            next_map[y][x] = State::Trees;
                        }
                        break;
                    case State::Lumberyard:
                        if (lumberyard_neighbours >= 1 && tree_neighbours >= 1) {
                            next_map[y][x] = State::Lumberyard;
                        } else {
                            next_map[y][x] = State::Open;
                        }
                        break;
                }
            }
        }

        std::swap(m_map, next_map);
    }

    [[nodiscard]]
    constexpr int resource_value() const {
        int tree_count{ 0 };
        int lumberyard_count{ 0 };
        for (const auto& row : m_map) {
            for (const auto& acre : row) {
                switch (acre) {
                    case State::Trees:
                        ++tree_count;
                        break;
                    case State::Lumberyard:
                        ++lumberyard_count;
                        break;
                    default:
                        break;
                }
            }
        }

        return tree_count * lumberyard_count;
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    auto lumber{ Lumber<50>::parse(file) };
    file.close();


    for (int min = 0; min < 10; ++min) {
        lumber.next_iteration();
    }

    std::cout << "[Part 1] Resource value: " << lumber.resource_value() << '\n';

    std::vector<int> resource_values{};
    std::unordered_set<int> seen_values{};

    while (true) {
        lumber.next_iteration();

        const int rv{ lumber.resource_value() };
        resource_values.push_back(rv);
        if (seen_values.insert(rv).second) {
            continue;
        }

        // Check if we are repeating
        const auto last_index{ std::find(resource_values.rbegin() + 1, resource_values.rend(), rv) - resource_values.rbegin() };
        if (static_cast<size_t>(last_index) > (resource_values.size() / 2)) {
            continue;
        }

        bool found_sequence{ true };
        for (int i = 0; i < last_index; ++i) {
            if (resource_values[resource_values.size() - i - 1] != resource_values[resource_values.size() - i - 1 - last_index]) {
                found_sequence = false;
            }
        }

        if (found_sequence) {
            const auto iterations_left{ 1'000'000'000 - resource_values.size() - 10 };
            const auto remainder{ iterations_left % last_index };
            const auto answer{ resource_values[resource_values.size() - last_index - 1 + remainder] };

            std::cout << "[Part 2] Resource value: " << answer << '\n';
            break;
        }
    }

    return EXIT_SUCCESS;
}
