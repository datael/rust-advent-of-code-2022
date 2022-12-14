use lib::read_all_lines_from_stdin;

// Space needs to be cleared before the last supplies can be unloaded from the
// ships, and so several Elves have been assigned the job of cleaning up
// sections of the camp. Every section has a unique ID number, and each Elf is
// assigned a range of section IDs.

#[derive(PartialEq, PartialOrd)]
struct SectionId(u32);

struct SectionAssignment {
    from: SectionId,
    to: SectionId,
}

// However, as some of the Elves compare their section assignments with each
// other, they've noticed that many of the assignments overlap.

impl SectionAssignment {
    fn has_overlap_with(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.from
            || self.from <= other.to && self.to >= other.to
    }
}

// To try to
// quickly find overlaps and reduce duplicated effort, the Elves pair up and
// make a big list of the section assignments for each pair (your puzzle
// input).

struct AssignmentPair(SectionAssignment, SectionAssignment);

// For example, consider the following list of section assignment pairs:

// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

impl From<&String> for AssignmentPair {
    fn from(value: &String) -> Self {
        let (a, b) = value.split_once(',').unwrap();

        Self(a.into(), b.into())
    }
}

impl From<&str> for SectionAssignment {
    fn from(value: &str) -> Self {
        let (from, to) = value.split_once('-').unwrap();

        Self {
            from: SectionId(from.parse().unwrap()),
            to: SectionId(to.parse().unwrap()),
        }
    }
}

// Some of the pairs have noticed that one of their assignments fully contains
// the other.

impl SectionAssignment {
    fn fully_contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

// In pairs where one assignment fully contains the other, one Elf in
// the pair would be exclusively cleaning sections their partner will already
// be cleaning, so these seem like the most in need of reconsideration.

impl AssignmentPair {
    fn needs_reconsideration(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    fn has_any_overlap(&self) -> bool {
        self.0.has_overlap_with(&self.1) || self.1.has_overlap_with(&self.0)
    }
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    // In how many assignment pairs does one range fully contain the other?

    let part1_assignments_needing_reconsideration = input
        .iter()
        .map(AssignmentPair::from)
        .filter(AssignmentPair::needs_reconsideration)
        .count();

    println!(
        "Assignments needing reconsideration: {}",
        part1_assignments_needing_reconsideration
    );

    // It seems like there is still quite a bit of duplicate work planned.
    // Instead, the Elves would like to know the number of pairs that overlap at
    // all.

    // In how many assignment pairs do the ranges overlap?

    let part2_assignments_with_any_overlap = input
        .iter()
        .map(AssignmentPair::from)
        .filter(AssignmentPair::has_any_overlap)
        .count();

    println!(
        "Assignments with any overlap: {}",
        part2_assignments_with_any_overlap
    );
}
