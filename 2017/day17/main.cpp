#include <iostream>
#include <vector>


int spinlock(const int steps_per_insert, const int rounds) {
    std::vector circular_buffer{ 0 };
    circular_buffer.reserve(rounds);

    auto current_position{ 0 };
    for (int i = 1; i <= rounds; ++i) {
        // Step forward
        current_position += steps_per_insert;
        current_position %= static_cast<int>(circular_buffer.size());

        // Insert the item
        // Note: `i` is the next value
        circular_buffer.insert(circular_buffer.begin() + current_position + 1, i);

        // Make sure current position is at the newly inserted item
        ++current_position;
    }

    // Return the item after the last inserted item
    return circular_buffer[static_cast<std::size_t>(current_position) + 1];
}

int angry_spinlock(const int steps_per_insert, const int rounds) {
    int value_after_0{ 0 };

    int current_position{ 0 };
    int buffer_size{ 1 };
    for (int i = 1; i <= rounds; ++i) {
        // Step forward
        current_position += steps_per_insert;
        current_position %= buffer_size;

        // Insert the item, but we only care about which item if it is inserted after 0.
        ++buffer_size;
        if (current_position == 0) {
            value_after_0 = i;
        }

        // Make sure current position is at the newly inserted item
        ++current_position;
    }

    return value_after_0;
}

int main() {
    std::cout << "[Part 1] Value after 2017: " << spinlock(369, 2017) << '\n';
    std::cout << "[Part 1] Value after 2017: " << angry_spinlock(369, 50'000'000) << '\n';
    return EXIT_SUCCESS;
}
