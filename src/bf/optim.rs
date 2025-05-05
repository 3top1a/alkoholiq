/// Optimizes series of instructions that have no effect, e.g. <><> or +-+-
pub fn optimize_no_effect(bf: String) -> String {
    // Literally remove <> and >< and +- and -+ until nothing changes

    let mut new = bf.clone();

    loop {
        let old = new.clone();
        new = new
            .replace("><", "")
            .replace("<>", "")
            .replace("+-", "")
            .replace("[-][-]", "[-]")
            .replace("-+", "");
        if old == new {
            break;
        }
    }

    new
}

/// Removes all non-brainfuck characters from the input
pub fn remove_non_brainfuck(bf: String) -> String {
    bf.chars()
        .filter(|&c| "+-><[].,".contains(c))
        .collect::<String>()
        .trim()
        .to_string()
}
