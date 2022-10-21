#include <algorithm>
#include <fstream>
#include <iostream>
#include <queue>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

template<typename T1, typename T2>
struct std::hash<std::pair<T1, T2>> {
    std::size_t operator()(const std::pair<T1, T2>& pair) const noexcept {
        const auto h1{ std::hash<T1>()(pair.first) };
        const auto h2{ std::hash<T2>()(pair.second) };
        return h1 ^ (h2 << 1);
    }
};

struct Node {
    int strength;
    int next_port;
    int component;
    int depth;

    std::vector<bool> used_components;
};


int main() {
    std::unordered_map<int, std::vector<std::pair<int, int>>> components;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    int component_number{ 0 };

    const std::regex regex{ R"(^(\d+)/(\d+)$)" };
    std::string line;
    while (std::getline(file, line)) {
        std::smatch match;
        std::regex_match(line, match, regex);

        const auto port1{ std::stoi(match[1]) };
        const auto port2{ std::stoi(match[2]) };

        components[port1].emplace_back(port2, component_number);
        components[port2].emplace_back(port1, component_number);

        ++component_number;
    }
    file.close();


    const auto total_components{ component_number };

    std::vector<std::pair<int, int>> bridges;

    // Use BFS to explore all paths
    std::deque<Node> queue;
    std::vector visited(total_components, false);

    queue.emplace_back(0, 0, -1, 0, std::vector(total_components, false));

    while (!queue.empty()) {
        const auto [strength, next_port, component, depth, uc] { queue.front() };
        queue.pop_front();

        bool increased_bridge{ false };
        for (const auto [np, nc] : components[next_port]) {
            if (!uc[nc]) {
                auto nuc{ uc };
                nuc[nc] = true;
                queue.emplace_back(strength + next_port + np, np, nc, depth + 1, nuc);

                increased_bridge = true;
            }
        }

        if (!increased_bridge) {
            bridges.emplace_back(strength, depth);
        }
    }

    std::ranges::sort(bridges, std::greater<>{}, [](const auto e) { return e.first; });
    const auto longest{ std::ranges::max_element(bridges, {}, [](const auto e) { return e.second; }) };


    std::cout << "[Part 1] Strongest bridge: " << bridges[0].first << '\n';
    std::cout << "[Part 2] Strength of longest bridge: " << longest->first << '\n';


    return EXIT_SUCCESS;
}
