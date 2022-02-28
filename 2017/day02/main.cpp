#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>


int main() {
    std::vector<std::vector<int>> rows;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        std::vector<int> row;
        std::stringstream ss{ line };

        int cell;
        while (ss >> cell) {
            row.push_back(cell);
        }

        rows.push_back(std::move(row));
    }
    file.close();


    int part1_checksum{ 0 };
    int part2_checksum{ 0 };
    for (const auto& row : rows) {
        // Part 1
        const auto [min, max] { std::ranges::minmax(row) };
        part1_checksum += max - min;

        // Part 2
        for (size_t i = 1; i < row.size(); ++i) {
            for (size_t j = 0; j < i; ++j) {
                if (row[i] % row[j] == 0) {
                    part2_checksum += row[i] / row[j];
                    goto next;
                } else if (row[j] % row[i] == 0) {
                    part2_checksum += row[j] / row[i];
                    goto next;
                }
            }
        }
    next:;
    }
    std::cout << "[Part 1] Checksum: " << part1_checksum << '\n';
    std::cout << "[Part 2] Checksum: " << part2_checksum << '\n';


    return EXIT_SUCCESS;
}
