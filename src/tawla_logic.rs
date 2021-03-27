    use rand::distributions::{Distribution, Uniform};
    use rand::rngs::ThreadRng;
    use rand::thread_rng;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::ops::{Deref, DerefMut};
    use std::thread::Thread;
    use std::fmt;
    use crate::tawla_logic::Color::{Black, White};

     pub(crate) struct Point {
        position: u8,
        checkers: u8,
        occupant: Option<Color>,
    }

    impl Point {
        pub fn new(position: u8) -> Point {
            Point {
                position ,
                checkers : if position == 1 || position == 24 {15} else{0},
                occupant : if position == 1 {Some(Black)} else if position == 24 {Some(White)} else {None}
            }
        }
    }


    pub struct Dice {
        range: Uniform<i32>,
        generator: RefCell<ThreadRng>,
    }

    impl Dice {
        pub fn new() -> Dice {
            Dice {
                range: Uniform::from(1..=6),
                generator: RefCell::new(thread_rng()),
            }
        }
        pub fn throw_once(&self) -> (u8, u8) {
            (
                self.range.sample(self.generator.borrow_mut().deref_mut()) as u8,
                0
            )
        }
        pub fn throw(&self) -> (u8, u8) {
            (
                self.range.sample(self.generator.borrow_mut().deref_mut()) as u8,
                self.range.sample(self.generator.borrow_mut().deref_mut()) as u8
            )
        }
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum Color {
        White, //White moves from Point 1 increasing
        Black, //Black moves from Point 24 decreasing
    }

    pub struct Grid {
        points: Vec<Point>,
    }

    impl Grid {
        pub fn new() -> Grid {
            let mut points = Vec::new();
            points.push(Point::new(1, 15, Some(Color::White)));
            for i in 2..24 {
                points.push(Point::new(i, 0, None));
            }
            points.push(Point::new(24, 15, Some(Color::Black)));
            Grid { points }
        }
        pub fn possible_moves(&self, pos: u8, roll: (u8, u8)) -> Vec<u8> {
            let (move_one, move_two, move_sum) = (roll.0, roll.1, roll.0 + roll.1);
            let current: &Point = self.points.get(pos as usize).unwrap();
            let (before, after) = self.points.split_at(pos as usize);
            match self
                .points
                .get(pos as usize)
                .unwrap()
                .occupant
                .as_ref()
                .unwrap()
            {
                Color::White => after
                    .iter()
                    .filter(|other| {
                        ((other.occupant == current.occupant || other.occupant == None)
                            && other.position == current.position + move_one)
                            || (other.position == current.position + move_two)
                            || (other.position == current.position + move_sum)
                    })
                    .map(|point| point.position)
                    .collect::<Vec<u8>>(),
                Color::Black => before
                    .iter()
                    .filter(|other| {
                        ((other.occupant == current.occupant || other.occupant == None)
                            && other.position == current.position - move_one)
                            || (other.position == current.position - move_two)
                            || (other.position == current.position - move_sum)
                    })
                    .map(|point| point.position)
                    .collect::<Vec<u8>>(),
            }
        }
        pub fn move_from_to(&mut self, src: usize, dist: usize) {
            let mut grid_iter = self.points.iter_mut();
            let src_point = grid_iter.nth(src).unwrap();
            let dist_point = grid_iter.nth(dist).unwrap();
            src_point.checkers -= 1;
            dist_point.checkers += 1;
            if dist_point.occupant == None {
                dist_point.occupant = src_point.occupant;
            }
        }
    }
