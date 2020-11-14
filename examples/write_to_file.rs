use rust_sbml::ModelRaw;

fn main() {
    let example = r#"<?xml version="1.0" encoding="UTF-8"?>
    <sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
         <model timeUnits="second" extentUnits="mole" substanceUnits="mole">
            <listOfReactions>
            <reaction id='J1' reversible='false'>
             <listOfReactants>
                     <speciesReference species='X0' stoichiometry='2' constant='true'/>
                 </listOfReactants>
              </reaction>
             <reaction id='J2' reversible='false'>
                 <listOfReactants>
                 <speciesReference species='CAP' stoichiometry='2' constant='true'/>
                 <speciesReference species='ZOOM' stoichiometry='-2' constant='true'/>
                 </listOfReactants>
             </reaction>
            </listOfReactions>
         </model>
    </sbml>"#;
    let res = ModelRaw::parse(&example).unwrap();
    println!("{:?}", res);
    std::fs::write("from_memory.xml", res.to_string().unwrap()).unwrap();
}
