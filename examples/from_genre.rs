use sbml::parse_document;

fn main() {
    let file_str = std::fs::read_to_string("examples/EcoliCore.xml").unwrap();
    let res = parse_document(&file_str);
    println!("{:?}", res.unwrap());
}
