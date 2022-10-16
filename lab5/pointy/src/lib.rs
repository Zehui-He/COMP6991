/// Returns the first element of a slice.
/// ```
/// use pointy::first;
/// let list = vec![1, 2, 3];
/// assert_eq!(&1, first(&list));
/// ```
///
/// ```
/// use pointy::first;
/// let list = vec!["a", "b", "c"];
/// assert_eq!(&"a", first(&list));
/// ```
// TODO: implement the below function
// You will need to change its signature
pub fn first<T>(slice: &[T]) -> &T {
    slice.first().unwrap()
}

/// A generic point struct.
///
/// ```
/// use pointy::Point;
/// let point = Point { x: "string", y: "another" };
/// assert_eq!("string", point.x);
/// assert_eq!("another", point.y);
/// ```
///
/// ```
/// use pointy::Point;
/// let point = Point { x: 1.0, y: 2.0 };
/// assert_eq!(1.0, point.x);
/// assert_eq!(2.0, point.y);
/// ```
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    /// Returns the distance between two points.
    /// ```
    /// use pointy::Point;
    /// let point1 = Point { x: 1.0, y: 2.0 };
    /// let point2 = Point { x: 3.0, y: 4.0 };
    /// assert_eq!(2.828427, point1.distance(point2));
    /// ```
    ///
    /// ```
    /// use pointy::Point;
    /// let point1 = Point::new(1.0, 2.0);
    /// assert_eq!(1.0, point1.x);
    /// ```
    pub fn distance(&self, second: Point<f32>) -> f32 {
        let x_diff = self.x - second.x;
        let y_diff = self.y - second.y;
        f32::sqrt(x_diff.powf(2.0) + y_diff.powf(2.0))
    }
}

impl<T> Point<T> {
    pub fn new(val1: T, val2:T) -> Self {
        Point { x: val1, y: val2 }
    }
}
