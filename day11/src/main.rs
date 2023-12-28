use utilities::get_input_lines;

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    x: i64,
    y: i64
}

fn is_pair(pair1: (&Galaxy, &Galaxy), pair2: (&Galaxy, &Galaxy)) -> bool {
    (pair1.0 == pair2.1 && pair1.1 == pair2.0) || (pair1 == pair2)
}

fn manhattan_distance(g1: &Galaxy, g2: &Galaxy) -> i64 {
    (g1.x - g2.x).abs() + (g1.y - g2.y).abs()
}

fn main() {
    let (line, part) = get_input_lines();

    let mut galaxies: Vec<Galaxy> = Vec::new();

    let mut map: Vec<Vec<bool>> = Vec::new(); 
    let mut row_offsets: Vec<usize> = Vec::new();
    line.iter().enumerate().for_each(|(i, line)| {
        let num_galaxies = line.chars().filter(|c| *c == '#').count();
        if num_galaxies == 0 { row_offsets.push(i) };
        map.push(line.chars().enumerate().map(|(j, c)| {
            if c == '#' {
                galaxies.push(Galaxy { x: j as i64, y: i as i64 }); 
                return true
            } else {
                return false
            }
        }).collect());
    });

    let mut column_offsets: Vec<usize> = Vec::new();
    'outer: for column_i in 0..map[0].len() { 
        for row in &map {
            if row[column_i] {
                continue 'outer;
            }
        }
        column_offsets.push(column_i);
    }

    let get_offset_galaxy = |g: &Galaxy| -> Galaxy {
        let row_offset_count = row_offsets.iter().filter(|row_i| g.y as usize > **row_i).count();
        let column_offset_count = column_offsets.iter().filter(|column_i| g.x as usize > **column_i).count();

        let new_y = if part == 1 { 
            g.y as usize + row_offset_count 
        } else {
            g.y as usize + (row_offset_count * 1000000 - row_offset_count)
        };

        let new_x = if part == 1 {
            g.x as usize + column_offset_count
        } else {
            g.x as usize + (column_offset_count * 1000000 - column_offset_count)
        };

        Galaxy { x: new_x as i64, y: new_y as i64 }
    };

    let mut pairs: Vec<(&Galaxy, &Galaxy)> = Vec::new();
    let mut path_sum: i64 = 0;
    for g1 in &galaxies {
        for g2 in  &galaxies {
            if g1 == g2 { continue };

            let matches = pairs.iter().filter(|existing_pair| is_pair(**existing_pair, (g1, g2))).count();
            if matches > 0 { continue };

            // Create galaxy with offset
            let off_g1 = get_offset_galaxy(g1);
            let off_g2 = get_offset_galaxy(g2);

            let sum = manhattan_distance(&off_g1, &off_g2);
            path_sum += sum;

            pairs.push((g1, g2))
        }
    }
    println!("Sum: {}", path_sum);
}
