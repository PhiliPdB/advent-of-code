#include <iostream>


constexpr auto GEN_A_START{ 618 };
constexpr auto GEN_B_START{ 814 };


class GeneratorBattle {
public:
    constexpr static std::uint64_t GEN_A_FACTOR{ 16807 };
    constexpr static std::uint64_t GEN_B_FACTOR{ 48271 };

private:
    std::uint32_t m_gen_a_start;
    std::uint32_t m_gen_b_start;

public:
    constexpr GeneratorBattle(const std::uint32_t gen_a_start, const std::uint32_t gen_b_start)
        : m_gen_a_start{ gen_a_start }, m_gen_b_start{ gen_b_start } {
    }

    template<bool Picky>
    [[nodiscard]]
    constexpr int matching_pairs(const int rounds) const {
        auto gen_a{ m_gen_a_start };
        auto gen_b{ m_gen_b_start };

        int matches{ 0 };
        for (int i = 0; i < rounds; ++i) {
            gen_a = next_gen_a_value<Picky>(gen_a);
            gen_b = next_gen_b_value<Picky>(gen_b);

            if ((gen_a & 0xffff) == (gen_b & 0xffff)) {
                ++matches;
            }
        }
        return matches;
    }

private:

    template<bool Picky>
    [[nodiscard]]
    constexpr static std::uint32_t next_gen_a_value(std::uint32_t value) {
#pragma warning(push)
#pragma warning(disable: 4127)
        do {
            value = (value * GEN_A_FACTOR) % 2147483647;
        } while (Picky && value % 4 != 0);
#pragma warning(pop)
        return value;
    }

    template<bool Picky>
    [[nodiscard]]
    constexpr static std::uint32_t next_gen_b_value(std::uint32_t value) {
#pragma warning(push)
#pragma warning(disable: 4127)
        do {
            value = (value * GEN_B_FACTOR) % 2147483647;
        } while (Picky && value % 8 != 0);
#pragma warning(pop)
        return value;
    }
};

int main() {
    constexpr GeneratorBattle battle{ GEN_A_START, GEN_B_START };
    std::cout << "[Part 1] Matching Pairs: " << battle.matching_pairs<false>(40'000'000) << '\n';
    std::cout << "[Part 2] Matching Pairs: " << battle.matching_pairs<true>(5'000'000) << '\n';

    return EXIT_SUCCESS;
}
