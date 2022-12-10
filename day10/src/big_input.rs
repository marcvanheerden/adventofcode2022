use std::collections::HashSet;
pub(crate) fn create() -> String {
    let custom = vec![
        true, true, false, false, true, false, false, false, true, true, true, false, true, true,
        true, false, true, true, true, false, false, false, true, true, true, true, false, true,
        true, true, false, true, true, true, false, false, false, false, false, false, false,
        false, false, false, true, false, false, false, true, false, false, false, false, true,
        false, false, true, false, false, false, false, false, true, false, false, false, false,
        true, false, false, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, true, true, false, false, false,
        true, false, false, true, true, true, false, false, false, true, false, true, true, false,
        true, true, false, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, true, false, false, false, false,
        true, false, false, false, false, true, false, false, false, true, false, false, true,
        false, true, false, false, false, false, true, false, false, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        true, false, false, true, true, true, false, false, false, true, true, true, true, false,
        true, true, true, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, true, false, true, true, true,
        false, true, true, true, false, false, true, true, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, true, false, true, true, false, true, false, true, false,
        true, false, true, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, true, false, true, false, true, false, true,
        false, true, true, true, false, false, true, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, false, false, false, true, false, true, false,
        true, false, true, false, true, false, false, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, false, false, true, false, true,
        true, true, false, true, false, false, true, false, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, true, false, false, false,
        true, true, true, false, true, true, true, false, false, true, false, false, true, true,
        true, false, false, true, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, true, false, false, true, true, true, false, false,
        true, false, false, false, false, true, false, false, true, false, true, false, true,
        false, true, false, false, true, false, false, false, false, true, false, false, false,
        false, false, false, false, false, true, true, true, false, false, true, false, false,
        false, true, true, true, false, false, true, false, false, true, true, true, false, true,
        true, true, false, false, true, true, true, false, true, true, true, false, false, true,
        false, false, false, false, false, true, false, false, false, false, false, false, false,
        false, false, true, false, false, true, false, false, true, false, true, false, true,
        false, true, false, false, false, false, true, false, false, true, false, false, true,
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, false, false, true, false, false, true, false, true, false, true,
        false, false, true, false, true, true, true, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false,
    ];

    let mut custom_long = custom;

    for _ in 0..13 {
        custom_long.append(&mut custom_long.clone());
    }

    encode(&custom_long, 40)
}

pub(crate) fn encode(pix: &[bool], frame_width: usize) -> String {
    let mut paths: Vec<HashSet<usize>> = Vec::new();

    for (idx, pixel) in pix.iter().enumerate() {
        let idx_pos = idx - (idx / frame_width) * frame_width;
        let options: HashSet<_> = (0..frame_width)
            .filter(|opt| {
                let diff = if *opt > idx_pos {
                    *opt - idx_pos
                } else {
                    idx_pos - *opt
                };

                if *pixel {
                    diff <= 1
                } else {
                    diff > 1
                }
            })
            .collect();

        paths.push(options);
    }

    let mut reg_x: i32 = 1;
    let mut instr = "".to_owned();
    let mut idx = 0usize;

    while idx < (pix.len() - 3) {
        let options: Vec<_> = paths[idx + 2].intersection(&paths[idx + 3]).collect();
        if options.contains(&&(reg_x as usize)) {
            instr.push_str("noop\n");
            idx += 1;
        } else {
            let adjustment = (*options[0] as i32) - reg_x;
            reg_x += adjustment;
            instr.push_str(&format!("{} {}\n", "addx", adjustment));
            idx += 2;
        }
    }

    instr
}
