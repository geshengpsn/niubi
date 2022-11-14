use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

pub trait ControlPoint:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + Copy
    + Clone
{
    /// origin
    fn zeros() -> Self;
}

impl<T> ControlPoint for T
where
    T: Default
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f64, Output = Self>
        + Div<f64, Output = Self>
        + AddAssign<Self>
        + SubAssign<Self>
        + Copy
        + Clone,
{
    fn zeros() -> Self {
        T::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HomoControlPoint<CP: ControlPoint> {
    control_point: CP,
    weight: f64,
}

impl<CP: ControlPoint> Default for HomoControlPoint<CP> {
    fn default() -> Self {
        Self {
            control_point: CP::zeros(),
            weight: 0.0,
        }
    }
}

impl<CP: ControlPoint> Add for HomoControlPoint<CP> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            control_point: self.control_point + rhs.control_point,
            weight: self.weight + rhs.weight,
        }
    }
}
impl<CP: ControlPoint> Sub for HomoControlPoint<CP> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            control_point: self.control_point - rhs.control_point,
            weight: self.weight - rhs.weight,
        }
    }
}
impl<CP: ControlPoint> Mul<f64> for HomoControlPoint<CP> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            control_point: self.control_point * rhs,
            weight: self.weight * rhs,
        }
    }
}
impl<CP: ControlPoint> Div<f64> for HomoControlPoint<CP> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            control_point: self.control_point / rhs,
            weight: self.weight / rhs,
        }
    }
}
impl<CP: ControlPoint> AddAssign for HomoControlPoint<CP> {
    fn add_assign(&mut self, rhs: Self) {
        self.control_point += rhs.control_point;
        self.weight += rhs.weight;
    }
}
impl<CP: ControlPoint> SubAssign for HomoControlPoint<CP> {
    fn sub_assign(&mut self, rhs: Self) {
        self.control_point -= rhs.control_point;
        self.weight -= rhs.weight;
    }
}

impl<CP: ControlPoint> HomoControlPoint<CP> {
    pub fn split(&self) -> (CP, f64) {
        (self.control_point, self.weight)
    }
    pub fn to_control_point_and_weight(&self) -> (CP, f64) {
        let (fat_cp, w) = self.split();
        (fat_cp / w, w)
    }
    pub fn from_control_point(control_point: CP, weight: f64) -> Self {
        Self {
            control_point: control_point * weight,
            weight,
        }
    }
}
