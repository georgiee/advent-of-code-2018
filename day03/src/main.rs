use std::fs::read_to_string;

#[derive(Debug)]
struct Rectangle {
    id:String,
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl Rectangle {
    fn left(self: &Self) -> u32 {
        return self.x;
    }
    fn right(self: &Self) -> u32 {
        return self.x + self.width;
    }
    fn top(self: &Self) -> u32 {
        return self.y;
    }
    fn bottom(self: &Self) -> u32 {
        return self.y + self.height;
    }

    fn intersects(self: &Self, other: &Rectangle) -> bool {
        return self.left() < other.right()
            && self.right() > other.left()
            &&  self.top() < other.bottom()
            &&  self.bottom() > other.top()
    }
}

fn main() {
//    part01();
    part02();
}

fn part02() {
//    let file_content = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    let rects:Vec<Rectangle> = file_content.lines().map(|line| {
        let rect = parse_rectangle(line);
        return rect;
    }).collect();


    for (index, rect_a) in rects.iter().enumerate() {
        let mut counter = 0;
        for rect_b in rects.iter() {

            if(rect_a.id.eq(&rect_b.id)) {
                continue;
            }

            if rect_a.intersects(rect_b) {
                counter += 1;
            }
        }

        if counter < 1 {
            println!("this rect has zero intersections: {:?}", rect_a);
        }
    }
}
fn part01() {
//    let file_content = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    let rects:Vec<Rectangle> = file_content.lines().map(|line| {
        let rect = parse_rectangle(line);
        return rect;
    }).collect();

    let mut unit = Rectangle {
        id: String::from("measure"),
        x: 0,
        y: 0,
        width: 1,
        height: 1
    };
    let mut counter_all = 0;

    // brute force, go over each "pixel" and find the rects intersecting it
    // count how many pixels are within at least two rects.
    // that's 1_000_000 pixels checked individually with all rects
    for x in 0..1000 {
        for y in 0..1000 {
            unit.x = x;
            unit.y = y;

            let mut counter = 0;
            for rect in rects.iter() {
                let intersects = rect.intersects(&unit);
                if intersects {
                    counter += 1;
                }
            }
            if counter >= 2 {
                counter_all += 1;
            }
        }
    }
    println!("counter_all {}", counter_all);
    //479 is too low -> that's the number of intersection
    // we need the area count of intersections
}

fn parse_rectangle(line:&str) -> Rectangle {
    //        println!("Hello, world! {}", line);
    let parts:Vec<&str> = line.split('@').collect();
    let parts2:Vec<&str> = parts[1].split(':').collect();

    let id= parts[0].trim();
    let offset:Vec<&str> = parts2[0].trim().split(',').collect();
    let size:Vec<&str> = parts2[1].trim().split('x').collect();

    let rect = Rectangle {
        id: String::from(id),
        x:offset[0].parse().expect("That's not a number"),
        y:offset[1].parse().expect("That's not a number"),
        width:size[0].parse().expect("That's not a number"),
        height: size[1].parse().expect("That's not a number")
    };

    return rect;
}
