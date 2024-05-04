use std::fs::File;

pub fn bad_index(){
    let vector = vec![0,1,2];
    vector[3];
    panic!("Nunca me voy a llegar a ejecutar!");
}

pub fn open_unexisting_file(){
    let f = File::open("No existo");

    let f = match f {
        Ok(file) => file,
        Err(Error) => {
            panic!("Error: El archivo no existe.");
        }
    };
}