use riesz::Matrix; //, Vector3};

fn main() {
    let v = Matrix::new([[1.0, 2.0, 3.0]]);
    let v2 = Matrix::<3, 2>::constant(3.0);
    let v3 = Matrix::new([[1.2], [3.4]]);

    // let mut _v4 = v + v2;
    // _v4 -= v2;

    // println!("{}, {:.3}", v, v.mag());
    // println!("{}, {:.3}", v2, v2.mag());
    // println!("{}, {:.3}", v3, v3.mag());
    // println!("Dot is {:.3}", v.dot(v2));

    // v.dot(v2);

    println!("{}", Matrix::<1, 4>::new([[1.00, 3.0, 4.0, 5.0]]));
    println!("{}", v);
    println!("{}", v2);
    println!("{}", v3);
}
