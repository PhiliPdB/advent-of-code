#include <algorithm>
#include <deque>
#include <execution>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>
#include <string>


class Node {
private:
    std::vector<Node> m_children;
    std::vector<int> m_metadata;

public:
    Node(std::vector<Node> children, std::vector<int> metadata)
        : m_children{ std::move(children) }, m_metadata{ std::move(metadata) } {
    }

    static Node parse(std::deque<int>& input) {
        const auto amount_children = input.front();
        input.pop_front();
        const auto amount_metadata = input.front();
        input.pop_front();

        std::vector<Node> children;
        std::vector<int> metadata;

        while (static_cast<int>(children.size()) < amount_children) {
            children.push_back(parse(input));
        }

        while (static_cast<int>(metadata.size()) < amount_metadata) {
            metadata.push_back(input.front());
            input.pop_front();
        }

        return { std::move(children), std::move(metadata) };
    }

    [[nodiscard]]
    int sum() const {
        const auto child_sum{
            std::transform_reduce(
                std::execution::par_unseq,
                m_children.begin(), m_children.end(), 0,
                std::plus(),
                [](const Node& e) { return e.sum(); }
            )
        };

        return child_sum + metadata_sum();
    }

    [[nodiscard]]
    int value() const {
        if (m_children.empty()) {
            return metadata_sum();
        } else {
            return std::transform_reduce(
                std::execution::par_unseq,
                m_metadata.begin(), m_metadata.end(), 0,
                std::plus(),
                [&](const int i) {
                    return i <= static_cast<int>(m_children.size()) ? m_children[i - 1].value() : 0;
                }
            );
        }
    }

private:
    [[nodiscard]]
    int metadata_sum() const {
        return std::reduce(std::execution::par_unseq, m_metadata.begin(), m_metadata.end());
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::deque<int> input;
    while (file >> line) {
        input.push_back(std::stoi(line));
    }
    file.close();


    const auto node{ Node::parse(input) };
    std::cout << "Metadata sum: " << node.sum() << '\n';
    std::cout << "Root value: " << node.value() << '\n';


    return EXIT_SUCCESS;
}
