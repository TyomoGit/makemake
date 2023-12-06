use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use crate::material::Material;

pub fn scan(path: &str) -> Result<Material, Error> {
    let mut dependencies = HashSet::new();

    collect(&mut dependencies, path)?;

    let name = path.replace(".c", "");
    Ok(Material::new(name, dependencies))
}

fn collect(set: &mut HashSet<String>, path: &str) -> Result<(), Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#include \"") {
            let mut iter = line.split_whitespace();
            iter.next();
            let mut dependency = iter.next().unwrap().to_string();
            dependency.remove(0);
            dependency.pop();

            collect(set, &dependency)?;
            
            set.insert(dependency);
        }
    }

    Ok(())
}