use util;

#[derive(Debug)]
struct TreeField {
    locations: Vec<(i32,i32)>,
    width: i32,
}

#[derive(Debug)]
enum TreeParseError {
    IoError(std::io::Error),
    InconsistentWidths,
}

impl From<std::io::Error> for TreeParseError {
    fn from(e: std::io::Error) -> Self {
        TreeParseError::IoError(e)
    }
}

impl TreeField {
    fn parse(filename: &str) -> Result<TreeField,TreeParseError> {
        let lines = util::file_lines(filename)?;

        let mut locations : Vec<(i32,i32)> = Vec::new();
        let mut width = 0i32;

        for (y,line) in lines.enumerate() {
            let line = line.unwrap();

            if width == 0 {
                width = line.len() as i32;
            } else if width != (line.len() as i32) {
                return Err(TreeParseError::InconsistentWidths);
            }

            line
                .chars()
                .enumerate()
                .filter(|(_x,c)| *c=='#')
                .for_each(|(x,_c)| locations.push( (x as i32, y as i32) ))
                ;
        }

        Ok(TreeField{locations: locations, width: width})
    }

    fn num_trees_hit(&self, dx: i32, dy: i32) -> usize {
        // Problem statement doesn't include cases where this is
        // needed, but I'd feel weird not checking for it.
        let gcd = util::gcd(dx,dy);
        let dx = dx/gcd;
        let dy = dy/gcd;

        self
            .locations
            .iter()
            .filter(|(_x,y)| y%dy == 0)
            .map(|(x,y)| x - y*dx/dy)
            .map(|xrel| ((xrel % self.width) + self.width) % self.width)
            .filter(|xrel| *xrel == 0)
            .count()
    }
}



fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let tree_field = TreeField::parse(filename).unwrap();
    println!("Part 1, trees hit: {}", tree_field.num_trees_hit(3,1));

    let directions : Vec<(i32,i32)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let prod : usize = directions
        .iter()
        .map(|(dx,dy)| tree_field.num_trees_hit(*dx,*dy))
        .inspect(|c| println!("Total trees: {}", c))
        .product();
    println!("Part 2, prod trees hit: {}", prod);
}
