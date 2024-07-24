use std::collections::HashMap;

fn create_ascii_art_catalog() -> HashMap<String, String> {
    let mut ascii_art_catalog: HashMap<String, String> = HashMap::new();

    ascii_art_catalog.insert(
    "catbert".to_string(),
    // This ASCII pic can be found at https://asciiart.website/index.php?art=comics/dilbert
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
    // This ASCII pic can be found at https://asciiart.website/index.php?art=comics/dilbert
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
    // This ASCII pic can be found at https://www.asciiart.eu/comics/dilbert
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
    // This ASCII pic can be found at https://www.asciiart.eu/comics/dilbert
    "
          .-\"\"-.
        .'      \\
       / /  |-()()
      ;  |  |  ()|
      |  `\"`     |
    ,_|   | |    |
    `-;   (_}    ;
       '.,   __.'
         / /| |
        / / | |
       (__) (__)
    ".to_string(),
    );

    ascii_art_catalog.insert(
    "tim".to_string(),
    // This ASCII pic can be found at https://afrodita.rcub.bg.ac.rs/~ggajic/ascii-art/dilbert.txt
    "
             #      #
             ##     #
            ###    ###
           #####   ###
            ####__####
           #####   ~~~)
           (c      * *_
            |         _)
            |      _  |
            |     (_  |
           _|_________|
          /    \\\\/ \\__/\\
         /      \\\\  \\~\\ \\
        /   |  | \\\\  \\~\\ \\
       /    |  |  \\\\  \\~\\ \\
      /     |  |   \\\\  \\~\\ \\
     /      |  |    \\\\  \\~\\ \\
    {       |__|     \\\\  \\~\\ \\
    |_______(__)____  \\\\  \\~\\ \\
        |_____o_____|  \\\\  \"\" |
        |           |__||_____|
    ".to_string(),
    );

    ascii_art_catalog.insert(
    "alice".to_string(),
    // This ASCII pic can be found at https://afrodita.rcub.bg.ac.rs/~ggajic/ascii-art/dilbert.txt
    "
          _   _
         {{} {}}
        {{}}{{}}
       {{{}}}{{}}
      {{{}}}}^^^^-.
        |      ~~ ~~
       |@       ' '|
        |        --.
        |        __'         _______
        |        O |        |       |
        ''';.......         |       |
        \"\"\"'..()...|______.---,     |
       /  \\\\  /\\  //      | |~      |
      /    || | | || _____| |       |
     /     \\\\/  \\//  |     -|_______|
     |      \\\\..//   |
     |       \\\\//    |
    ".to_string(),
    );

    ascii_art_catalog.insert(
    "wally".to_string(),
    // This ASCII pic can be found at https://asciiart.website/index.php?art=comics/dilbert
    "
            .~~----~,
           /         \\
          :___________\\//_
          |    |    |  ?)
          |   /|    |  |
          |__(_|____|  |
          |   _        |
          |   _)       |
          |____________|
         /____/_\\______\\
        /    /::/       \\
       /    /::/ _LL/   |
      /    /::/ |  /____|
     /    /::/ _|___| |_|__
    /     \\:/  |    | |   |
    |      \"   |    | |   |
    |__________|    | |   |
       |#######|____(__)__|
       |################|
    ".to_string(),
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

