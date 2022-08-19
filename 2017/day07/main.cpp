#include <algorithm>
#include <fstream>
#include <iostream>
#include <optional>
#include <regex>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>

class Tower {
private:
    std::string m_name;
    int m_weight;

    std::optional<std::string> m_parent{};
    std::unordered_set<std::string> m_children{};

    int m_total_child_weight{ 0 };
    int m_correct_child_weight{ 0 };
    bool m_balanced_children{ true };
    std::optional<std::string_view> m_incorrect_child{};

public:
    Tower() noexcept : m_name{}, m_weight{ -1 } {
    }

    Tower(std::string name) noexcept
        : m_name{ std::move(name) }, m_weight{ -1 } {
    }

    Tower(std::string name, const int weight) noexcept
        : m_name{ std::move(name) }, m_weight{ weight } {
    }

    void update_from(Tower tower) {
        m_weight = tower.m_weight;
        m_children = std::move(tower.m_children);
    }

    void set_parent(const std::string& parent) {
        m_parent = parent;
    }

    void add_child(std::string child) {
        m_children.insert(std::move(child));
    }

    [[nodiscard]]
    constexpr std::string_view name() const {
        return m_name;
    }

    [[nodiscard]]
    constexpr int weight() const {
        return m_weight;
    }

    [[nodiscard]]
    constexpr bool has_parent() const {
        return m_parent.has_value();
    }

    [[nodiscard]]
    constexpr std::string parent() const {
        return m_parent.value();
    }

    [[nodiscard]]
    constexpr bool balanced_children() const {
        return m_balanced_children;
    }

    [[nodiscard]]
    constexpr std::string_view incorrect_child() const {
        return m_incorrect_child.value();
    }

    [[nodiscard]]
    int weight_correction() const {
        return static_cast<int>(m_children.size()) * m_correct_child_weight - m_total_child_weight;
    }

    int calculate_weights(std::unordered_map<std::string, Tower>& towers) {
        if (m_children.empty()) {
            return m_weight;
        } else if (m_total_child_weight != 0) {
            return m_weight + m_total_child_weight;
        } else {
            std::unordered_map<int, int> weights{};
            for (std::string_view child_name : m_children) {
                const auto weight = towers[static_cast<std::string>(child_name)].calculate_weights(towers);
                weights[weight] += 1;
                m_total_child_weight += weight;
            }

            m_balanced_children = weights.size() == 1;

            if (!m_balanced_children) {
                int correct_weight{ 0 };
                int max_child_count{ 0 };
                for (const auto& [w, c] : weights) {
                    if (c > max_child_count) {
                        max_child_count = c;
                        correct_weight = w;
                    }
                }
                m_correct_child_weight = correct_weight;

                for (std::string_view child_name : m_children) {
                    if (towers[static_cast<std::string>(child_name)].calculate_weights(towers) != m_correct_child_weight) {
                        m_incorrect_child = child_name;
                    }
                }
            } else {
                m_correct_child_weight = m_total_child_weight / static_cast<int>(m_children.size());
            }

            return m_weight + m_total_child_weight;
        }
    }
};


int main() {
    std::unordered_map<std::string, Tower> towers{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        const std::regex regex{ R"(^([a-z]+) \((\d+)\)(( -> )([a-z]+(, )?)+)?$)" };

        std::smatch match;
        std::regex_match(line, match, regex);

        Tower tower{ match[1].str(), std::stoi(match[2]) };
        std::string name{ tower.name() };

        // Add children if it has children
        if (match[3].matched) {
            auto children = match[3].str().substr(3);
            children.append(",");

            std::stringstream children_stream{ children };
            std::string child;
            while (std::getline(children_stream, child, ',')) {
                child = child.substr(1);
                tower.add_child(child);

                // Set parent of child
                if (towers.contains(child)) {
                    towers[child].set_parent(name);
                } else {
                    Tower child_tower{ child };
                    child_tower.set_parent(name);
                    towers.insert({ child, std::move(child_tower) });
                }
            }
        }

        
        if (towers.contains(name)) {
            // Update existing tower
            towers[name].update_from(std::move(tower));
        } else {
            towers.insert({ std::move(name), std::move(tower) });
        }
    }
    file.close();

    // Part 1: Find top tower
    auto current{ towers.begin()->second };
    while (current.has_parent()) {
        current = towers[current.parent()];
    }
    std::cout << "Bottom program: " << current.name() << '\n';

    // Part 2: Calculate weights
    int weight_correction{ 0 };
    current.calculate_weights(towers);
    while (!current.balanced_children()) {
        weight_correction = current.weight_correction();
        current = towers[static_cast<std::string>(current.incorrect_child())];
    }
    std::cout << current.name() << " should weigh " << current.weight() + weight_correction << '\n';

    return EXIT_SUCCESS;
}
