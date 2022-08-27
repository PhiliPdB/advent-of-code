#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>
#include <unordered_map>
#include <vector>


class ProgramDance;

class DanceMove {
public:
    virtual ~DanceMove() = default;

    virtual constexpr void move(std::array<char, 16>& programs) const = 0;
};

class ProgramDance {
private:
    std::array<char, 16> m_programs{ 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p' };

public:
    constexpr ProgramDance() = default;

    void dance(const std::vector<const DanceMove*>& moves) {
        for (const DanceMove* move : moves) {
            move->move(m_programs);
        }
    }

    [[nodiscard]]
    std::string to_string() const {
        std::string str;
        for (const char program : m_programs) {
            str.push_back(program);
        }
        return str;
    }

    friend std::ostream& operator<<(std::ostream& os, const ProgramDance& obj) {
        for (const char program : obj.m_programs) {
            os << program;
        }
        return os;
    }
};

class SpinMove : public DanceMove {
private:
    int m_size;

public:
    constexpr SpinMove(const int size) : m_size{ size } {
    }

    constexpr void move(std::array<char, 16>& programs) const override {
        std::ranges::rotate(programs, programs.begin() + static_cast<std::ptrdiff_t>(programs.size() - m_size));
    }
};

class ExchangeMove : public DanceMove {
private:
    int m_a_index;
    int m_b_index;

public:
    constexpr ExchangeMove(const int a_index, const int b_index)
        : m_a_index{ a_index }, m_b_index{ b_index } {
    }

    constexpr void move(std::array<char, 16>& programs) const override {
        std::swap(programs[m_a_index], programs[m_b_index]);
    }
};

class PartnerMove : public DanceMove {
private:
    char m_a_program;
    char m_b_program;

public:
    constexpr PartnerMove(const char a_program, const char b_program)
        : m_a_program{ a_program }, m_b_program{ b_program } {
    }

    constexpr void move(std::array<char, 16>& programs) const override {
        const auto a_index{ std::ranges::find(programs, m_a_program) - programs.begin() };
        const auto b_index{ std::ranges::find(programs, m_b_program) - programs.begin() };

        std::swap(programs[a_index], programs[b_index]);
    }
};


int main() {
    std::vector<const DanceMove*> moves{};

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line, ',')) {
        switch (line[0]) {
            case 's':
                moves.push_back(new SpinMove(std::stoi(line.substr(1, line.size() - 1))));
                break;
            case 'x': {
                const auto brk_pnt{ line.find_first_of('/') };

                moves.push_back(new ExchangeMove(
                    std::stoi(line.substr(1, brk_pnt - 1)),
                    std::stoi(line.substr(brk_pnt + 1, line.size() - brk_pnt - 1))
                ));
                break;
            }
            case 'p':
                moves.push_back(new PartnerMove(line[1], line[3]));
                break;
            default:
                throw std::runtime_error("Invalid dance move");
        }
    }
    file.close();

    // Part 1
    ProgramDance dance{};
    dance.dance(moves);
    std::cout << "[Part 1] Order: " << dance << '\n';

    // Part 2
    std::vector<std::string> orders { "abcdefghijklmnop" };

    int round{ 1 };
    while (true) {
        std::string order{ dance.to_string() };

        if (order == "abcdefghijklmnop") {
            std::cout << "[Part 2] Order: " << orders[1'000'000'000 % round] << '\n';
            return EXIT_SUCCESS;
        }
        orders.push_back(order);

        // Another dancing round
        dance.dance(moves);
        ++round;
    }
}
