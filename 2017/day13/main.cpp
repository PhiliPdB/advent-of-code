#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

constexpr auto TOTAL_LAYERS{ 99 };

int severity(const std::vector<int>& layer_depth, const int delay = 0) {
    int severity{ 0 };
    std::vector scanner_locations(TOTAL_LAYERS, delay);
    for (int i = 0; i < TOTAL_LAYERS; ++i) {
        if (layer_depth[i] != -1) {
            scanner_locations[i] %= 2 * layer_depth[i] - 2;
        }
    }

    for (int i = 0; i < TOTAL_LAYERS; ++i) {
        // Enter layer
        if (layer_depth[i] != -1 && scanner_locations[i] == 0) {
            severity += (i + delay) * layer_depth[i];
        }

        // Move scanners
        for (int s = 0; s < TOTAL_LAYERS; ++s) {
            if (layer_depth[s] == -1) continue;

            scanner_locations[s] += 1;
            scanner_locations[s] %= 2 * layer_depth[s] - 2;
        }
    }

    return severity;
}

int main() {
    std::vector layer_depth(TOTAL_LAYERS, -1);

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        std::regex regex{ R"(^(\d+): (\d+)$)" };

        std::smatch match;
        std::regex_match(line, match, regex);

        layer_depth[std::stoi(match[1])] = std::stoi(match[2]);
    }
    file.close();

    // Part 1
    std::cout << "[Part 1] Severity: " << severity(layer_depth) << '\n';

    // Part 2
    int delay{ 1 };
    while (true) {
        bool encounters_scanner{ false };
        for (int i = 0; i < TOTAL_LAYERS; ++i) {
            if (layer_depth[i] != -1 && (delay + i) % (2 * layer_depth[i] - 2) == 0) {
                encounters_scanner = true;
                break;
            }
        }

        if (!encounters_scanner) {
            break;
        }

        ++delay;
    }

    // Found the delay
    std::cout << "[Part 2] Delay: " << delay << '\n';
    return EXIT_SUCCESS;
}

