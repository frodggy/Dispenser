use requestty::{Question};

pub fn create() {
    prompt()
}

fn prompt() {
    let question = vec![
        Question::select("server_software")
            .message("what server software would like to use")
            .choices(vec![
                "Spigot",
                "Paper",
                "Forge",
                "Vanilla",
            ])
            .default(3)
            .build(),
    ];
    let answers = requestty::prompt(question).unwrap();
    for answers in answers.values() {
        println!("{:?}", answers)
    }
}
