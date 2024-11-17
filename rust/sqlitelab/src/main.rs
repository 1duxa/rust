use sqlite::State;

fn main() -> Result<(), sqlite::Error> {
    let connection = sqlite::open(":memory").unwrap();
    let query = "
    DROP TABLE IF EXISTS users;
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();
    let query = "SELECT * FROM users";
    let mut statment = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statment.next() {
        println!("name = {}", statment.read::<String, _>("name").unwrap());
        println!("age = {}", statment.read::<i64, _>("age").unwrap());
    }
    let query = "UPDATE users SET name = 'hello world'";
    connection.execute(query).unwrap();
    let query = "SELECT * FROM users";
    let mut statment = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statment.next() {
        println!("name = {}", statment.read::<String, _>("name").unwrap());
        println!("age = {}", statment.read::<i64, _>("age").unwrap());
    }
    Ok(())
}
