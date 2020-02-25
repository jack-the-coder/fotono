use image;
use cgmath::Vector3;
use cgmath;
use std::f32;

// Note on coordinate system: positive x is right, y is up, z is away
// `image` crate has its own separate 2D coordinate system
// TODO: Start real documentation

const IMG_X: u32 = 4000; // dimensions of image
const IMG_Y: u32 = 3000;

#[derive(Debug)]
struct Ray {
	origin: Vector3<f32>,
	direction: Vector3<f32>,
}

impl Ray {
	fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
		Ray { origin, direction }
	}
	fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
		self.origin + t * self.direction
	}

	// Returns the unit vector of the direction of a ray
	fn unit_direction(&self) -> Vector3<f32> {
		let sum = self.direction.x.powi(2) + self.direction.y.powi(2) + self.direction.z.powi(2);
		let length = sum.sqrt();
		self.direction / length
	}
}

// Returns whether a sphere with a given center and radius is hit by a given ray
fn hit_sphere(center: &Vector3<f32>, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = cgmath::dot(ray.direction, ray.direction);
    // let b = IMG_X as f32/-100.0 * cgmath::dot(oc, ray.direction);
    let b = 2.0 * cgmath::dot(oc, ray.direction);
    let c = cgmath::dot(oc, oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vector3<f32> {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vector3::new(0.8, 0.2, 0.2);
    }
	let u_direction = r.unit_direction();
	let t: f32 = 0.5 * (u_direction.y + 1.0);

	// blendedValue=(1−t)∗startValue+t∗endValue,
    // aka a linear interpolation
	(1.0-t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
	let lower_left_corner = Vector3::new(IMG_X as f32/-100.0, IMG_Y as f32/-100.0, IMG_Y as f32/-100.0);
	let horizontal = Vector3::new(IMG_X as f32/50.0, 0.0, 0.0);
	let vertical = Vector3::new(0.0, IMG_Y as f32/50.0, 0.0);
	let origin = Vector3::new(0.0, 0.0, 0.0);

    let mut imgbuf = image::ImageBuffer::new(IMG_X, IMG_Y);

    for x in 0..IMG_X {
        for y in 0..IMG_Y {
        	let u = y as f32 / IMG_X as f32;
        	let v = x as f32 / IMG_Y as f32;
        	let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

        	let col = color(&ray);

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;

            *pixel = image::Rgb([(255.99 * col.x) as u8, (255.99 * col.y) as u8, (255.99 * col.z) as u8]); // render blue sky
        }
    }

    imgbuf.save("output/image.png").unwrap();
}
