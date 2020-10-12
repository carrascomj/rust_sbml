use rust_sbml::Model;

fn main() {
    let file_str = std::fs::read_to_string("examples/EcoliCore.xml").unwrap();
    let res = Model::parse(&file_str);
    println!("{:?}", res.unwrap());
}
