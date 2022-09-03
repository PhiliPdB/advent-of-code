#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <ostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

enum class Operation {
    Snd,
    Set, Add, Mul, Mod,
    Rcv,
    Jgz
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
        { 'f', 0 },
        { 'i', 0 },
        { 'p', 0 },
    };

    std::int64_t m_last_played_sound{ 0 };

public:
    explicit ProgramRunner(std::vector<Instruction> instructions, const int id = 0) : m_instructions{ std::move(instructions) } {
        m_registers['p'] = id;
    }

    std::int64_t first_recovered_sound() {
        while (m_instruction_pointer >= 0 && m_instruction_pointer < static_cast<int>(m_instructions.size())) {
            const auto current_instruction{ m_instructions[m_instruction_pointer] };
            switch (current_instruction.operation) {
                case Operation::Snd:
                    m_last_played_sound = current_instruction.first_argument_value(m_registers);
                    break;
                case Operation::Set:
                    m_registers[static_cast<char>(current_instruction.first_argument)] = current_instruction.second_argument_value(m_registers);
                    break;
                case Operation::Add:
                    m_registers[static_cast<char>(current_instruction.first_argument)] += current_instruction.second_argument_value(m_registers);
                    break;
                case Operation::Mul:
                    m_registers[static_cast<char>(current_instruction.first_argument)] *= current_instruction.second_argument_value(m_registers);
                    break;
                case Operation::Mod:
                    m_registers[static_cast<char>(current_instruction.first_argument)] %= current_instruction.second_argument_value(m_registers);
                    if (m_registers[static_cast<char>(current_instruction.first_argument)] < 0) {
                        m_registers[static_cast<char>(current_instruction.first_argument)] += current_instruction.second_argument_value(m_registers);
                    }
                    break;
                case Operation::Rcv:
                    if (current_instruction.first_argument_value(m_registers) != 0) {
                        return m_last_played_sound;
                    }
                    break;
                case Operation::Jgz:
                    if (current_instruction.first_argument_value(m_registers) > 0) {
                        m_instruction_pointer += static_cast<int>(current_instruction.second_argument_value(m_registers)) - 1;
                    }
                    break;
                default:
                    throw std::runtime_error("Unknown instruction");
            }

            ++m_instruction_pointer;
        }

        throw std::runtime_error("Unreachable");
    }

    void run(std::deque<std::int64_t>& rcv_queue, std::deque<std::int64_t>& snd_queue, int& send_amount) {
        while (m_instruction_pointer >= 0 && m_instruction_pointer < static_cast<int>(m_instructions.size())) {
            const auto current_instruction{ m_instructions[m_instruction_pointer] };
            switch (current_instruction.operation) {
            case Operation::Snd:
                snd_queue.push_back(current_instruction.first_argument_value(m_registers));
                ++send_amount;
                break;
            case Operation::Set:
                m_registers[static_cast<char>(current_instruction.first_argument)] = current_instruction.second_argument_value(m_registers);
                break;
            case Operation::Add:
                m_registers[static_cast<char>(current_instruction.first_argument)] += current_instruction.second_argument_value(m_registers);
                break;
            case Operation::Mul:
                m_registers[static_cast<char>(current_instruction.first_argument)] *= current_instruction.second_argument_value(m_registers);
                break;
            case Operation::Mod:
                m_registers[static_cast<char>(current_instruction.first_argument)] %= current_instruction.second_argument_value(m_registers);
                if (m_registers[static_cast<char>(current_instruction.first_argument)] < 0) {
                    m_registers[static_cast<char>(current_instruction.first_argument)] += current_instruction.second_argument_value(m_registers);
                }
                break;
            case Operation::Rcv:
                if (rcv_queue.empty()) {
                    return;
                } else {
                    m_registers[static_cast<char>(current_instruction.first_argument)] = rcv_queue.front();
                    rcv_queue.pop_front();
                }
                break;
            case Operation::Jgz:
                if (current_instruction.first_argument_value(m_registers) > 0) {
                    m_instruction_pointer += static_cast<int>(current_instruction.second_argument_value(m_registers)) - 1;
                }
                break;
            default:
                throw std::runtime_error("Unknown instruction");
            }

            ++m_instruction_pointer;
        }
    }
};

class RunnerContainer {
private:
    std::deque<std::int64_t> m_runner_1_queue{};
    ProgramRunner m_runner_1;

    std::deque<std::int64_t> m_runner_2_queue{};
    ProgramRunner m_runner_2;

    int m_runner_1_sent_amount{ 0 };
    int m_runner_2_sent_amount{ 0 };

public:
    explicit RunnerContainer(const std::vector<Instruction>& instructions)
        : m_runner_1{ instructions, 0 }, m_runner_2{ instructions, 1 } {
    }

    int run() {
        do {
            m_runner_1.run(m_runner_1_queue, m_runner_2_queue, m_runner_1_sent_amount);
            m_runner_2.run(m_runner_2_queue, m_runner_1_queue, m_runner_2_sent_amount);
        } while (!m_runner_1_queue.empty() || !m_runner_2_queue.empty());

        return m_runner_2_sent_amount;
    }
};

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
        if      (match[1] == "snd") op = Operation::Snd;
        else if (match[1] == "set") op = Operation::Set;
        else if (match[1] == "add") op = Operation::Add;
        else if (match[1] == "mul") op = Operation::Mul;
        else if (match[1] == "mod") op = Operation::Mod;
        else if (match[1] == "rcv") op = Operation::Rcv;
        else if (match[1] == "jgz") op = Operation::Jgz;
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
    std::cout << "[Part 1] First recovered sound: " << runner.first_recovered_sound() << '\n';

    // Part 2
    RunnerContainer container{ instructions };
    std::cout << "[Part 2] Program 1 sent something " << container.run() << " times \n";

    return EXIT_SUCCESS;
}
