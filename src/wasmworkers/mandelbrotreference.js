
export default (max_iter, real, imaginary) => {
    let zrzi = (real, imaginary);
    let iter = 0;
    
    while (((zrzi[0]*zrzi[0]) + (zrzi[1]*zrzi[1]) <= 4.0) && (iter < max_iter)) {
        zrzi = (
            ((zrzi[0]*zrzi[0]) - (zrzi[1]*zrzi[1]) + real),
            ((2.0 * zrzi[0] * zrzi[1]) + imaginary)
        );
        iter = iter + 1;
    }

    return iter
}