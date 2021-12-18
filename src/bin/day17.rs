use std::io::{self};


fn main() -> io::Result<()> {
    let y_min = -103;
    let y_max = -58;

    let x_min = 265;
    let x_max = 287;

    let mut sum = 0;
    for x in 0..x_max+1{
        for y in y_min..i32::abs(y_min){
            sum+=test(&mut  x.clone(), &mut y.clone(),(x_min,x_max),(y_min,y_max));
        }
    }
    let max_y_vel = i32::abs(y_min)-1;
    println!("Result1: {}",(max_y_vel*(max_y_vel+1))/2);
    println!("Result2: {}",sum);

    Ok(())
}
fn test(vel_x: &mut i32, vel_y: &mut i32, x_lim: (i32,i32), y_lim: (i32,i32)) -> i32{
    let mut x = 0;
    let mut y = 0;
    loop{
        x += *vel_x;
        y += *vel_y;
        if x >= x_lim.0 && x <= x_lim.1 && y >= y_lim.0 && y <= y_lim.1{
            return 1
        }
        if x > x_lim.1 || y < y_lim.0{
            break
        }
        if *vel_x > 0{
            *vel_x-=1;
        }
        *vel_y-=1;
    }
    0
}
