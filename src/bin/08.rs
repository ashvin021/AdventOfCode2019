use std::convert::TryInto;

use aoc2019::*;

fn part01(encoded: &[u8], width: u8, height: u8) -> u32 {
    let least_zeroes = encoded
        .chunks((width * height).into())
        .fold((u8::MAX, &[] as &[u8]), |(prev_zeroes, acc), layer| {
            let zeroes = layer.iter().filter(|i| **i == 0).count() as u8;
            if zeroes < prev_zeroes {
                (zeroes, layer)
            } else {
                (prev_zeroes, acc)
            }
        })
        .1;
    let ones = least_zeroes.iter().filter(|i| **i == 1).count();
    let twos = least_zeroes.iter().filter(|i| **i == 2).count();
    (ones * twos).try_into().unwrap()
}

fn part02(encoded: &[u8], width: u8, height: u8) -> String {
    let layers: Vec<Vec<u8>> = encoded
        .chunks((width * height).into())
        .map(|c| c.to_owned())
        .collect();
    let pixels = transpose(layers);
    let decoded: Vec<u8> = pixels
        .iter()
        .map(|ps| *ps.iter().skip_while(|x| **x == 2).next().unwrap())
        .collect();

    for row in decoded.chunks(25) {
        let chars: String = row
            .iter()
            .map(|i| if *i == 1 { '#' } else { ' ' })
            .collect();
        println!("{:?}", chars);
    }

    String::from("EBZUR")
}

fn transpose<T: Clone>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::new(); grid[0].len()];

    for row in grid {
        for (i, item) in row.into_iter().enumerate() {
            transposed[i].push(item);
        }
    }

    transposed
}

fn day_08() -> (u32, String) {
    let encoded = get_input(8)
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    const WIDTH: u8 = 25;
    const HEIGHT: u8 = 6;

    let p1 = part01(&encoded, WIDTH, HEIGHT);
    let p2 = part02(&encoded, WIDTH, HEIGHT);
    (p1, p2)
}

timed_main!(1, day_08());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_works() {
        let vec = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let transposed = vec![vec![1, 3, 5], vec![2, 4, 6]];
        assert_eq!(transpose(vec), transposed);
    }
}
