fn main() {

  let capital = CityInfo {
    name: Cities::Ankara,
    mayor: String::from("Mansur Yavaş"),
    population: 5_850_000
  };

  println!("Ankara population: {}", capital.population) // Ankara population: 5850000

}

enum Cities {
  Ankara,
  Istanbul,
  Izmir,
  Adana,
  Kahramanmaras,
  Kayseri
}

struct CityInfo {
  name: Cities,
  mayor: String,
  population: u32
}
