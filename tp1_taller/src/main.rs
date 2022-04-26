mod lib;

fn main() {
    let aux = lib::run_program();
    if aux.is_err() {
        println!("ERROR: {:?}", aux.unwrap_err());
    }
}
