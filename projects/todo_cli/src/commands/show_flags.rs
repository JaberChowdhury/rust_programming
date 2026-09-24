pub fn show_flags() {
    println!(
        r#"todo_cli - Todo CLI

USAGE:
    todo_cli <COMMAND> [OPTIONS]

COMMANDS:
    add                         Add a new todo
    del, delete <id>            Delete a todo
    up, update <id>             Update an existing todo
    show, list                  Show all todos
    search <query>              Search todos

ADD OPTIONS:
    --title <title>             Todo title
    --desc <description>        Todo description
    --done <0|1>                Todo completion status

UPDATE OPTIONS:
    --title <title>             New todo title
    --desc <description>        New todo description
    --done <0|1>                New completion status

EXAMPLES:
    todo_cli add --title "Buy milk"

    todo_cli add \
        --title "Learn Rust" \
        --desc "Learn Rust CLI development" \
        --done 0

    todo_cli del 1

    todo_cli delete 1

    todo_cli up 1 --title "Learn Rust properly"

    todo_cli update 1 --desc "Finish CLI project" --done 1

    todo_cli show

    todo_cli list

    todo_cli search "rust"
"#
    );
}
