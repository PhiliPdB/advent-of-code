#include <fstream>
#include <iostream>
#include <string>
#include <vector>


int main() {
    std::vector<int> jumps;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        jumps.push_back(std::stoi(line));
    }
    file.close();


    // Part 1
    std::vector part1_jumps{ jumps };
    int part1_steps{ 0 };
    int part1_current_index{ 0 };
    while (0 <= part1_current_index && part1_current_index < static_cast<int>(part1_jumps.size())) {
        // NOTE: Increase the current offset after jumping with post-increment
        part1_current_index += part1_jumps[part1_current_index]++;
        ++part1_steps;
    }
    std::cout << "[Part 1] Reached exit in " << part1_steps << " steps\n";


    // Part 2
    std::vector part2_jumps{ jumps };
    int part2_steps{ 0 };
    int part2_current_index{ 0 };
    while (0 <= part2_current_index && part2_current_index < static_cast<int>(part2_jumps.size())) {
        auto& value{ part2_jumps[part2_current_index] };

        part2_current_index += value;
        if (value >= 3) {
            --value;
        } else {
            ++value;
        }

        ++part2_steps;
    }
    std::cout << "[Part 2] Reached exit in " << part2_steps << " steps\n";

    return EXIT_SUCCESS;
}
