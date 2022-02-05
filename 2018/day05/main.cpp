#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>

class Polymer {
private:
    std::vector<char> m_polymer;

public:
    Polymer() = default;

    [[nodiscard]]
    size_t size() const {
        return m_polymer.size();
    }

    void add_reduce(const char c) {
        m_polymer.push_back(c);

        while (m_polymer.size() >= 2
            && std::tolower(m_polymer[m_polymer.size() - 1]) == std::tolower(m_polymer[m_polymer.size() - 2])
            && m_polymer[m_polymer.size() - 1] != m_polymer[m_polymer.size() - 2]
        ) {
            m_polymer.pop_back();
            m_polymer.pop_back();
        }
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string input;
    std::getline(file, input);
    file.close();


    Polymer polymer{};
    Polymer p_without[26]{};

    for (const auto c : input) {
        polymer.add_reduce(c);

        for (int i = 0; i < 26; ++i) {
            if (i == std::tolower(c) - 'a') {
                continue;
            }

            p_without[i].add_reduce(c);
        }
    }

    // Part 1
    std::cout << "Remaining: " << polymer.size() << '\n';

    // Part 2
    const auto shortest = std::ranges::min_element(p_without, {}, [](auto e) { return e.size(); });
    std::cout << "Shortest: " << shortest->size() << '\n';

    return EXIT_SUCCESS;
}
