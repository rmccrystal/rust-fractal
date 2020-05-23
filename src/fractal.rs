// Returns the number of iterations it took to diverge or returns None if it doesn't diverge
pub fn mandelbrot(x: f64, y: f64, iterations: i32) -> Option<i32> {
    // Max magnitude from (0,0) until it diverges
    let max_magnitude: f64 = 2.0;

    // The current iteration we're on
    let mut iteration = 0;

    // Real and imaginary parts for the iteration (z)
    let mut re: f64 = 0.0;
    let mut im: f64 = 0.0;

    while ((re*re+im*im).sqrt() < max_magnitude) && (iteration < iterations) {
        // Iterate z^2 + c
        // (a+bi)^2=(a^2-b^2) + (2ab)i
        let re_temp = re*re-im*im+x;
        im = 2.0*re*im+y;
        re = re_temp;

        iteration += 1;
    }

    // If we didn't hit the max iterations
    if iteration < iterations {
        return Some(iteration)
    }
    // Else, we hit the limit
    None
}