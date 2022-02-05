#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_set>
#include <vector>
#include <string>

struct PairHash {
    template<class T1, class T2>
    std::size_t operator()(const std::pair<T1, T2>& pair) const noexcept {
        return std::hash<T1>()(pair.first) ^ std::hash<T2>()(pair.second);
    }
};


class Claim {
public:
    int id;
    std::pair<int, int> coordinate;
    std::pair<int, int> size;

    Claim(const int id, std::pair<int, int> coordinate, std::pair<int, int> size)
        : id(id), coordinate(std::move(coordinate)), size(std::move(size)) {
    }

    static Claim parse(const std::string_view line) {
        const auto at_pos = line.find(" @ ");
        const auto comma_pos = line.find(',');
        const auto colon_pos = line.find(": ");
        const auto x_pos = line.find('x');

        const auto id = std::stoi(line.substr(1, at_pos - 1).data());

        const auto coordinate_x = std::stoi(line.substr(at_pos + 3, comma_pos - at_pos - 3).data());
        const auto coordinate_y = std::stoi(line.substr(comma_pos + 1, colon_pos - comma_pos - 1).data());

        const auto size_w = std::stoi(line.substr(colon_pos + 2, x_pos - colon_pos - 2).data());
        const auto size_h = std::stoi(line.substr(x_pos + 1, line.size() - x_pos - 1).data());

        return { id, {coordinate_x, coordinate_y}, {size_w, size_h} };
    }

    [[nodiscard]]
    int area() const {
        return size.first * size.second;
    }

    [[nodiscard]]
    bool intersects(const std::pair<int, int> coord) const {
        return coordinate.first <= coord.first && coord.first < coordinate.first + size.first
            && coordinate.second <= coord.second && coord.second < coordinate.second + size.second;
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
    std::vector<Claim> input;
    while (std::getline(file, line)) {
        input.push_back(Claim::parse(line));
    }
    file.close();


    std::unordered_set<std::pair<int, int>, PairHash> claimed_twice;
    std::vector<bool> overlaps;
    // Fill the array with false values
    overlaps.resize(input.size(), false);


    for (auto i = 0; i < static_cast<int>(input.size()); ++i) {
        const auto& claim1{ input[i] };

        for (auto j = 0; j < i; ++j) {
            const auto& claim2{ input[j] };

            const auto claim1_bigger{ claim1.area() >= claim2.area() };
            const auto& bigger{ claim1_bigger ? claim1 : claim2 };
            const auto& smaller{ claim1_bigger ? claim2 : claim1 };

            for (int x = smaller.coordinate.first; x < smaller.coordinate.first + smaller.size.first; ++x) {
                for (int y = smaller.coordinate.second; y < smaller.coordinate.second + smaller.size.second; ++y) {
                    if (bigger.intersects({ x, y })) {
                        overlaps[claim1.id - 1] = true;
                        overlaps[claim2.id - 1] = true;
                        claimed_twice.insert({ x, y });
                    }
                }
            }
        }
    }

    std::cout << "Shared fabric: " << claimed_twice.size() << '\n';
    for (int i = 0; i < static_cast<int>(overlaps.size()); ++i) {
        if (!overlaps[i]) {
            std::cout << "Intact: " << i + 1 << '\n';
        }
    }

    return EXIT_SUCCESS;
}
