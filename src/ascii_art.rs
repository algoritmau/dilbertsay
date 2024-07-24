use std::collections::HashMap;

fn create_ascii_art_catalog() -> HashMap<String, String> {
    let mut ascii_art_catalog: HashMap<String, String> = HashMap::new();

    ascii_art_catalog.insert(
        "catbert".to_string(),
        "
            ^-\"`^,
           /=  -OO
           |=  ->j<-
          (|= U   ;
          ))\\=, ./
         ))  `|||
        ()    |_))
        ".to_string(),
    );

    ascii_art_catalog.insert(
        "dilbert".to_string(),
        "
         (~'~~'~~'~~)
          |        |
          |        |
          |       ~|~
          |-------())
          (        _)
          |        |
          |        |
          ''..     |
          |'..'---_/\\
         /    ''---|| /\\
        /     \\    \\\\/\\/
        |  \\  /     \\_/
        |   \\/\\\\    | \\
        ".to_string(),
    );

    ascii_art_catalog.insert(
        "dilbert_full".to_string(),
        "
           (`'`'`'`')
            |      |
            |      |
           (|-()()-|)
            | (__) |
            |      |
            |______|
           /._/\\/\\_.\\
          /  , /\\    \\
         ; / \\\\|| __\\ ;
         |-|  './ \\/|-|
         \\ |   |    | /
          '\\___|____/`
            |--LI--|
            |  |   |
            |  |   |
            |  |   |
            |  |   |
            |  |   |
            |__|___|
        .----'=||='----.
        `\"\"\"\"`\"  \"`\"\"\"\"`
        ".to_string()
    );

    ascii_art_catalog.insert(
        "dogbert".to_string(),
        "TBD".to_string(),
    );

    ascii_art_catalog.insert(
        "tim".to_string(),
        "TBD".to_string(),
    );

    ascii_art_catalog.insert(
        "alice".to_string(),
        "TBD".to_string(),
    );

    ascii_art_catalog.insert(
        "wally".to_string(),
        "TBD".to_string(),
    );

    ascii_art_catalog
}


pub fn get_ascii_art(character: String) -> String {
    let catalog = create_ascii_art_catalog();
    if catalog.contains_key(&character) {
        return catalog.get(&character).unwrap().to_string()
    } else {
        return catalog.get("dilbert").unwrap().to_string()
    };

    //let ascii_art = if catalog.contains_key(&character) {
    //    catalog.get(&character)
    //} else {
    //    catalog.get("dilbert")
    //};
    //
    //ascii_art
}

