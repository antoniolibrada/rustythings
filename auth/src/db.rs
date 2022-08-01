use crate::{
    errors::{AppError, AppErrorType::*},
    models::User,
};
use postgres::Client;

fn prepare_db(client: &mut Client) {
    client
        .batch_execute(
            "
                CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";
                CREATE TABLE users (
                  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
                  name TEXT NOT NULL,
                  password TEXT NOT NULL,
                  email TEXT NOT NULL UNIQUE
                );
                CREATE TABLE applications (
                  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
                  name TEXT NOT NULL
                );
                CREATE TABLE roles (
                  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
                  name TEXT NOT NULL,
                  application uuid,
                  CONSTRAINT fk_application
                    FOREIGN KEY(application)
                      REFERENCES applications(id)
                      ON DELETE CASCADE
                );
                ",
        )
        .unwrap();
}

fn add_user(
    client: &mut Client,
    name: &String,
    email: &String,
    password: &String,
) -> Result<User, AppError> {
    let stmt = client
        .prepare("INSERT INTO users (name, email, password) values ($1, $2, $3) RETURNING *")
        .unwrap();
    client
        .query(&stmt, &[name, email, password])
        .unwrap()
        .iter()
        .map(User::from_row)
        .collect::<Vec<User>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating user".to_string()),
            cause: Some("Unknown error.".to_string()),
            error_type: DbError,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::{Client, NoTls};
    fn prepare_tests(client: &mut Client) {
        client
            .batch_execute(
                "
                DROP TABLE IF EXISTS roles;
                DROP TABLE IF EXISTS applications;
                DROP TABLE IF EXISTS users;
                ",
            )
            .unwrap();
    }
    #[test]
    fn it_adds_a_user() {
        let mut client = Client::connect("host=localhost user=auth dbname=test", NoTls).unwrap();
        prepare_tests(&mut client);
        prepare_db(&mut client);
        let user = add_user(
            &mut client,
            &"Max".to_string(),
            &"max@example.com".to_string(),
            &"123455".to_string(),
        )
        .unwrap();
        assert_eq!(user.name, "Max");
        assert_eq!(user.email, "max@example.com");
    }
}
