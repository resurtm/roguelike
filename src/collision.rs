use cgmath::Point2;
use std::cmp::Ordering;

pub(crate) struct Aabb<T> {
    a: Point2<T>,
    b: Point2<T>,
}

impl<T: PartialOrd> Aabb<T> {
    pub(crate) fn new(a: Point2<T>, b: Point2<T>) -> Aabb<T> {
        Aabb { a, b }
    }

    pub(crate) fn intersects(&self, other: &Aabb<T>) -> bool {
        let (s_min_x, s_max_x) = if self.a.x.partial_cmp(&self.b.x).unwrap() == Ordering::Less {
            (&self.a.x, &self.b.x)
        } else {
            (&self.b.x, &self.a.x)
        };
        let (o_min_x, o_max_x) = if other.a.x.partial_cmp(&other.b.x).unwrap() == Ordering::Less {
            (&other.a.x, &other.b.x)
        } else {
            (&other.b.x, &other.a.x)
        };

        let (s_min_y, s_max_y) = if self.a.y.partial_cmp(&self.b.y).unwrap() == Ordering::Less {
            (&self.a.y, &self.b.y)
        } else {
            (&self.b.y, &self.a.y)
        };
        let (o_min_y, o_max_y) = if other.a.y.partial_cmp(&other.b.y).unwrap() == Ordering::Less {
            (&other.a.y, &other.b.y)
        } else {
            (&other.b.y, &other.a.y)
        };

        let intersect_x0 = s_min_x < o_min_x && o_min_x < s_max_x;
        let intersect_x1 = o_min_x < s_min_x && s_min_x < o_max_x;

        let intersect_y0 = s_min_y < o_min_y && o_min_y < s_max_y;
        let intersect_y1 = o_min_y < s_min_y && s_min_y < o_max_y;

        (intersect_x0 || intersect_x1) && (intersect_y0 || intersect_y1)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Point2;

    use super::Aabb;

    #[test]
    fn test_case1() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(12.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 7.5), Point2::new(17.5, 17.5));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn test_case2() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(22.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 7.5), Point2::new(17.5, 17.5));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn test_case3() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(12.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 7.5), Point2::new(10.5, 17.5));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn test_case4() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(12.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 13.5), Point2::new(17.5, 17.5));
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }

    #[test]
    fn test_case5() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(22.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 13.5), Point2::new(17.5, 17.5));
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }

    #[test]
    fn test_case6() {
        let a = Aabb::new(Point2::new(2.5, 2.5), Point2::new(12.5, 12.5));
        let b = Aabb::new(Point2::new(7.5, 13.5), Point2::new(10.5, 17.5));
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }
}
