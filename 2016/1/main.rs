enum Move {
    Left(i32),
    Right(i32),
}
use Move::{Left, Right};

// returns a new (here, facing)
fn go(command: &Move, mut here: (i32, i32), mut facing: (i32, i32), beenhere: &mut std::collections::HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    // first we rotate facing left or right
    let distance;
    match command {
        &Left(d) => {
            facing = match facing {
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (_, _) => (0, 0),
            };
            distance = d;
        }
        &Right(d) => {
            facing = match facing {
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (_, _) => (0, 0),
            };
            distance = d;
        }
    }
    for _ in 0..distance {
      // Mark each step walked
      here.0 += facing.0;
      here.1 += facing.1;
      if beenhere.contains(&here) {
        println!("Already been to {:?} (Sum {})", here, here.0+here.1);
      }
      beenhere.insert(here);
    }
    (here, facing)
}

fn main() {
    let mut beenhere = std::collections::HashSet::new();

    let input =
        [Left(1), Left(5), Right(1), Right(3), Left(4), Left(5), Right(5), Right(1), Left(2),
         Left(2), Left(3), Right(4), Left(2), Right(3), Right(1), Left(2), Right(5), Right(3),
         Left(4), Right(4), Left(3), Right(3), Right(3), Left(2), Right(1), Left(3), Right(2),
         Left(1), Right(4), Left(2), Right(4), Left(4), Right(5), Left(3), Right(1), Right(1),
         Left(1), Left(3), Left(2), Right(1), Right(3), Right(2), Left(1), Right(4), Left(4),
         Right(2), Left(189), Left(4), Right(5), Right(3), Left(1), Right(47), Right(4), Right(1),
         Right(3), Left(3), Left(3), Left(2), Right(70), Left(1), Right(4), Right(185), Right(5),
         Left(4), Left(5), Right(4), Left(1), Left(4), Right(5), Left(3), Right(2), Right(3),
         Left(5), Left(3), Right(5), Left(1), Right(5), Left(4), Right(1), Right(2), Left(2),
         Left(5), Left(2), Right(4), Left(3), Right(5), Right(1), Left(5), Left(4), Left(3),
         Right(4), Left(3), Left(4), Left(1), Left(5), Left(5), Right(5), Left(5), Left(2),
         Left(1), Left(2), Left(4), Left(1), Left(2), Right(3), Right(1), Right(1), Left(2),
         Left(5), Right(2), Left(3), Left(5), Left(4), Left(2), Left(1), Left(2), Right(3),
         Left(1), Left(4), Right(3), Right(3), Left(2), Right(5), Left(1), Left(3), Left(3),
         Left(3), Left(5), Right(5), Right(1), Right(2), Left(3), Left(2), Right(4), Right(1),
         Right(1), Right(3), Right(4), Right(3), Left(3), Right(3), Left(5), Right(2), Left(2),
         Right(4), Right(5), Left(4), Left(3), Left(1), Left(5), Left(1), Right(1), Right(2),
         Left(1), Right(3), Right(4), Right(5), Right(2), Right(3), Left(2), Left(1), Left(5)];


    // location, direction facing
    let mut position = ((0, 0), (1, 0));

    for command in &input[..] {
        println!("at: {:?}", position.0);
        position = go(command, position.0, position.1, &mut beenhere);
    }

    println!("Ended at here: {}, {}", (position.0).0, (position.0).1);
    println!("Sum is {}", (position.0).0 + (position.0).1);
}
