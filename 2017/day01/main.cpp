#include <fstream>
#include <iostream>
#include <string>


constexpr int captcha(const std::string& line, const int step) {
    int sum{ 0 };
    for (size_t i = 0; i < line.size(); ++i) {
        const auto current_digit{ line[i] - '0' };
        const auto next_digit{ line[(i + step) % line.size()] - '0' };

        if (current_digit == next_digit) {
            sum += current_digit;
        }
    }
    return sum;
}


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    std::getline(file, line);
    file.close();

    
    std::cout << "[Part 1] Captcha: " << captcha(line, 1) << '\n';
    std::cout << "[Part 2] Captcha: " << captcha(line, static_cast<int>(line.size() / 2)) << '\n';
    

    return EXIT_SUCCESS;
}
