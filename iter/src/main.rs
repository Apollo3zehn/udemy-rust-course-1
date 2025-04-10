
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{:#?}", element);
    // }
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements
        .iter_mut()
        .for_each(|element| element.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|element| element
            .chars()
            .map(|char| char.to_string()).collect())
        .collect()
}

fn find_color_or(
    elements: &[String],
    search: &str,
    fallback: &str
) -> String {
    elements
        .iter()
        .find(|element| element.contains(search))
        .map_or(String::from(fallback), |element| element.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    // shorten_strings(&mut colors[1..3]);
    // print_elements(&colors);

    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let found_color = find_color_or(
        &colors,
        "re",
        "Orange"
    );

    println!("{}", found_color);
}

