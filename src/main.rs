use std::collections::HashMap;
// use path::to::item;
//std - rust standart library
//collections::HashMap - processamento e estura de dados, data storege;
//HashMap - associar chave arbitrarias com um valor arbitrario
        //- quer um cache - memória temporária
        //- quer mapa sem funcionalidade extra
//mapa - dictionary, associative array, hash map: data structure q guarda
//uma coleção de pares chave-valor, cada chave associada unico valor
//maneira eficiente guardar e recuperar dados com identificador exclusivo(key)
fn main() {
    let action = std::env::args().nth(1).expect("Please provide ana action");
    let item = std::env::args().nth(2).expect("Please provide an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

struct Todo {
    //use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        //insert a new item into our map.
        //active state is set to true by default.
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
