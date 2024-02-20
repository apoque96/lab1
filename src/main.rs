use serde_json::{Result, Value};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// Lee el archivo linea por linea
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
//Convierte la linea en el mapa de apartamentos
fn get_map(ln: &String) -> Result<Value> {
    let v: Value = serde_json::from_str(ln)?;

    Ok(v)
}
//Crea la matriz del mapa
fn create_matrix(width: usize, height: usize, apartment_map: Value) -> Vec<bool>{
    let mut matrix = vec![false; height*width];
    let mut apartment = 0;
    let mut building = 0;
    while building < width{
        let building_string = apartment_map["input2"][building].to_string();
        let building_string = (building_string[1..building_string.len()-1]).to_string();
        while apartment < height{
            if apartment_map["input1"][apartment][&building_string] == true{
                matrix[apartment * width + building] = true;
            }
            apartment += 1;
        }
        apartment = 0;
        building += 1;
    }
    matrix
}

struct Apartment{
    apartment_number: usize,
    distance_to_buildings: Vec<usize>,
}
//Encuentra el departamento con las mejores caracteristicas
fn find_best_apartment(width: usize, height: usize, matrix: Vec<bool>) -> Vec<Apartment>{
    let mut possible_best = vec![];
    let mut min_distance = 0;
    let mut apartment = 0;
    while apartment < height{
        let mut current = Apartment{
            apartment_number: apartment,
            distance_to_buildings: vec![],
        };
        let mut building = 0;
        while building < width{
            //Busca en los apartamentos adyacentes, dejando constancia de la distancia,
            //hasta que encuentre el edificio deseado
            let mut distance = 0;
            let result: bool = loop{
                if (apartment as i32 - distance as i32) >= 0{
                    if matrix[(apartment - distance) * width + building] == true {
                        break true;
                    }
                }
                if (apartment + distance) < height{
                    if matrix[(apartment + distance) * width + building] == true{
                        break true;
                    }
                }
                if (apartment as i32 - distance as i32) < 0 &&
                   (apartment + distance) >= height{
                    break false;
                }
                distance += 1;
            };
            if result == true{
                current.distance_to_buildings.push(distance);
            }
            else {break;}
            building += 1;
        }
        //Determina si el apartamento está dentro de los apartamentos en los que se tiene
        //la menor distancia
        if current.distance_to_buildings.len() == width{
            let dist: usize = current.distance_to_buildings.iter().sum();
            if possible_best.len() == 0{
                possible_best.push(current);
                min_distance = dist;
            }
            else if min_distance == dist{
                possible_best.push(current);
            }
            else if min_distance > dist{
                possible_best.clear();
                possible_best.push(current);
                min_distance = dist;
            }
        }
        apartment += 1;
    }
    if possible_best.len() == 0{
        //No hay apartamento que satiface las condiciones
        return possible_best;
    }
    //Busca los apartamentos con el que se tiene que caminar menos
    let mut lowest = -1;
    apartment = width-1;
    for app in &mut possible_best{
        app.distance_to_buildings.sort();
        if lowest == -1{
            lowest = app.distance_to_buildings[apartment] as i32;
        }
        if lowest > app.distance_to_buildings[apartment] as i32{
            lowest = app.distance_to_buildings[apartment] as i32;
        }
    }
    let mut i = 0;
    while i < possible_best.len(){
        if possible_best[i].distance_to_buildings[apartment] > lowest as usize{
            possible_best.remove(i);
        }
        else{
            i += 1;
        }
    }

    lowest = -1;
    for app in &mut possible_best{
        if lowest == -1{
            lowest = app.distance_to_buildings[0] as i32;
        }
        if lowest > app.distance_to_buildings[0] as i32{
            lowest = app.distance_to_buildings[0] as i32;
        }
    }

    i = 0;
    while i < possible_best.len(){
        if possible_best[i].distance_to_buildings[0] > lowest as usize{
            possible_best.remove(i);
        }
        else{
            i += 1;
        }
    }
    //Please update
    possible_best
}
fn lab1(path: &str){
    //Lee el json
    let lines = lines_from_file(path);
    for ln in lines{
        let apartment_map = get_map(&ln);
        //Valida que se halla creado el mapa correctamente
        if  !apartment_map.is_ok() {
            println!("Couldn't read line");
            break;
        }
        let apartment_map = apartment_map.ok().unwrap();
        if apartment_map["input1"].is_null() || apartment_map["input2"].is_null(){
            println!("Incorrect file format");
            return;
        }
        //Obtiene la cantidad de apartamentos y de edificios a buscar y crea una matriz con el tamaño adecuado
        let height = apartment_map["input1"].as_array().unwrap().len();
        let width = apartment_map["input2"].as_array().unwrap().len();
        let matrix = create_matrix(width, height, apartment_map);
        let best_apartment = find_best_apartment(width, height, matrix);
        print!("[");
        {
        let mut i = 0;
        while i < best_apartment.len(){
            print!("{}", best_apartment[i].apartment_number);
            if i < best_apartment.len()-1{
                print!(", ");
            }
            i += 1;
        }
        }
        println!("]");
    }
}
fn main() {
    // lab1("./input/input_example.jsonl", true);
    lab1("./input/input_challenge.jsonl");
}

