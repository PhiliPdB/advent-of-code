#include <fstream>
#include <iostream>
#include <sstream>
#include <string>


int group_score(std::stringstream& stream, const int depth = 0) {
    int score{ 0 };
    while (true) {
        switch (stream.peek()) {
            case '{':
                stream.get();
                score += group_score(stream, depth + 1);
                break;
            case '}':
                stream.get();
                score += depth;
                return score;
            default:
                return score;
        }
    }
}

int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string stream;
    file >> stream;
    file.close();

    // Remove ignored characters
    std::size_t ignore_pos;
    while ((ignore_pos = stream.find_first_of('!')) != std::string::npos) {
        stream.replace(ignore_pos, 2, "");
    }

    // Remove garbage
    int removed_garbage{ 0 };

    std::size_t garbage_start;
    while ((garbage_start = stream.find_first_of('<')) != std::string::npos) {
        const std::size_t garbage_end = stream.find_first_of('>');
        stream.replace(garbage_start, garbage_end - garbage_start + 1, "");

        removed_garbage += static_cast<int>(garbage_end - garbage_start + 1) - 2;
    }

    // Remove group separators
    std::size_t group_sep_location;
    while ((group_sep_location = stream.find_first_of(',')) != std::string::npos) {
        stream.replace(group_sep_location, 1, "");
    }


    // Part 1
    std::stringstream string_stream{ stream };
    const int score{ group_score(string_stream) };
    std::cout << "Group score: " << score << '\n';

    // Part 2
    std::cout << "Removed garbage: " << removed_garbage << '\n';

    return EXIT_SUCCESS;
}
