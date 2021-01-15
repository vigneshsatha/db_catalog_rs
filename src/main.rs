// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: i8
// }

// impl Person {
//     fn get_name(self) -> String {
//         self.name
//     }
//     fn get_age(self) -> i8 {
//         self.age
//     }
// }

// impl Display for Person {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}", self.age)
//     }
// }

// fn main() {
//     let person = Person {
//         name: String::from("Vignesh"),
//         age: 28
//     };

//     println!("{}", person);
// }

use tokio_postgres::{Error, NoTls};

pub mod databases;

pub use crate::databases::db_traits;
pub use crate::databases::db_traits::Catalog;

fn main2() {
    // let postgres = db_traits::Postgres {
    //     conn_url: String::from(""),
    //     name: String::from(""),
    // };
    // let mut tables = postgres.query_tables();

    // let length = tables.len();
    // for i in 0..length {
    //     let table_box = match tables.pop() {
    //         Some(x) => x,
    //         None => Box::new(String::from("")),
    //     };

    //     println!("{}", *table_box);
    // }
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=vic dbname=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let postgres = db_traits::Postgres {
        name: String::from("postgres"),
        client: client
    };
    let tables = postgres.query_tables(&String::from("emp")).await.unwrap();

    for table in tables.iter() {
        println!("{}", table);
        let columns = postgres.query_columns(&String::from("emp"), &table).await.unwrap();
        for column in columns.iter() {
            println!("{}", column.column_name.as_ref().unwrap())
        }
        let count = postgres.query_row_count(&String::from("emp"), &table).await.unwrap();

        println!("Count:{}", count)
    }
    // Now we can execute a simple statement that just returns its parameter.
    // let rows = client.query("SELECT id, name from emp.users", &[]).await?;

    // // And then check that we got back the same string we sent over.
    // let id: i32 = rows[0].get(0);
    // let name: String = rows[0].get(1);
    // assert_eq!(id, 1);
    // assert_eq!(name, String::from("vignesh"));
    // println!("{}", id);
    Ok(())
}
