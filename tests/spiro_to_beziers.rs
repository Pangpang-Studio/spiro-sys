use spiro_sys::{self, bezctx};
use std::os::raw::{c_char, c_int};

macro_rules! spiro_cp {
    ({$x:literal, $y:literal, $ty:literal}) => {
        spiro_sys::spiro_cp {
            x: $x as f64,
            y: $y as f64,
            ty: $ty as c_char,
        }
    };
}

unsafe extern "C" fn println_moveto(_bc: *mut bezctx, x: f64, y: f64, _is_open: c_int) {
    println!("M {}, {} ", x, y);
}
unsafe extern "C" fn println_lineto(_bc: *mut bezctx, x: f64, y: f64) {
    println!("L {}, {} ", x, y);
}
unsafe extern "C" fn println_quadto(_bc: *mut bezctx, x1: f64, y1: f64, x2: f64, y2: f64) {
    println!("Q {}, {}, {}, {} ", x1, y1, x2, y2);
}
unsafe extern "C" fn println_curveto(
    _bc: *mut bezctx,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) {
    println!("C {}, {}, {}, {}, {}, {} ", x1, y1, x2, y2, x3, y3);
}

#[test]
fn spiro_to_beziers() {
    // Path from
    // https://github.com/fontforge/libspiro/blob/84ce4dfd24f0e3ee83589bbfb02723dff5c03414/tests/call-test.c#L278
    let mut path5 = vec![
        spiro_cp!({  0,   0, '{'}),
        spiro_cp!({100, 100, 'c'}),
        spiro_cp!({200, 200, '['}),
        spiro_cp!({300, 200, ']'}),
        spiro_cp!({400, 150, 'c'}),
        spiro_cp!({300, 100, '['}),
        spiro_cp!({200, 100, ']'}),
        spiro_cp!({150,  50, 'c'}),
        spiro_cp!({100,   0, '['}),
        spiro_cp!({  0,-100, ']'}),
        spiro_cp!({-50,-200, 'c'}),
        spiro_cp!({-80,-250, '}'}),
    ];

    let mut ctx = bezctx {
        moveto: Some(println_moveto),
        lineto: Some(println_lineto),
        quadto: Some(println_quadto),
        curveto: Some(println_curveto),
        mark_knot: None,
    };

    unsafe {
        spiro_sys::TaggedSpiroCPsToBezier(path5.as_mut_ptr(), &mut ctx);
    }
}
