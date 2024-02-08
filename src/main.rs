
fn generate_matrix(x: i32, y: i32) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    let mut i = 0;
    let x_axis = y / 2;
    let y_axis = x / 2;
    while i < y {
        let mut line = Vec::new();
        let mut j = 0;
        while j < x {
            line.push(x_axis == i || y_axis == j);
            j += 1;
        }
        result.push(line);
        i += 1;
    }

    return result;
}

fn generate_graph<F>(f: F, x: i32, y: i32) -> Vec<Vec<bool>> where F: Fn(i32) -> i32{
    let mut matrix = generate_matrix(x, y);
    let mut i = 0;
    let x_middle = x / 2;
    let y_middle = y / 2;
    while i < x {
        let value = (y_middle - f(i - x_middle)) as usize;
        if value < y as usize {
            matrix[value][i as usize] = true;
        }
        i += 1;
    }

    return matrix;
}


fn print_matrix(mat: &Vec<Vec<bool>>) {
    for line in mat {
        for i in line {
            print!("{}", if i.clone() { "x" } else { " " });
        }
        println!();
    }
}

fn main() {
    print_matrix(&generate_graph(|x| x, 32, 32));
}
