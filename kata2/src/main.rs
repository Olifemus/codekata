use std::convert::TryInto;

fn main() {
    binary(&2, &[1, 3, 5, 7]);
}

// fn binary(target: &isize, data: &[usize]) -> isize {
//     if data.len() == 0 {
//         return -1;
//     }
//     let middle = data[data.len() / 2];
//     println!("{}", middle);
//     let utarget = target.clone() as usize;
//     if utarget > data[data.len() - 1] {
//         return -1;
//     }
//     if middle == utarget {
//         println!("{}", middle);
//         return (data.len() / 2).try_into().unwrap();
//     } else if middle != data[data.len() - 1] && data[data.len() / 2 + 1] == utarget {
//         return (data.len() / 2 + 1).try_into().unwrap();
//     } else {
//         let mut min = 0;
//         let mut max = data.len() - 1;
//         if middle > utarget {
//             println!("HIGHER");
//             max = data.len() / 2;
//         } else {
//             println!("LOWER");
//             min = data.len() / 2;
//         }
//         println!("{} {}", min, max);
//         return binary(target, &data[min..max]);
//     }
// }

fn binary(target: &isize, data: &[usize]) -> isize {
    println!("--------------------------------");
    println!("{:?}", data);
    println!("{}", target);
    if data.len() == 0 {
        return -1;
    }
    let mut min = 0;
    let mut max = data.len() - 1;
    let utarget = target.clone() as usize;
    if utarget > data[data.len() - 1] {
        return -1;
    }
    loop {
        let marker = (min + max) / 2;
        let middle = data[marker];
        println!("{}, {}", marker, middle);
        if middle == utarget {
            println!("finded: {}", middle);
            return marker.try_into().unwrap();
        } else if middle != data[data.len() - 1] && data[data.len() / 2 + 1] == utarget {
            return (data.len() / 2 + 1).try_into().unwrap();
        } else {
            if middle > utarget {
                println!("HIGHER");
                max = (max - min) / 2;
            } else {
                println!("LOWER");
                min = (max - min) / 2;
            }
            if (min + max) / 2 == marker {
                return -1;
            }
            println!("bounds : {}, {}", min, max);
        }
    }
}

#[cfg(test)]
mod test;
