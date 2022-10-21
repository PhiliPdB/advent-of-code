#include <algorithm>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

enum class Operation {
    Set,
    Sub, Mul,
    Jnz
};

struct Instruction {
    Operation operation;

    bool first_is_register;
    std::int64_t first_argument;

    bool second_is_register;
    std::int64_t second_argument;

    constexpr std::int64_t first_argument_value(std::unordered_map<char, std::int64_t>& registers) const {
        if (first_is_register) {
            return registers[static_cast<char>(first_argument)];
        } else {
            return first_argument;
        }
    }

    constexpr std::int64_t second_argument_value(std::unordered_map<char, std::int64_t>& registers) const {
        if (second_is_register) {
            return registers[static_cast<char>(second_argument)];
        } else {
            return second_argument;
        }
    }
};

class ProgramRunner {
private:
    std::vector<Instruction> m_instructions;

    int m_instruction_pointer{ 0 };
    std::unordered_map<char, std::int64_t> m_registers{
        { 'a', 0 },
        { 'b', 0 },
        { 'c', 0 },
        { 'd', 0 },
        { 'e', 0 },
        { 'f', 0 },
        { 'g', 0 },
        { 'h', 0 },
    };

public:
    explicit ProgramRunner(std::vector<Instruction> instructions) : m_instructions{ std::move(instructions) } {
    }

    void set_register(char reg, std::int64_t value) {
        m_registers[reg] = value;
    }

    int run() {
        int mul_instruction_calls{ 0 };

        while (m_instruction_pointer >= 0 && m_instruction_pointer < static_cast<int>(m_instructions.size())) {
            const auto current_instruction{ m_instructions[m_instruction_pointer] };
            switch (current_instruction.operation) {
                case Operation::Set:
                    m_registers[static_cast<char>(current_instruction.first_argument)] = current_instruction.second_argument_value(m_registers);
                    break;
                case Operation::Sub:
                    m_registers[static_cast<char>(current_instruction.first_argument)] -= current_instruction.second_argument_value(m_registers);
                    break;
                case Operation::Mul:
                    m_registers[static_cast<char>(current_instruction.first_argument)] *= current_instruction.second_argument_value(m_registers);
                    ++mul_instruction_calls;
                    break;
                case Operation::Jnz:
                    if (current_instruction.first_argument_value(m_registers) != 0) {
                        m_instruction_pointer += static_cast<int>(current_instruction.second_argument_value(m_registers)) - 1;
                    }
                    break;
                default:
                    throw std::runtime_error("Unknown instruction");
            }

            ++m_instruction_pointer;
        }

        return mul_instruction_calls;
    }
};


bool is_prime(const std::int64_t n) {
    if (n % 2 == 0) {
        return false;
    }

    int p = 3;
    while (p < std::sqrt(n) + 1) {
        if (n % p == 0) {
            return false;
        }

        p += 2;
    }
    return true;
}

int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    std::vector<Instruction> instructions;
    const std::regex regex{ R"(^([a-z]{3}) ([a-z]|-?\d+)( ([a-z]|-?\d+))?$)" };
    std::string line;
    while (std::getline(file, line)) {
        std::smatch match;
        std::regex_match(line, match, regex);
        // Parts:
        // match[1]: Operation
        // match[2]: First argument
        // match[4]: Second argument

        Operation op;
        if      (match[1] == "set") op = Operation::Set;
        else if (match[1] == "sub") op = Operation::Sub;
        else if (match[1] == "mul") op = Operation::Mul;
        else if (match[1] == "jnz") op = Operation::Jnz;
        else throw std::runtime_error("Unknown operation");


        const bool first_is_register{ !static_cast<bool>(std::isdigit(match[2].str().at(match[2].length() - 1))) };
        int first_argument;
        if (first_is_register) {
            first_argument = static_cast<std::int64_t>(match[2].str().at(0));
        } else {
            first_argument = std::stoi(match[2]);
        }

        bool second_is_register;
        int second_argument;
        if (match[4].matched) {
            second_is_register = !static_cast<bool>(std::isdigit(match[4].str().at(match[4].length() - 1)));
            if (second_is_register) {
                second_argument = static_cast<std::int64_t>(match[4].str().at(0));
            }
            else {
                second_argument = std::stoi(match[4]);
            }
        }

        instructions.emplace_back(op, first_is_register, first_argument, second_is_register, second_argument);
    }
    file.close();

    ProgramRunner runner{ instructions };
    // Part 1
    std::cout << "[Part 1] Mul instruction call count: " << runner.run() << '\n';

    // Part 2
    // Using hand decompiled code (also see other input file)
    std::int64_t b{ static_cast<std::int64_t>(99) * 100 + 100'000 };
    const std::int64_t c{ b + 17'000 };
    std::int64_t h{ 0 };

    while (b <= c) {
        if (!is_prime(b)) {
            ++h;
        }
        b += 17;
    }

    std::cout << "[Part 2] Final value of h: " << h << '\n';
    
    return EXIT_SUCCESS;
}
