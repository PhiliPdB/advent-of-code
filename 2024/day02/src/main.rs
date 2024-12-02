
fn is_safe(report: &[u32]) -> bool {
    let is_increasing = report[0] < report[report.len() - 1];

    for w in report.windows(2) {
        if u32::abs_diff(w[0], w[1]) > 3
            || w[0] == w[1]
            || (is_increasing && w[0] > w[1])
            || (!is_increasing && w[0] < w[1])
        {
            return false;
        }
    }

    true
}

fn main() {
    let reports: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let safe_reports = reports.iter()
        .filter(|r| is_safe(r))
        .count();
    println!("[Part 1] Safe reports: {safe_reports}");

    let problem_dampener_safe_reports = reports.iter()
        .filter(|r| {
            if is_safe(r) {
                return true;
            }

            for i in 0..r.len() {
                let mut new_report = (*r).clone();
                new_report.remove(i);

                if is_safe(&new_report) {
                    return true;
                }
            }

            false
        })
        .count();
    println!("[Part 2] Safe reports: {problem_dampener_safe_reports}");
}
