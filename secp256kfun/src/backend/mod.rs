mod parity;
pub use parity::*;

pub trait BackendScalar: Sized {
    fn minus_one() -> Self;
    fn from_u32(int: u32) -> Self;
    fn zero() -> Self;
    fn from_bytes_mod_order(bytes: [u8; 32]) -> Self;
    fn from_bytes(bytes: [u8; 32]) -> Option<Self>;
    fn to_bytes(&self) -> [u8; 32];
}

pub trait BackendXOnly: Sized {
    fn from_bytes(bytes: [u8; 32]) -> Option<Self>;
    fn as_bytes(&self) -> &[u8; 32];
    fn into_bytes(self) -> [u8; 32];
    fn into_norm_point_even_y(self) -> Point;
}

pub trait BackendPoint {
    fn zero() -> Point;
    fn is_zero(&self) -> bool;
    fn norm_to_coordinates(&self) -> ([u8; 32], [u8; 32]);
    fn norm_to_xonly(&self) -> XOnly;
    fn norm_from_bytes_y_oddness(x_bytes: [u8; 32], y_odd: bool) -> Option<Point>;
    fn norm_from_coordinates(x: [u8; 32], y: [u8; 32]) -> Option<Point>;
}

pub trait TimeSensitive {
    fn scalar_mul_norm_point(lhs: &Scalar, rhs: &Point) -> Point;
    fn scalar_mul_point(lhs: &Scalar, rhs: &Point) -> Point;
    fn scalar_eq(lhs: &Scalar, rhs: &Scalar) -> bool;
    fn point_eq_point(lhs: &Point, rhs: &Point) -> bool;
    fn point_normalize(point: &mut Point);
    fn point_eq_norm_point(lhs: &Point, rhs: &Point) -> bool;
    fn point_eq_xonly(lhs: &Point, rhs: &XOnly) -> bool;
    fn point_add_point(lhs: &Point, rhs: &Point) -> Point;
    fn point_add_norm_point(lhs: &Point, rhs: &Point) -> Point;
    fn point_sub_point(lhs: &Point, rhs: &Point) -> Point {
        let mut rhs = rhs.clone();
        Self::point_neg(&mut rhs);
        Self::point_add_point(lhs, &rhs)
    }
    fn point_neg(point: &mut Point);
    fn point_sub_norm_point(lhs: &Point, rhs: &Point) -> Point;
    fn point_conditional_negate(point: &mut Point, cond: bool);
    fn norm_point_sub_point(lhs: &Point, rhs: &Point) -> Point;
    fn norm_point_neg(point: &mut Point);
    fn norm_point_eq_xonly(point: &Point, xonly: &XOnly) -> bool;
    fn norm_point_eq_norm_point(lhs: &Point, rhs: &Point) -> bool;
    fn norm_point_is_y_even(point: &Point) -> bool;
    fn norm_point_conditional_negate(point: &mut Point, cond: bool);
    fn basepoint_double_mul(x: &Scalar, A: &BasePoint, y: &Scalar, B: &Point) -> Point;
    fn scalar_add(lhs: &Scalar, rhs: &Scalar) -> Scalar;
    fn scalar_sub(lhs: &Scalar, rhs: &Scalar) -> Scalar;
    fn scalar_cond_negate(scalar: &mut Scalar, neg: bool);
    fn scalar_is_high(scalar: &Scalar) -> bool;
    fn scalar_is_zero(scalar: &Scalar) -> bool;
    fn scalar_mul(lhs: &Scalar, rhs: &Scalar) -> Scalar;
    fn scalar_invert(scalar: &Scalar) -> Scalar;
    fn scalar_mul_basepoint(scalar: &Scalar, base: &BasePoint) -> Point;
    fn xonly_eq(lhs: &XOnly, rhs: &XOnly) -> bool;
}
