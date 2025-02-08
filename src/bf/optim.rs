/// Optimizes series of instructions that have no effect, e.g. <><> or +-+-
/// TODO
pub fn optimize_no_effect() {}

pub fn remove_comments(bf: String) -> String {
    bf.chars().filter(|&c| "+-><[].,".contains(c)).collect()
}
