#include <iostream>
#include <vector>


constexpr auto TOTAL_RECIPES = 540'391;
constexpr auto TOTAL_RECIPES_MOD = 1'000'000;


int main() {
    auto elf1_location{ 0 };
    auto elf2_location{ 1 };
    std::vector recipes{ 3, 7 };

    auto current_score{ 37 };

    while (true) {
        const auto score = recipes[elf1_location] + recipes[elf2_location];
        if (score < 10) {
            recipes.push_back(score);

            current_score *= 10;
            current_score += score;
        } else {
            // Add first recipe and check the score
            recipes.push_back(score / 10);
            current_score *= 10;
            current_score += score / 10;
            current_score %= TOTAL_RECIPES_MOD;
            if (current_score == TOTAL_RECIPES) {
                break;
            }

            // Add the second recipe
            recipes.push_back(score % 10);
            current_score *= 10;
            current_score += score % 10;
        }
        current_score %= TOTAL_RECIPES_MOD;
        if (current_score == TOTAL_RECIPES) {
            break;
        }

        elf1_location = (elf1_location + recipes[elf1_location] + 1) % static_cast<int>(recipes.size());
        elf2_location = (elf2_location + recipes[elf2_location] + 1) % static_cast<int>(recipes.size());
    }

    unsigned long long final_score{ 0 };
    for (int i = 0; i < 10; ++i) {
        final_score *= 10;
        final_score += recipes[TOTAL_RECIPES + i];
    }

    std::cout << "Score of next 10 recipes: " << final_score << '\n';
    std::cout << "Found " << TOTAL_RECIPES << " after " << recipes.size() - 6 << " steps\n";


    return EXIT_SUCCESS;
}
