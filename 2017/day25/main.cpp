#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

enum class State {
    A, B, C, D, E, F,
};

class TuringMachine {
private:
    std::unordered_map<int, bool> m_tape;
    int m_cursor = 0;
    State m_state = State::A;

public:
    void next() {
        const bool current_value{ m_tape[m_cursor] };

        switch (m_state) {
            case State::A:
                if (current_value) {
                    m_tape[m_cursor] = false;
                    --m_cursor;
                } else {
                    m_tape[m_cursor] = true;
                    ++m_cursor;
                }
                m_state = State::B;
                break;
            case State::B:
                if (current_value) {
                    m_tape[m_cursor] = true;
                    --m_cursor;
                } else {
                    m_tape[m_cursor] = false;
                    ++m_cursor;
                    m_state = State::C;
                }
                break;
            case State::C:
                if (current_value) {
                    m_tape[m_cursor] = false;
                    --m_cursor;
                    m_state = State::A;
                } else {
                    m_tape[m_cursor] = true;
                    ++m_cursor;
                    m_state = State::D;
                }
                break;
            case State::D:
                m_tape[m_cursor] = true;
                --m_cursor;

                if (current_value) {
                    m_state = State::F;
                } else {
                    m_state = State::E;
                }
                break;
            case State::E:
                if (current_value) {
                    m_tape[m_cursor] = false;
                    --m_cursor;
                    m_state = State::D;
                } else {
                    m_tape[m_cursor] = true;
                    --m_cursor;
                    m_state = State::A;
                }
                break;
            case State::F:
                m_tape[m_cursor] = true;
                if (current_value) {
                    --m_cursor;
                    m_state = State::E;
                } else {
                    ++m_cursor;
                    m_state = State::A;
                }
                break;
            default:
                throw std::runtime_error("Unreachable");
        }
    }

    [[nodiscard]]
    int checksum() const {
        int checksum{ 0 };
        for (const auto& [pos, val] : m_tape) {
            if (val) {
                ++checksum;
            }
        }
        return checksum;
    }
};


int main() {
    TuringMachine machine{};
    for (int i = 0; i < 12'586'542; ++i) {
        machine.next();
    }

    std::cout << "Checksum: " << machine.checksum() << '\n';

    return EXIT_SUCCESS;
}
