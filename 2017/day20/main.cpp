#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

class Particle {
public:
    int px, py, pz;
    int vx, vy, vz;
    int ax, ay, az;

    Particle(
        const int px, const int py, const int pz,
        const int vx, const int vy, const int vz,
        const int ax, const int ay, const int az)
        : px{px}, py{py}, pz{pz},
          vx{vx}, vy{vy}, vz{vz},
          ax{ax}, ay{ay}, az{az} {
    }

    [[nodiscard]]
    int acceleration() const {
        return std::abs(ax) + std::abs(ay) + std::abs(az);
    }
};

constexpr bool intersects(
    const int p1, const int v1, const int a1,
    const int p2, const int v2, const int a2
) {
    // Use quadratic formula to find intersection points
    const auto a{ 0.5 * (a1 - a2) };
    const auto b{ (v1 - v2) + 0.5 * (a1 - a2) };
    const auto c{ static_cast<double>(p1 - p2) };

    const auto d{ b * b - 4 * a * c };
    if (d < 0) {
        return false;
    } else if (d == 0) {
        const auto t{ -b / (2 * a) };

        return t >= 0 && t == std::round(t);
    } else {
        const auto square_root{ std::sqrt(d) };
        const auto t1{ (-b + square_root) / (2 * a) };
        const auto t2{ (-b - square_root) / (2 * a) };

        return (t1 >= 0 && t1 == std::round(t1))
            || (t2 >= 0 && t2 == std::round(t2));
    }
}

int main() {
    std::vector<Particle> particles;

    // Read input
    std::ifstream file{ "input.txt" };
    if (!file) {
        std::cerr << "Can't open file: input.txt\n";
        return EXIT_FAILURE;
    }

    const std::regex regex{ R"(^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$)" };
    std::string line;
    while (std::getline(file, line)) {
        std::smatch match;
        std::regex_match(line, match, regex);

        particles.emplace_back(
            std::stoi(match[1]), std::stoi(match[2]), std::stoi(match[3]),
            std::stoi(match[4]), std::stoi(match[5]), std::stoi(match[6]),
            std::stoi(match[7]), std::stoi(match[8]), std::stoi(match[9])
        );
    }
    file.close();

    // Part 1
    const auto closest_particle_index{
        std::ranges::min_element(particles, {}, [](const auto& e) { return e.acceleration(); }) - particles.begin()
    };
    std::cout << "[Part 1] Closest particle: " << closest_particle_index << '\n';

    // Part 2
    int non_colliding_particles{ 0 };
    for (int i = 0; i < static_cast<int>(particles.size()); ++i) {
        bool found_collision{ false };

        for (int j = 0; j < static_cast<int>(particles.size()); ++j) {
            if (i == j) continue;

            // Check if i and j will ever collide

            // Mathematical solution:
            // p(t) = 1/2 * a_0 * t^2 + (v_0 + 1/2 a_0) * t + p_0

            const auto& p1{ particles[i] };
            const auto& p2{ particles[j] };

            if (intersects(p1.px, p1.vx, p1.ax, p2.px, p2.vx, p2.ax)
                && intersects(p1.py, p1.vy, p1.ay, p2.py, p2.vy, p2.ay)
                && intersects(p1.pz, p1.vz, p1.az, p2.pz, p2.vz, p2.az)) {
                found_collision = true;
                break;
            }
        }

        if (!found_collision) {
            ++non_colliding_particles;
        }
    }
    std::cout << "[Part 2] Particles left: " << non_colliding_particles << '\n';
    
    return EXIT_SUCCESS;
}
