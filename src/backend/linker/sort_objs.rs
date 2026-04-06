use std::collections::{HashMap, VecDeque};
use crate::backend::linker::obj_file::ObjFile;

pub fn sort_objs_bfs(mut objs: Vec<ObjFile>) -> Result<Vec<ObjFile>, String> {

    let mut obj_map: HashMap<String, ObjFile> =
        objs.drain(..).map(|o| (o.name.clone(), o)).collect();

    let mut indegree: HashMap<String, usize> = HashMap::new();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for name in obj_map.keys() {
        indegree.insert(name.clone(), 0);
    }
    
    for (name, obj) in &obj_map {
        for import in &obj.imports {
            if !obj_map.contains_key(import) {
                return Err(format!(
                    "Module '{}' imports missing module '{}'",
                    name, import
                ));
            }
            graph.entry(import.clone())
                .or_default()
                .push(name.clone());

            *indegree.get_mut(name).unwrap() += 1;
        }
    }

    let mut queue = VecDeque::new();

    for (name, deg) in &indegree {
        if *deg == 0 {
            queue.push_back(name.clone());
        }
    }

    let mut result = Vec::new();

    while let Some(name) = queue.pop_front() {

        let obj = obj_map.remove(&name).unwrap();
        result.push(obj);

        if let Some(dependents) = graph.get(&name) {
            for dep in dependents {
                let entry = indegree.get_mut(dep).unwrap();
                *entry -= 1;

                if *entry == 0 {
                    queue.push_back(dep.clone());
                }
            }
        }
    }

    if !obj_map.is_empty() {
        let remaining: Vec<_> = obj_map.keys().cloned().collect();
        return Err(format!(
            "Import cycle detected between modules: {:?}",
            remaining
        ));
    }

    Ok(result)
}
