use rand::{self, Rng};
use std::{error::Error, fmt};
use surrealdb::{
    sql::{Thing, Value},
    Datastore, Session,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let ds = Datastore::new("memory").await?;
    let ds = Datastore::new("file://test.db").await?;

    let ses = Session::for_kv().with_ns("test").with_db("test");
    // let ast = r#"CREATE entry SET when = time::now(), num = {};"#;

    let entry_num = 10_000;
    // for i in 0..entry_num {
    //     ds.execute(
    //         &format!(r#"CREATE entry SET when = time::now(), num = {};"#, i),
    //         &ses,
    //         None,
    //         false,
    //     )
    //     .await?;
    // }

    // let select_response = ds
    //     .execute("SELECT * FROM entry;", &ses, None, false)
    //     .await?;
    // // println!("{:#?}", select_response);

    // let select_res = select_response[0].output().unwrap();

    // let mut id_acc = vec![];

    // if let Value::Array(rows) = select_res {
    //     for row in rows.iter() {
    //         if let Value::Object(obj) = row {
    //             // println!("{}", obj);
    //             id_acc.push(get_thing(obj.get("id").unwrap())?);
    //         }
    //     }
    // }

    // for id in id_acc {
    //     println!("{:?}", id.to_string());
    // }

    let mut rng = rand::thread_rng();

    // let select_response = ds
    //     .execute(
    //         &format!(
    //             "SELECT * FROM entry WHERE num = {};",
    //             rng.gen_range(0..entry_num)
    //         ),
    //         &ses,
    //         None,
    //         false,
    //     )
    //     .await?;

    let select_response = &ds
        .execute("SELECT * FROM entry;", &ses, None, false)
        .await?;

    let select_res = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_res {
        for row in rows.iter() {
            if let Value::Object(obj) = row {
                println!("{:?}", obj)
            }
        }
    }

    println!("time taken:{}", select_response[0].speed());
    Ok(())
}

fn get_thing(val: &Value) -> Result<&Thing, GenericError> {
    match val {
        Value::Thing(t) => Ok(t),
        _ => Err(GenericError::new("No Thing in Value enum")),
    }
}

#[derive(Debug)]
struct GenericError {
    details: String,
}

impl GenericError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}
