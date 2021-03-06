use na::Transform;
use na;
use point::{LocalPointQuery, PointQuery};
use entities::shape::Ball;
use math::{Scalar, Point, Vect};

#[old_impl_check]
impl<N, P, V> LocalPointQuery<N, P> for Ball<N>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> {
    #[inline]
    fn project_point(&self, pt: &P, solid: bool) -> P {
        let sqdist = na::sqnorm(pt.as_vec());

        if sqdist <= self.radius() * self.radius() && solid {
            pt.clone()
        }
        else {
            na::orig::<P>() + *pt.as_vec() / sqdist.sqrt()
        }
    }

    #[inline]
    fn distance_to_point(&self, pt: &P) -> N {
        (na::norm(pt.as_vec()) - self.radius()).max(na::zero())
    }

    #[inline]
    fn contains_point(&self, pt: &P) -> bool {
        na::sqnorm(pt.as_vec()) <= self.radius() * self.radius()
    }
}

#[old_impl_check]
impl<N, P, V, M> PointQuery<N, P, M> for Ball<N>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> {
}
