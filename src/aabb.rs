use cgmath::{AbsDiffEq, InnerSpace, Point2, RelativeEq, Vector2};

pub(crate) struct Aabb {
    min: Point2<f64>,
    max: Point2<f64>,
}

impl Aabb {
    pub(crate) fn new(min: Point2<f64>, max: Point2<f64>) -> Aabb {
        Aabb { min, max }
    }

    pub(crate) fn check_contact(&self, other: &Self) -> AabbContact {
        // mtv == minimum translation vector
        // current minimum distance -- set max value so next value is always less
        let mut mtv_dist = f64::MAX;
        // axis along which to travel with the minimum distance
        let mut mtv_axis = Vector2::new(0.0, 0.0);

        // axes of potential separation
        // each shape must be projected on these axes to test for intersection:
        // (1, 0, 0) - x axis
        // (0, 1, 0) - y axis
        // (0, 0, 1) - z axis (not applicable here)

        // x axis
        if !Self::check_sat_axis(
            Vector2::unit_x(),
            self.min.x,
            self.max.x,
            other.min.x,
            other.max.x,
            &mut mtv_axis,
            &mut mtv_dist,
        ) {
            return AabbContact::empty();
        }

        // y axis
        if !Self::check_sat_axis(
            Vector2::unit_y(),
            self.min.y,
            self.max.y,
            other.min.y,
            other.max.y,
            &mut mtv_axis,
            &mut mtv_dist,
        ) {
            return AabbContact::empty();
        }

        // z axis
        // ^^^ (not applicable here)

        // mtv (== minimum translation vector) = normal * penetration
        let normal = mtv_axis.normalize();

        // multiply the penetration depth by itself plus a small increment
        // when the penetration is resolved using the mtv, it will no longer intersect
        let penetration = f64::sqrt(mtv_dist) * 1.001;

        AabbContact::new(penetration, normal)
    }

    fn check_sat_axis(
        axis: Vector2<f64>,
        min_a: f64,
        max_a: f64,
        min_b: f64,
        max_b: f64,
        mtv_axis: &mut Vector2<f64>,
        mtv_dist: &mut f64,
    ) -> bool {
        // Separating Axis Theorem (SAT):
        // - two convex shapes only overlap if they overlap on all axes of separation
        // - in order to create accurate responses we need to find the collision vector (minimum translation vector)
        // - the collision vector is made from a vector and a scalar:
        //   * the vector value is the axis associated with the smallest penetration
        //   * the scalar value is the smallest penetration value
        // - find if the two boxes intersect along a single axis
        // - compute the intersection interval for that axis
        // - keep the smallest intersection/penetration value

        let axis_len_square = axis.dot(axis);
        // if the axis is degenerate then ignore
        if axis_len_square < 1.0e-8 {
            return true;
        }

        // calculate the two possible overlap ranges
        // either we overlap on the left or the right sides
        let d0 = max_b - min_a; // 'left' side
        let d1 = max_a - min_b; // 'right' side

        // intervals do not overlap, so no intersection
        if d0 <= 0.0 || d1 <= 0.0 {
            return false;
        }

        // find out if we overlap on the 'right' or 'left' of the object
        let overlap = if d0 < d1 { d0 } else { -d1 };

        // the mtd vector for that axis
        let sep = axis * (overlap / axis_len_square);
        let sep_len_square = sep.dot(sep);

        // if that vector is smaller than our computed minimum translation distance,
        // then use that vector as our current minimum translation vector & distance
        if sep_len_square < *mtv_dist {
            *mtv_axis = sep;
            *mtv_dist = sep_len_square;
        }

        true
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct AabbContact {
    pub(crate) intersects: bool,
    pub(crate) penetration: f64,
    pub(crate) min_trans: Vector2<f64>,
}

impl AabbContact {
    pub(crate) fn empty() -> AabbContact {
        AabbContact { intersects: false, penetration: 0.0, min_trans: Vector2::new(0.0, 0.0) }
    }

    pub(crate) fn new(penetration: f64, min_trans: Vector2<f64>) -> AabbContact {
        AabbContact { intersects: true, penetration, min_trans }
    }
}

impl AbsDiffEq for AabbContact {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.intersects == other.intersects
            && f64::abs_diff_eq(&self.penetration, &other.penetration, epsilon)
            && self.min_trans.abs_diff_eq(&other.min_trans, epsilon)
    }
}

impl RelativeEq for AabbContact {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.intersects == other.intersects
            && f64::relative_eq(&self.penetration, &other.penetration, epsilon, max_relative)
            && self.min_trans.relative_eq(&other.min_trans, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::{Aabb, AabbContact};
    use cgmath::{assert_relative_eq, Point2, Vector2};

    #[test]
    fn test_no_intersection_case1() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(20.0, 20.0), Point2::new(30.0, 30.0));
        assert_eq!(a.check_contact(&b), AabbContact::empty());
        assert_eq!(b.check_contact(&a), AabbContact::empty());
    }

    #[test]
    fn test_no_intersection_case2() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(11.0, 20.0), Point2::new(19.0, 30.0));
        assert_eq!(a.check_contact(&b), AabbContact::empty());
        assert_eq!(b.check_contact(&a), AabbContact::empty());
    }

    #[test]
    fn test_no_intersection_case3() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(20.0, 11.0), Point2::new(30.0, 19.0));
        assert_eq!(a.check_contact(&b), AabbContact::empty());
        assert_eq!(b.check_contact(&a), AabbContact::empty());
    }

    #[test]
    fn test_intersection_case1() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(15.0, 15.0), Point2::new(30.0, 30.0));
        assert_relative_eq!(a.check_contact(&b), AabbContact::new(5.005, Vector2::new(-1.0, 0.0)));
        assert_relative_eq!(b.check_contact(&a), AabbContact::new(5.005, Vector2::new(1.0, 0.0)));
    }

    #[test]
    fn test_intersection_case2() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(11.0, 18.5), Point2::new(19.0, 30.0));
        assert_relative_eq!(a.check_contact(&b), AabbContact::new(1.5015, Vector2::new(0.0, -1.0)));
        assert_relative_eq!(b.check_contact(&a), AabbContact::new(1.5015, Vector2::new(0.0, 1.0)));
    }

    #[test]
    fn test_intersection_case3() {
        let a = Aabb::new(Point2::new(10.0, 10.0), Point2::new(20.0, 20.0));
        let b = Aabb::new(Point2::new(17.5, 11.0), Point2::new(30.0, 19.0));
        assert_relative_eq!(a.check_contact(&b), AabbContact::new(2.5025, Vector2::new(-1.0, 0.0)));
        assert_relative_eq!(b.check_contact(&a), AabbContact::new(2.5025, Vector2::new(1.0, 0.0)));
    }
}
