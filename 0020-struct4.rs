fn main() {

  let point_1 = Points {x: 5.4, y: 12.0, z: 4.7};

  let point_2 = Points {x: -6.14, y: 21.25, z: 14.7};

  let point_3 = Points {x: 5.14, y: 112.0, z: 74.7};

  let avg_result = point_average(point_1, point_2, point_3);

  println!("Average points => x = {}, y = {}, z = {}", avg_result.0, avg_result.1, avg_result.2) // Average points => x = 1.4666666666666668, y = 48.416666666666664, z = 31.366666666666664

  
}

struct Points {
  x: f64,
  y: f64,
  z: f64
}

fn point_average(p1: Points, p2: Points, p3: Points) -> (f64, f64, f64) {

  let avg_x = (p1.x + p2.x + p3.x) / 3.0;

  let avg_y = (p1.y + p2.y + p3.y) / 3.0;

  let avg_z = (p1.z + p2.z + p3.z) / 3.0;

  (avg_x, avg_y, avg_z)

}



