use riesz::{Vector, Vector3};

fn main() {
    let v = Vector::try_from(vec![1.0, 2.0, 3.0]).unwrap();
    let v2 = Vector3::constant(3.0);
    let v3 = Vector::new([1.2, 3.4]);

    let _v4 = v + v2;

    println!("{}, {:.3}", v, v.mag());
    println!("{}, {:.3}", v2, v2.mag());
    println!("{}, {:.3}", v3, v3.mag());
    println!("Dot is {:.3}", Vector::dot(v, v2));

    println!("{}", Vector::<4>::new([1.00, 3.0, 4.0, 5.0]));
}
