#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>


using AdjacencyList = std::vector<std::vector<int>>;
constexpr auto TOTAL_PROGRAMS{ 2000 };


constexpr int count_connected_nodes(const AdjacencyList& adjacency_list, const int start_node, std::vector<bool>& visited) {
    int nodes{ 0 };

    // Count nodes using depth first search
    std::vector stack{ start_node };
    while (!stack.empty()) {
        const auto current = stack.back();
        stack.pop_back();

        if (visited[current]) {
            continue;
        }
        visited[current] = true;
        // Count this node
        ++nodes;

        for (const auto neighbour : adjacency_list[current]) {
            stack.push_back(neighbour);
        }
    }

    return nodes;
}

constexpr int total_groups(const AdjacencyList& adjacency_list) {
    auto groups{ 0 };
    std::vector visited(TOTAL_PROGRAMS, false);

    for (int i = 0; i < TOTAL_PROGRAMS; ++i) {
        if (!visited[i]) {
            count_connected_nodes(adjacency_list, i, visited);
            ++groups;
        }
    }

    return groups;
}

int main() {
    AdjacencyList adjacency_list(TOTAL_PROGRAMS);

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        std::regex regex{ R"(^(\d+) <-> ((\d+(, )?)+$))" };

        std::smatch match;
        std::regex_match(line, match, regex);

        const int program{ std::stoi(match[1]) };
        std::vector<int> neighbours_vector{};

        std::stringstream neighbours{ match[2] };
        std::string neighbour;
        while (std::getline(neighbours, neighbour, ',')) {
            neighbours_vector.push_back(std::stoi(neighbour));
        }

        adjacency_list[program] = std::move(neighbours_vector);
    }
    file.close();

    // Part 1
    std::vector v(TOTAL_PROGRAMS, false);
    std::cout << "[Part 1] Group with 0 contains " << count_connected_nodes(adjacency_list, 0, v) << " nodes\n";

    // Part 2
    std::cout << "[Part 2] Number of groups: " << total_groups(adjacency_list) << '\n';

    return EXIT_SUCCESS;
}
