use std::convert::TryInto;

fn main() {}

fn binary(target: &isize, data: &[usize]) -> isize {
    if data.len() == 0 {
        return -1;
    }
    let middle = data[data.len() / 2];
    println!("{}", middle);
    let utarget = target.clone() as usize;
    if utarget > data[data.len() - 1] {
        return -1;
    }
    if middle == utarget {
        println!("{}", middle);
        return (data.len() / 2).try_into().unwrap();
    } else if middle != data[data.len() - 1] && data[data.len() / 2 + 1] == utarget {
        return (data.len() / 2 + 1).try_into().unwrap();
    } else {
        let mut min = 0;
        let mut max = data.len() - 1;
        if middle > utarget {
            println!("HIGHER");
            max = data.len() / 2;
        } else {
            println!("LOWER");
            min = data.len() / 2;
        }
        println!("{} {}", min, max);
        return binary(target, &data[min..max]);
    }
}

#[cfg(test)]
mod test;
