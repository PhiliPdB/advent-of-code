#include <algorithm>
#include <chrono>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>
#include <string>


class Guard {
public:
    int asleep[60];
    int total_sleep;

    Guard(): asleep{}, total_sleep{ 0 } {
    }

    void add_sleep(const int from, const int to) {
        total_sleep += to - from;
        for (int m = from; m < to; ++m) {
            asleep[m] += 1;
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
    std::string line;
    std::vector<std::string> input;
    while (std::getline(file, line)) {
        input.push_back(line);
    }
    file.close();


    // Sort input
    std::ranges::sort(input, {}, [](auto e) {
        auto time{ std::chrono::sys_seconds{} };
        auto string_stream{ std::istringstream(e.substr(1, 16)) };
        string_stream >> parse(std::string{ "%Y-%m-%d %H:%M" }, time);

        return time;
    });

    // Count sleeping hours per guard

    std::unordered_map<int, Guard> guards;
    int last_guard{ 0 };
    int asleep_since{ 0 };

    for (std::string_view string : input) {
        const auto minute_start = string.find(':');

        const auto minute = std::stoi(string.substr(minute_start + 1, 2).data());
        if (string.ends_with("falls asleep")) {
            asleep_since = minute;
        } else if (string.ends_with("wakes up")) {
            guards[last_guard].add_sleep(asleep_since, minute);
        } else {
            // Parse guard to set the last guard id
            const auto tag = string.find('#');
            const auto begins = string.find(" begins");

            const auto id = std::stoi(string.substr(tag + 1, begins - tag - 1).data());

            last_guard = id;
        }
    }

    // Part 1

    const auto part1_sleepy_guard{ std::ranges::max_element(guards, {}, [](auto e) { return e.second.total_sleep; }) };
    const auto part1_sleepy_minute{ std::ranges::max_element(part1_sleepy_guard->second.asleep) - part1_sleepy_guard->second.asleep };

    std::cout << "Strategy 1 score: " << part1_sleepy_guard->first * part1_sleepy_minute << '\n';


    // Part 2

    int part2_sleepy_guard{ 0 };
    int part2_sleepy_minute{ 0 };
    int highest_value{ 0 };
    for (int m = 0; m < 60; ++m) {
        if (const auto guard{ std::ranges::max_element(guards, {}, [m](auto e) { return e.second.asleep[m]; })};
            guard->second.asleep[m] > highest_value
        ) {
            highest_value = guard->second.asleep[m];
            part2_sleepy_minute = m;
            part2_sleepy_guard = guard->first;
        }
    }

    std::cout << "Strategy 2 score: " << part2_sleepy_guard * part2_sleepy_minute << '\n';

    return EXIT_SUCCESS;
}
