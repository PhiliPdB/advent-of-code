#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>


enum class InstructionAction {
    Increment,
    Decrement,
};

enum class ConditionCheck {
    GreaterThan, GreaterThanEqualTo,
    LessThan, LessThanEqualTo,
    EqualTo, NotEqualTo,
};

class Instruction {
private:
    std::string m_register;

    InstructionAction m_action;
    int m_action_amount;

    std::string m_condition_register;
    ConditionCheck m_condition_check;
    int m_condition_value;

public:
    Instruction(std::string register_, const InstructionAction action, const int action_amount,
        std::string condition_register, const ConditionCheck condition_check, const int condition_value)
        : m_register{std::move(register_)},
          m_action{action},
          m_action_amount{action_amount},
          m_condition_register{std::move(condition_register)},
          m_condition_check{condition_check},
          m_condition_value{condition_value} {
    }

    void execute(std::unordered_map<std::string, int>& registers) const {
        if (check_condition(registers)) {
            switch (m_action) {
                case InstructionAction::Increment:
                    registers[m_register] += m_action_amount;
                    break;
                case InstructionAction::Decrement:
                    registers[m_register] -= m_action_amount;
                    break;
                default:
                    throw std::runtime_error("Unreachable");
            }
        }
    }

private:
    [[nodiscard]]
    bool check_condition(std::unordered_map<std::string, int>& registers) const {
        switch (m_condition_check) {
            case ConditionCheck::GreaterThan:
                return registers[m_condition_register] >  m_condition_value;
            case ConditionCheck::GreaterThanEqualTo:
                return registers[m_condition_register] >= m_condition_value;
            case ConditionCheck::LessThan:
                return registers[m_condition_register] <  m_condition_value;
            case ConditionCheck::LessThanEqualTo:
                return registers[m_condition_register] <= m_condition_value;
            case ConditionCheck::EqualTo:
                return registers[m_condition_register] == m_condition_value;
            case ConditionCheck::NotEqualTo:
                return registers[m_condition_register] != m_condition_value;
            default:
                throw std::runtime_error("Unreachable");
        }
    }
};


int main() {
    std::unordered_map<std::string, int> registers;
    std::vector<Instruction> instructions;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }
    std::string line;
    while (std::getline(file, line)) {
        const std::regex regex{ R"(^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (>|>=|<|<=|==|!=) (-?\d+)$)" };

        std::smatch match;
        std::regex_match(line, match, regex);

        InstructionAction action{ match[2] == "inc" ? InstructionAction::Increment : InstructionAction::Decrement };
        ConditionCheck condition_check;
        if (match[5] == ">") {
            condition_check = ConditionCheck::GreaterThan;
        } else if (match[5] == ">=") {
            condition_check = ConditionCheck::GreaterThanEqualTo;
        } else if (match[5] == "<") {
            condition_check = ConditionCheck::LessThan;
        } else if (match[5] == "<=") {
            condition_check = ConditionCheck::LessThanEqualTo;
        } else if (match[5] == "==") {
            condition_check = ConditionCheck::EqualTo;
        } else if (match[5] == "!=") {
            condition_check = ConditionCheck::NotEqualTo;
        } else {
            throw std::runtime_error("Invalid condition");
        }

        instructions.emplace_back(match[1], action, std::stoi(match[3]), match[4], condition_check, std::stoi(match[6]));
    }
    file.close();

    int highest_register_value{ 0 };

    // Execute
    for (const auto& instruction : instructions) {
        instruction.execute(registers);

        const auto current_highest_register{ std::ranges::max_element(registers, {}, [](auto e) { return e.second; })->second };
        if (current_highest_register > highest_register_value) {
            highest_register_value = current_highest_register;
        }
    }

    // Part 1
    auto current_max_register{ std::ranges::max_element(registers, {}, [](auto e) { return e.second; }) };
    std::cout << "Biggest register: " << current_max_register->first << " with a value of " << current_max_register->second << '\n';

    // Part 2
    std::cout << "Highest recorded value: " << highest_register_value << '\n';

    return EXIT_SUCCESS;
}
