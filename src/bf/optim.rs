use std::collections::HashSet;
use std::fmt::format;

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

pub fn remove_redundant_zeros(bf: String) -> String {
    // Removes all `[-]` that do not do anything
    // At this stage we need `#`/instruction separators to be kept

    let split = bf.split("[-]");
    let mut output_code = String::new();
    let mut pointer = 0;
    let mut bracket_index = 0;
    let mut used_variables = vec![];

    let mut debug_index = 0;

    for code in split {
        debug_index += 1;
        for char in code.chars() {
            match char {
                '<' => pointer -= 1,
                '>' => pointer += 1,
                '[' => {
                    bracket_index += 1;
                    if !used_variables.contains(&pointer) {
                        used_variables.push(pointer)
                    }
                }
                ']' => {
                    bracket_index -= 1;
                    if !used_variables.contains(&pointer) {
                        used_variables.push(pointer)
                    }
                }
                ',' | '.' | '+' | '-' => {
                    if !used_variables.contains(&pointer) {
                        used_variables.push(pointer)
                    }
                },
                '#' => {
                    // Remove all pointers in `used_variables` if they're temporary
                    used_variables.retain(|x| *x > 0);
                }
                _ => {},
            }
        }

        output_code += code;
        let mut should_add = true;

        // Do not add if the variable has not been used anytime
        // Or is temporary and a new instruction has just been generated (viz match case above)
        // Limited to code that is not inside any bracket because iteration makes it invalid
        if !used_variables.contains(&pointer) && bracket_index == 0 {
            should_add = false;
        }

        // output_code += &*format!("{debug_index}");
        if should_add {
            output_code += "[-]";
            used_variables.retain(|x| *x != pointer);
        }
    }

    output_code
}
