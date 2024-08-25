use generador_horario::funciones;
fn main() {
    println!("Esto es antes de la función");
    match funciones::read_json("/home/gd_15/Escritorio/generador_horario/materias.json") {
        Ok(_) => {
            println!("Archivo leído correctamente")
        }
        Err(e) => println!("Error {e}"),
    };
    println!("Esto es depués de la función");
}
