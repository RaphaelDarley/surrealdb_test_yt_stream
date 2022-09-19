use rand::{self, Rng};
use std::{collections::BTreeMap, error::Error, fmt, sync::Arc, thread};
use surrealdb::{
    sql::{self, Data},
    sql::{Thing, Value},
    Datastore, Session,
};
use tokio;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ds = Datastore::new("memory").await?;
    // let ds = Datastore::new("file://test.db").await?;

    let ses = Session::for_kv().with_ns("test").with_db("test");
    // let ast = r#"CREATE entry SET when = time::now(), num = {};"#;

    let entry_num = 2;
    for i in 0..entry_num {
        let mut vars = BTreeMap::new();
        vars.insert(
            "num".to_string(),
            sql::Value::Number(sql::Number::Int(i as i64)),
        );
        ds.execute(
            r#"CREATE entry SET when = time::now(), num = $num;"#,
            &ses,
            Some(vars),
            false,
        )
        .await?;
    }

    // let select_res = select_response[0].output().unwrap();

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

    let select_response = &ds
        .execute("SELECT * FROM entry;", &ses, None, false)
        .await?;

    let select_result = select_response[0].output().unwrap();

    let mut id_acc = vec![];

    if let Value::Array(rows) = select_result {
        for row in rows.iter() {
            if let Value::Object(obj) = row {
                // println!("{:?}", obj)
                id_acc.push(obj.rid().unwrap());
            }
        }
    }

    println!("time taken:{}", select_response[0].speed());

    // for id in id_acc {
    //     println!("{:?}", id.to_string());
    // }

    // let mut vars2 = BTreeMap::new();
    // vars2.insert("rid".to_string(), sql::Value::Thing(id_acc[0].clone()));
    // let select_response2 = &ds
    //     .execute("SELECT * FROM $rid;", &ses, Some(vars2), false)
    //     .await?;

    // let select_result2 = select_response2[0].output().unwrap();

    // if let Value::Array(rows) = select_result2 {
    //     for row in rows.iter() {
    //         if let Value::Object(obj) = row {
    //             println!("{:?}", obj)
    //         }
    //     }
    // }

    let thread_ds = Arc::new(Mutex::new(ds));
    let thread_ses = Arc::new(Mutex::new(ses));
    let db_thread = {
        let thread_ds_clone = thread_ds.clone();
        let thread_ses_clone = thread_ses.clone();
        thread::spawn(|| async move { print_db_entries(thread_ds_clone, thread_ses_clone).await })
    };

    let test = db_thread.join().unwrap().await;

    print_db_entries(thread_ds, thread_ses).await.unwrap();

    Ok(())
}

async fn print_db_entries(
    thread_ds: Arc<Mutex<Datastore>>,
    thread_ses: Arc<Mutex<Session>>,
) -> Result<(), Box<dyn Error>> {
    let ds = &*thread_ds.lock().await;
    let ses = &*thread_ses.lock().await;
    let select_response = ds.execute("SELECT * FROM entry;", ses, None, false).await?;

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        for row in rows.iter() {
            if let Value::Object(obj) = row {
                println!("{:?}", obj)
            }
        }
    }

    Ok(())
}
