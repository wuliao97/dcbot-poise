



pub fn extract_vec(material: &Vec<String>, (first, last): (usize, usize)) -> Vec<String> {
    let tmp: &[String] = &material[first..=last];
    tmp.to_vec()
}

pub fn distinction_vec(material: &Vec<String>, max_value: usize) -> Vec<Vec<String>> {
    let mut vec: Vec<Vec<String>> = Vec::new();
    let limit = material.len();
    let max_count = (limit as f32 / max_value as f32).ceil() as usize;

    for idx in 0..max_count {
        let (first, last) = {
            let index = idx;
            let first = &index * max_value;
            let last = if first + max_value > material.len() && material.len() % max_value != 0 {
                let tmp = limit.clone();
                tmp - 1
            } else {
                (index + 1) * max_value - 1
            };
            (first, last)
        };
        vec.push(extract_vec(material, (first, last)));
    }
    vec
}