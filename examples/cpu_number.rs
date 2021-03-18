/// Get the number of cpus, will be useful for multi-threading.
fn main(){
    let num = num_cpus::get();
    println!("This system have {} cpus.", num);
}