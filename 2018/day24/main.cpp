#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <optional>
#include <regex>
#include <string>
#include <unordered_set>
#include <vector>


class Group {
private:
    int m_units;
    int m_initiative;

    int m_hit_points;

    int m_attack_damage;
    std::string m_attack_type;

    std::unordered_set<std::string> m_weaknesses;
    std::unordered_set<std::string> m_immunities;

public:
    Group(const int units, const int initiative, const int hit_points, const int attack_damage, std::string attack_type,
        std::unordered_set<std::string> weaknesses, std::unordered_set<std::string> immunities)
        : m_units{units}, m_initiative{initiative},
          m_hit_points{hit_points}, m_attack_damage{attack_damage},
          m_attack_type{std::move(attack_type)},
          m_weaknesses{std::move(weaknesses)}, m_immunities{std::move(immunities)} {
    }

    [[nodiscard]]
    constexpr int effective_power() const {
        return m_units * m_attack_damage;
    }

    [[nodiscard]]
    constexpr int initiative() const {
        return m_initiative;
    }

    [[nodiscard]]
    constexpr const std::string& attack_type() const {
        return m_attack_type;
    }

    [[nodiscard]]
    int damage_taken(const int power, const std::string& attack) const {
        if (m_weaknesses.contains(attack)) {
            return 2 * power;
        } else if (m_immunities.contains(attack)) {
            return 0;
        } else {
            return power;
        }
    }
};


std::unordered_set<std::string> get_words(std::string line) {
    std::unordered_set<std::string> set{};

    size_t pos{ line.find(", ") };
    while (pos != std::string::npos) {
        set.insert(line.substr(0, pos));

        line.erase(0, pos + 2);
        pos = line.find(", ");
    }
    set.insert(line);

    return set;
}

std::vector<std::optional<size_t>> targets(const std::vector<Group>& attackers, const std::vector<Group>& defenders) {
    std::vector targets(attackers.size(), std::optional<size_t>{});

    std::vector<std::pair<size_t, const Group*>> choosing_order{};
    choosing_order.reserve(attackers.size());
    for (const auto& group : attackers) {
        choosing_order.emplace_back(choosing_order.size(), &group);
    }
    std::ranges::sort(choosing_order, {}, [](auto& e) { return e.second->effective_power(); });

    std::unordered_set<size_t> selected_targets{};
    for (const auto& [index, group] : choosing_order) {
        std::optional<size_t> target;
        for (size_t i = 0; i < defenders.size(); ++i) {
            if (selected_targets.contains(i)) {
                continue;
            }

            if (!target.has_value()
                || defenders[i].damage_taken(group->effective_power(), group->attack_type()) > defenders[*target].damage_taken(group->effective_power(), group->attack_type())
                || (
                    defenders[i].damage_taken(group->effective_power(), group->attack_type()) == defenders[*target].damage_taken(group->effective_power(), group->attack_type())
                    && defenders[i].effective_power() > defenders[*target].effective_power()
                    )
                || (
                    defenders[i].damage_taken(group->effective_power(), group->attack_type()) == defenders[*target].damage_taken(group->effective_power(), group->attack_type())
                    && defenders[i].effective_power() == defenders[*target].effective_power()
                    && defenders[i].initiative() < defenders[*target].initiative()
                    )
                ) {
                target = i;
            }
        }

        if (target.has_value() && defenders[*target].damage_taken(group->effective_power(), group->attack_type()) > 0) {
            selected_targets.insert(*target);
            targets[index] = target;
        }
    }

    return targets;
}


int main() {
    // Read input
    std::ifstream file{ "test_input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    const std::regex base_regex{ R"((\d+) units each with (\d+) hit points\s*(\(.+\))? with an attack that does (\d+) (\S+) damage at initiative (\d+))" };

    std::vector<Group> immune_system;
    std::vector<Group> infection;

    bool filling_immune_system{ false };
    while (std::getline(file, line)) {
        if (std::smatch match; std::regex_match(line, match, base_regex)) {
            const int units{ std::stoi(match[1]) };
            const int initiative{ std::stoi(match[6]) };

            const int hit_points{ std::stoi(match[2])};

            const int attack_damage{ std::stoi(match[4]) };
            std::string attack_type{ match[5] };

            std::unordered_set<std::string> weaknesses{};
            std::unordered_set<std::string> immunities{};

            // Parse weaknesses and immunities
            if (std::string weakness_immunity_string{ match[3] }; !weakness_immunity_string.empty()) {
                weakness_immunity_string = weakness_immunity_string.substr(1, weakness_immunity_string.size() - 2);
                const auto del_location{ weakness_immunity_string.find("; ") };
                const std::string first_part{ weakness_immunity_string.substr(0, del_location) };
                const std::string second_part{
                    del_location == std::string::npos
                        ? ""
                        : weakness_immunity_string.substr(del_location + 2, weakness_immunity_string.size() - del_location - 2)
                };

                if (first_part.starts_with("weak to ")) {
                    weaknesses = get_words(first_part.substr(8, std::string::npos));
                } else if (second_part.starts_with("weak to ")) {
                    weaknesses = get_words(second_part.substr(8, std::string::npos));
                }

                if (first_part.starts_with("immune to ")) {
                    immunities = get_words(first_part.substr(10, std::string::npos));
                }
                else if (second_part.starts_with("immune to ")) {
                    immunities = get_words(second_part.substr(10, std::string::npos));
                }
            }

            Group group{ units, initiative, hit_points, attack_damage, std::move(attack_type), std::move(weaknesses), std::move(immunities)};
            if (filling_immune_system) {
                immune_system.push_back(std::move(group));
            } else {
                infection.push_back(std::move(group));
            }
        } else if (line == "Immune System:") {
            filling_immune_system = true;
        } else if (line == "Infection:") {
            filling_immune_system = false;
        }
    }
    file.close();

    // Part 1
    while (true) {
        // FIGHT!

        // Run target selection
        std::vector immune_targets{ targets(immune_system, infection) };
        std::vector infection_targets{ targets(infection, immune_system) };

        // Let them attack
        
        break;
    }



    return EXIT_SUCCESS;
}
