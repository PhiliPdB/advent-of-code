#include <algorithm>
#include <cassert>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <unordered_set>
#include <vector>
#include <string>



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

    // Parse input
    std::map<char, std::unordered_set<char>> requirements;
    for (const std::string_view string : input) {
        const char dependent_on = string[5];

        requirements[string[36]].insert(dependent_on);
        if (!requirements.contains(dependent_on)) {
            requirements.insert({ dependent_on, {} });
        }
    }

    // Calculate order of operations

    auto req{ requirements };
    std::cout << "Order of operations: ";
    do {
        const auto next_step = std::ranges::min_element(req, {}, [](auto e) { return e.second.size(); });
        const auto operation = next_step->first;
        assert(next_step->second.empty());

        std::cout << next_step->first;

        req.erase(operation);
        for (auto& [_, set] : req) {
            set.erase(operation);
        }
    } while (!req.empty());
    std::cout << '\n';

    // Calculate time to complete
    std::vector<std::optional<std::pair<char, int>>> workers;
    workers.resize(5);

    auto current_time{ 0 };
    while (true) {
        // Check which workers are done
        for (auto& worker : workers) {
            if (!worker.has_value()) {
                continue;
            }

            if (const auto [job, time] = worker.value();
                time <= current_time
            ) {
                // Remove the job
                for (auto& [_, set] : requirements) {
                    set.erase(job);
                }

                worker = std::nullopt;
            }
        }

        // Fill workers with available jobs
        std::deque<char> available_jobs;
        for (const auto& [job, set] : requirements) {
            if (set.empty()) {
                available_jobs.push_back(job);
            }
        }

        for (auto& worker : workers) {
            if (!worker.has_value() && !available_jobs.empty()) {
                const auto job = available_jobs.front();
                available_jobs.pop_front();

                requirements.erase(job);

                worker = { job, current_time + 60 + 1 + (job - 'A') };
            }
        }

        // Go to next time something changes
        // ReSharper disable once CppTooWideScopeInitStatement
        const auto next_time{
            std::ranges::min(workers, {}, [](auto e) {
                return e.has_value() ? e->second : INT32_MAX;
            })
        };

        if (next_time.has_value()) {
            current_time = next_time.value().second;
        } else {
            break;
        }
    }

    std::cout << "Total construction time: " << current_time << '\n';


    return EXIT_SUCCESS;
}
