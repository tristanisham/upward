use std::vec;

pub struct Args {
    pub x: u32,
    pub y: u32,
    pub description: Vec<String>,
}

impl Args {
    pub fn parse(args: &Vec<String>) -> Self {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut description: Vec<String> = vec![];

        for (i, arg) in args.into_iter().enumerate() {
            match arg.as_str() {
                "-x" | "--x" => {
                    if args.len() > i + 1 {
                        x = args[i + 1].parse().unwrap();
                    }
                }
                "-y" | "--y" => {
                    if args.len() > i + 1 {
                        y = args[i + 1].parse().unwrap();
                    }
                }
                "-s" | "--search" => {
                    if args.len() > i + 1 {
                        description = args[i + 1..].to_vec()
                    }
                }
                _ => {}
            }
        }

        Self { x, y, description }
    }
}
