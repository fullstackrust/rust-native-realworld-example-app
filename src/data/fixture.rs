pub fn tag_data() -> Vec<String> {
    let data = vec![
        "but",
        "test",
        "dragons",
        "training",
        "tags",
        "as",
        "coffee",
        "animation",
        "baby",
        "flowers",
        "money",
        "caramel",
        "cars",
        "japan",
        "tag1",
        "happiness",
        "sugar",
        "clean",
        "cookies",
        "sushi",
    ];

    data.into_iter().map(|s| s.to_string()).collect()
}
