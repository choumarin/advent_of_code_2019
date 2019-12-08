pub fn entry_a(line: String) -> String {
    let mut v = to_instruction_vect(line);
    *v.get_mut(1).unwrap() = 12;
    *v.get_mut(2).unwrap() = 2;
    run_ops(&mut v).unwrap();
    v.get(0).unwrap().to_string()
}

pub fn entry_b(line: String) -> String {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut v = to_instruction_vect(line.clone());
            *v.get_mut(1).unwrap() = noun;
            *v.get_mut(2).unwrap() = verb;
            if run_ops(&mut v).is_ok() && *v.get(0).unwrap() == 19690720 {
                return (100 * noun + verb).to_string();
            }
        }
    }
    0.to_string()
}

fn to_instruction_vect(line: String) -> Vec<u32> {
    line.split(",")
        .map(|s| -> u32 { s.parse().unwrap() })
        .collect()
}

fn run_ops(ops: &mut Vec<u32>) -> Result<(), &str> {
    let mut cur = 0;
    loop {
        match ops.get(cur) {
            Some(i) => {
                match i {
                    1 | 2 => {
                        let opa_cur = *ops.get(cur + 1).unwrap() as usize;
                        let opb_cur = *ops.get(cur + 2).unwrap() as usize;
                        let result_cur = *ops.get(cur + 3).unwrap() as usize;
                        if *i == 1 {
                            *ops.get_mut(result_cur).unwrap() =
                                ops.get(opa_cur).unwrap() + ops.get(opb_cur).unwrap()
                        } else {
                            *ops.get_mut(result_cur).unwrap() =
                                ops.get(opa_cur).unwrap() * ops.get(opb_cur).unwrap()
                        }
                    }
                    99 => break Ok(()),
                    _ => return Err("Illegal opcode"),
                }
                cur += 4;
            }
            _ => return Err("Out of bound"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string(v: Vec<u32>) -> String {
        let mut s = v
            .into_iter()
            .map(|u| u.to_string() + ",")
            .collect::<String>();
        s.pop();
        s
    }

    fn run(s: String) -> String {
        let mut v = to_instruction_vect(s);
        run_ops(&mut v).unwrap();
        to_string(v)
    }

    #[test]
    fn to_from() {
        let s = String::from("1,2,3");
        let v = to_instruction_vect(s.clone());
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(to_string(v), s);
    }

    #[test]
    fn official_results_a() {
        assert_eq!(run(String::from("1,0,0,0,99")), "2,0,0,0,99");
        assert_eq!(run(String::from("2,3,0,3,99")), "2,3,0,6,99");
        assert_eq!(run(String::from("2,4,4,5,99,0")), "2,4,4,5,99,9801");
        assert_eq!(
            run(String::from("1,1,1,4,99,5,6,0,99")),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
