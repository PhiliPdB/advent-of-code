#include <algorithm>
#include <array>
#include <execution>
#include <fstream>
#include <numeric>
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

    Group(const Group& group) = default;


    [[nodiscard]]
    constexpr int units() const {
        return m_units;
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

    void attack(const int power, const std::string& attack) {
        m_units -= damage_taken(power, attack) / m_hit_points;
        m_units = std::max(m_units, 0);
    }

    auto operator<=>(const Group& group) const {
        if (const auto order{ effective_power() <=> group.effective_power() };
            order == std::strong_ordering::equal
        ) {
            return m_initiative <=> group.m_initiative;
        } else {
            return order;
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
    std::ranges::sort(
        choosing_order,
        [](const Group* a, const Group* b) { return *a > *b; },
        [](const auto& e) { return e.second; }
    );

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
                    && defenders[i].initiative() > defenders[*target].initiative()
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

void remove_dead_groups(std::vector<Group>& groups) {
    for (auto it{ groups.begin() }; it != groups.end();) {
        if (it->units() == 0) {
            it = groups.erase(it);
        } else {
            ++it;
        }
    }
}


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
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
    while (!immune_system.empty() && !infection.empty()) {
        // FIGHT!

        // Run target selection
        std::vector immune_targets{ targets(immune_system, infection) };
        std::vector infection_targets{ targets(infection, immune_system) };

        // Let them attack
        std::vector<std::pair<size_t, const Group*>> immune_order;
        immune_order.reserve(immune_system.size());
        for (const auto& group : immune_system) {
            immune_order.emplace_back(immune_order.size(), &group);
        }
        std::ranges::sort(immune_order, std::ranges::greater{}, [](const auto& e) { return e.second->initiative(); });

        std::vector<std::pair<size_t, const Group*>> infection_order;
        infection_order.reserve(infection.size());
        for (const auto& group : infection) {
            infection_order.emplace_back(infection_order.size(), &group);
        }
        std::ranges::sort(infection_order, std::ranges::greater{}, [](const auto& e) { return e.second->initiative(); });

        size_t immune_index{ 0 };
        size_t infection_index{ 0 };
        while (immune_index < immune_order.size() || infection_index < infection_order.size()) {
            const auto immune_initiative{
                immune_index < immune_order.size()
                    ? immune_order[immune_index].second->initiative()
                    : -1
            };
            const auto infection_initiative{
                infection_index < infection_order.size()
                    ? infection_order[infection_index].second->initiative()
                    : -1
            };

            if (immune_initiative > infection_initiative) {
                if (const auto target{ immune_targets[immune_order[immune_index].first] }; target.has_value()) {
                    infection[*target].attack(
                        immune_order[immune_index].second->effective_power(),
                        immune_order[immune_index].second->attack_type()
                    );
                }
                ++immune_index;
            } else {
                if (const auto target{ infection_targets[infection_order[infection_index].first] }; target.has_value()) {
                    immune_system[*target].attack(
                        infection_order[infection_index].second->effective_power(),
                        infection_order[infection_index].second->attack_type()
                    );
                }
                ++infection_index;
            }
        }

        // Remove dead groups
        remove_dead_groups(immune_system);
        remove_dead_groups(infection);
    }

    int winning_army_units;
    if (!immune_system.empty()) {
        winning_army_units = std::transform_reduce(
            std::execution::par_unseq,
            immune_system.begin(), immune_system.end(), 
            0,
            std::plus(),
            [](const auto& e) { return e.units(); }
        );
    } else {
        winning_army_units = std::transform_reduce(
            std::execution::par_unseq,
            infection.begin(), infection.end(),
            0,
            std::plus(),
            [](const auto& e) { return e.units(); }
        );
    }
    std::cout << "Winning army units: " << winning_army_units << '\n';

    return EXIT_SUCCESS;
}

// Too High: 22680
// Too Low: 20622
