use crate::fractal::mandelbrot;

mod fractal;
mod canvas;

fn main() {
    canvas::create(1600, 900, mandelbrot)

    /*
    for im in -100..100 {
        for re in -100..100 {
            if mandelbrot(re as f64/50.0, im as f64/30.0, iterations) == None {
                print!("#")
            } else {
                print!(" ")
            }
        }
        print!("\n")
    }
    println!("{:?}", mandelbrot(1f64, 0f64, 50));
     */
}
