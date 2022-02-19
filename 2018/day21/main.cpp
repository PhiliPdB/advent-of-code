#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>


enum class OpCode {
    AddR, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtiR, GtrI, GtrR,
    EqiR, EqrI, EqrR,
};

constexpr OpCode op_code_from_string(const std::string_view string) {
    if (string == "addr") {
        return OpCode::AddR;
    } else if (string == "addi") {
        return OpCode::AddI;
    } else if (string == "mulr") {
        return OpCode::MulR;
    } else if (string == "muli") {
        return OpCode::MulI;
    } else if (string == "banr") {
        return OpCode::BanR;
    } else if (string == "bani") {
        return OpCode::BanI;
    } else if (string == "borr") {
        return OpCode::BorR;
    } else if (string == "bori") {
        return OpCode::BorI;
    } else if (string == "setr") {
        return OpCode::SetR;
    } else if (string == "seti") {
        return OpCode::SetI;
    } else if (string == "gtir") {
        return OpCode::GtiR;
    } else if (string == "gtri") {
        return OpCode::GtrI;
    } else if (string == "gtrr") {
        return OpCode::GtrR;
    } else if (string == "eqir") {
        return OpCode::EqiR;
    } else if (string == "eqri") {
        return OpCode::EqrI;
    } else if (string == "eqrr") {
        return OpCode::EqrR;
    }

    throw std::runtime_error("Unexpected instruction");
}

struct Instruction {
    OpCode op_code;
    std::array<int, 3> args;
};

class Program {
private:
    std::array<int64_t, 6> m_registers{ 0, 0, 0, 0, 0, 0 };
    std::vector<Instruction> m_instructions;
    int m_ip_register;

public:
    constexpr Program(std::vector<Instruction> instructions, const int ip_register) noexcept
        : m_instructions{ std::move(instructions) }, m_ip_register{ ip_register } {
    }

    static Program parse(std::ifstream& file) {
        std::vector<Instruction> instructions;
        int ip_register{ 0 };

        std::string line;
        while (std::getline(file, line)) {
            if (line.starts_with("#ip")) {
                ip_register = line[4] - '0';
                continue;
            }

            // Parse instruction
            std::array<std::string, 4> parts;
            auto ss{ std::istringstream(line) };
            for (int i = 0; i < 4; ++i) {
                ss >> parts[i];
            }
            auto op_code{ op_code_from_string(parts[0]) };
            std::array args{ std::stoi(parts[1]), std::stoi(parts[2]), std::stoi(parts[3]) };
            instructions.emplace_back(op_code, args);
        }

        return { std::move(instructions), ip_register };
    }

    [[nodiscard]]
    constexpr int64_t get_register_value(const int index) const {
        return m_registers[index];
    }

    constexpr void set_register_value(const int index, const int64_t value) {
        m_registers[index] = value;
    }

    constexpr int64_t run() {
        while (static_cast<size_t>(m_registers[m_ip_register]) < m_instructions.size()) {
            // Execute instruction
            switch (const auto& [op_code, args] { m_instructions[m_registers[m_ip_register]] }; op_code) {
                case OpCode::AddR:
                    m_registers[args[2]] = m_registers[args[0]] + m_registers[args[1]];
                    break;
                case OpCode::AddI:
                    m_registers[args[2]] = m_registers[args[0]] + args[1];
                    break;
                case OpCode::MulR:
                    m_registers[args[2]] = m_registers[args[0]] * m_registers[args[1]];
                    break;
                case OpCode::MulI:
                    m_registers[args[2]] = m_registers[args[0]] * args[1];
                    break;
                case OpCode::BanR:
                    m_registers[args[2]] = m_registers[args[0]] & m_registers[args[1]];
                    break;
                case OpCode::BanI:
                    m_registers[args[2]] = m_registers[args[0]] & args[1];
                    break;
                case OpCode::BorR:
                    m_registers[args[2]] = m_registers[args[0]] | m_registers[args[1]];
                    break;
                case OpCode::BorI:
                    m_registers[args[2]] = m_registers[args[0]] | args[1];
                    break;
                case OpCode::SetR:
                    m_registers[args[2]] = m_registers[args[0]];
                    break;
                case OpCode::SetI:
                    m_registers[args[2]] = args[0];
                    break;
                case OpCode::GtiR:
                    m_registers[args[2]] = args[0] > m_registers[args[1]];
                    break;
                case OpCode::GtrI:
                    m_registers[args[2]] = m_registers[args[0]] > args[1];
                    break;
                case OpCode::GtrR:
                    m_registers[args[2]] = m_registers[args[0]] > m_registers[args[1]];
                    break;
                case OpCode::EqiR:
                    m_registers[args[2]] = args[0] == m_registers[args[1]];
                    break;
                case OpCode::EqrI:
                    m_registers[args[2]] = m_registers[args[0]] == args[1];
                    break;
                case OpCode::EqrR:
                    m_registers[args[2]] = m_registers[args[0]] == m_registers[args[1]];
                    break;
            }

            // Increment the instruction pointer
            ++m_registers[m_ip_register];

            if (m_registers[m_ip_register] == 28) {
                return m_registers[3];
            }
        }

        return 0;
    }
};


int main() {
    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    auto program{ Program::parse(file) };
    file.close();

    // Part 1
    Program part1_program{ program };
    std::cout << "[Part 1] Answer: " << part1_program.run() << '\n';

    // Part 2
    uint64_t last_value{ 0 };
    std::unordered_set<uint64_t> seen_values{};
    while (true) {
        if (const auto v{ program.run() }; seen_values.insert(v).second) {
            last_value = v;
        } else {
            break;
        }
    }
    std::cout << "[Part 2] Answer: " << last_value << '\n';

    return EXIT_SUCCESS;
}
