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

    // Parse the string of data into serde_json::Value.
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
fn main() {
    let lines = lines_from_file("./input/input_example.jsonl");
    // for line in lines {
    //     println!("{:?}", line);
    // }
    for ln in lines{
        let apartment_map = get_map(&ln);
        //Valida que se halla creado el mapa correctamente
        if  !apartment_map.is_ok() {
            println!("Couldn't read line");
            continue;
        }
        let apartment_map = apartment_map.ok().unwrap();
        //Obtiene la cantidad de apartamentos y de edificios a buscar y crea una matriz con el tamaño adecuado
        let height = apartment_map["input1"].as_array().unwrap().len();
        let width = apartment_map["input2"].as_array().unwrap().len();
        let matrix = create_matrix(width, height, apartment_map);
        let mut i = 0;
        let mut j = 0;
        while i < matrix.len(){
            if matrix[i]{
                j+=1;
            }
            i+=3;
        }
        println!("{}", j);
        // println!("{} {}", apartment_map["input1"][2][apartment_map["input2"][0].to_string()], apartment_map["input2"][0]);
    }
}

