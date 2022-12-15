use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    dist_to_closest: i32,
}

struct Point {
    x: i32,
    open: bool,
}

fn day15a(sensors: &Vec<Sensor>, beacons: &HashSet<(i32, i32)>) {
    let y = 2000000;
    let mut res = 0;

    for x in -10000000..10000000 {
        if beacons.contains(&(x, y)) {
            continue;
        }

        for sensor in sensors {
            let dist = (x - sensor.x).abs() + (y - sensor.y).abs();
            if dist <= sensor.dist_to_closest {
                res += 1;
                break;
            }
        }
    }

    println!("{}", res);
}

fn day15b(sensors: &Vec<Sensor>) {
    let limit = 4000000;

    for y in 0..=limit {
        let mut points = Vec::new();

        for sensor in sensors {
            if (y - sensor.y).abs() <= sensor.dist_to_closest {
                let dx = sensor.dist_to_closest - (y - sensor.y).abs();
                let start = sensor.x - dx;
                let end = sensor.x + dx + 1;

                points.push(Point {
                    x: start,
                    open: true,
                });
                points.push(Point {
                    x: end,
                    open: false,
                });
            }
        }

        points.sort_by_key(|k| k.x);

        let mut i = 0;
        let mut open = 0;

        while i < points.len() {
            let x = points[i].x;

            while i < points.len() && points[i].x == x {
                if points[i].open {
                    open += 1;
                } else {
                    open -= 1;
                }
                i += 1;
            }

            if open == 0 && x >= 0 && x <= limit {
                let freq = x as i64 * 4000000 + y as i64;
                println!("{}", freq);
                dbg!(x, y);
                return;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let coords = line
            .replace(['=', ',', ':'], " ")
            .split(' ')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<_>>();

        sensors.push(Sensor {
            x: coords[0],
            y: coords[1],
            dist_to_closest: (coords[0] - coords[2]).abs() + (coords[1] - coords[3]).abs(),
        });

        beacons.insert((coords[2], coords[3]));
    }

    day15a(&sensors, &beacons);
    day15b(&sensors);
}
