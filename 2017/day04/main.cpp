#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>


int main() {
    std::vector<std::vector<std::string>> passphrases;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        std::vector<std::string> passphrase;
        std::stringstream ss{ line };

        std::string phrase;
        while (ss >> phrase) {
            passphrase.push_back(phrase);
        }

        passphrases.push_back(std::move(passphrase));
    }
    file.close();

    // Part 1
    int part1_valid_passphrases{ 0 };
    for (const auto& passphrase : passphrases) {
        std::unordered_set<std::string_view> phrases{};
        for (std::string_view phrase : passphrase) {
            phrases.insert(phrase);
        }

        if (phrases.size() == passphrase.size()) {
            ++part1_valid_passphrases;
        }
    }
    std::cout << "[Part 1] Valid passphrases: " << part1_valid_passphrases << '\n';

    // Part 2
    int part2_valid_passphrases{ 0 };
    for (const auto& passphrase : passphrases) {
        std::unordered_set<std::string> phrases{};
        for (const auto& phrase : passphrase) {
            std::string sorted_phrase{ phrase };
            std::ranges::sort(sorted_phrase);

            phrases.insert(sorted_phrase);
        }

        if (phrases.size() == passphrase.size()) {
            ++part2_valid_passphrases;
        }
    }
    std::cout << "[Part 2] Valid passphrases: " << part2_valid_passphrases << '\n';


    return EXIT_SUCCESS;
}
