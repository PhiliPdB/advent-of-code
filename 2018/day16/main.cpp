#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <optional>
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
    GtiR, GtiI, GtrR,
    EqiR, EqiI, EqrR,
};

constexpr std::array<int, 4> apply_op_code(const OpCode op, const std::array<int, 4> args, const std::array<int, 4> register_state) {
    std::array result{ register_state };
    switch (op) {
        case OpCode::AddR:
            result[args[3]] = result[args[1]] + result[args[2]];
            break;
        case OpCode::AddI:
            result[args[3]] = result[args[1]] + args[2];
            break;
        case OpCode::MulR:
            result[args[3]] = result[args[1]] * result[args[2]];
            break;
        case OpCode::MulI:
            result[args[3]] = result[args[1]] * args[2];
            break;
        case OpCode::BanR:
            result[args[3]] = result[args[1]] & result[args[2]];
            break;
        case OpCode::BanI:
            result[args[3]] = result[args[1]] & args[2];
            break;
        case OpCode::BorR:
            result[args[3]] = result[args[1]] | result[args[2]];
            break;
        case OpCode::BorI:
            result[args[3]] = result[args[1]] | args[2];
            break;
        case OpCode::SetR:
            result[args[3]] = result[args[1]];
            break;
        case OpCode::SetI:
            result[args[3]] = args[1];
            break;
        case OpCode::GtiR:
            result[args[3]] = args[1] > result[args[2]];
            break;
        case OpCode::GtiI:
            result[args[3]] = result[args[1]] > args[2];
            break;
        case OpCode::GtrR:
            result[args[3]] = result[args[1]] > result[args[2]];
            break;
        case OpCode::EqiR:
            result[args[3]] = args[1] == result[args[2]];
            break;
        case OpCode::EqiI:
            result[args[3]] = result[args[1]] == args[2];
            break;
        case OpCode::EqrR:
            result[args[3]] = result[args[1]] == result[args[2]];
            break;
    }
    return result;
}


class Sample {
private:
    std::array<int, 4> m_instruction;

    std::array<int, 4> m_before;
    std::array<int, 4> m_after;

public:
    Sample(const std::array<int, 4> instruction, const std::array<int, 4> before, const std::array<int, 4> after)
        : m_instruction(instruction),
          m_before(before), m_after(after) {
    }

    [[nodiscard]]
    constexpr int instruction_code() const {
        return m_instruction[0];
    }

    [[nodiscard]]
    constexpr int possible_op_codes() const {
        auto result{ 0 };
        for (int i = 0; i < 16; ++i) {
            if (apply_op_code(OpCode{ i }, m_instruction, m_before) == m_after) {
                ++result;
            }
        }
        return result;
    }

    [[nodiscard]]
    std::optional<OpCode> get_only_op_code(const std::unordered_set<OpCode>& possibilities) const {
        std::optional<OpCode> only_possibility;
        for (const auto& op : possibilities) {
            if (apply_op_code(op, m_instruction, m_before) == m_after) {
                if (only_possibility.has_value()) {
                    return {};
                } else {
                    only_possibility = std::make_optional(op);
                }
            }
        }
        return only_possibility;
    }
};

int main() {
    std::vector<Sample> samples;

    // Read input
    std::ifstream samples_file{ "input_samples.txt" };
    if (!samples_file) {
        std::cerr << "Can't open file: input_samples.txt\n";
        return EXIT_FAILURE;
    }
    while (!samples_file.eof()) {
        std::string before_line;
        std::getline(samples_file, before_line);
        std::array before{
            before_line[9] - '0', before_line[12] - '0', before_line[15] - '0', before_line[18] - '0'
        };

        std::string instruction_line;
        std::getline(samples_file, instruction_line);
        auto instruction_line_stream{ std::istringstream(instruction_line) };
        std::array<int, 4> instruction{};
        for (int i = 0; i < 4; ++i) {
            int current;
            instruction_line_stream >> current;
            instruction[i] = current;
        }

        std::string after_line;
        std::getline(samples_file, after_line);
        std::array after{
            after_line[9] - '0', after_line[12] - '0', after_line[15] - '0', after_line[18] - '0'
        };

        std::string dummy;
        std::getline(samples_file, dummy);

        samples.emplace_back(instruction, before, after);
    }
    samples_file.close();


    auto part1_answer{ 0 };
    for (const auto& sample : samples) {
        if (sample.possible_op_codes() >= 3) {
            ++part1_answer;
        }
    }
    std::cout << "Part 1 answer: " << part1_answer << '\n';

    std::array<OpCode, 16> op_code_map{};
    std::unordered_set op_codes_left{
        OpCode::AddR, OpCode::AddI, OpCode::MulR, OpCode::MulI, OpCode::BanR, OpCode::BanI, OpCode::BorR, OpCode::BorI,
        OpCode::SetR, OpCode::SetI, OpCode::GtiR, OpCode::GtiI, OpCode::GtrR, OpCode::EqiR, OpCode::EqiI, OpCode::EqrR,
    };
    while (!op_codes_left.empty()) {
        for (const auto& sample : samples) {
            if (const auto possibility{ sample.get_only_op_code(op_codes_left) };
                possibility.has_value()
            ) {
                op_codes_left.erase(*possibility);
                op_code_map[sample.instruction_code()] = *possibility;
            }
        }
    }

    // Execute the program

    std::vector<std::array<int, 4>> instructions;

    // Read program
    std::ifstream program_file{ "input_test_program.txt" };
    if (!program_file) {
        std::cerr << "Can't open file: input_test_program.txt\n";
        return EXIT_FAILURE;
    }
    std::string instruction_line;
    while (std::getline(program_file, instruction_line)) {
        auto instruction_line_stream{ std::istringstream(instruction_line) };
        std::array<int, 4> instruction{};
        for (int i = 0; i < 4; ++i) {
            int current;
            instruction_line_stream >> current;
            instruction[i] = current;
        }

        instructions.push_back(instruction);
    }
    program_file.close();


    std::array registers{ 0, 0, 0, 0 };
    for (const auto& instruction : instructions) {
        registers = apply_op_code(op_code_map[instruction[0]], instruction, registers);
    }
    std::cout << "Part 2 answer: " << registers[0] << '\n';

    return EXIT_SUCCESS;
}
