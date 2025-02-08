/// Optimizes series of instructions that have no effect, e.g. <><> or +-+-
/// TODO
pub fn optimize_no_effect() {}

/// Removes all non-brainfuck characters from the input
pub fn remove_nonbf(bf: String) -> String {
    bf.chars().filter(|&c| "+-><[].,".contains(c)).collect::<String>().trim().to_string()
}
