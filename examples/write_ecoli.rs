use rust_sbml::ModelRaw;

fn main() {
    let file_str = std::fs::read_to_string("examples/EcoliCore.xml").unwrap();
    let res = ModelRaw::parse(&file_str).unwrap();
    println!("{:?}", res);
    std::fs::write("ecoli_from_memory.xml", res.to_string().unwrap()).unwrap();
}
