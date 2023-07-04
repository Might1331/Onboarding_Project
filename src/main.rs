use typedb_client::{
    concept::{Attribute, Concept, Value},
    error::ConnectionError,
    Connection, DatabaseManager, Error, Options, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write},
    Credential, Database
};
use futures::{executor::block_on, StreamExt};
use  std::fs;
use text_io::read;

const TEST_DATABASE: &str = "onboarding3";
 
fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

async fn load_data(connection: Connection)->std::io::Result<()>{
    let data=fs::read_to_string("./src/data.tql")?;    
    let databases = DatabaseManager::new(connection.clone());
    if databases.contains(TEST_DATABASE).await.unwrap()==false {
        databases.create(TEST_DATABASE).await;    // insert data
        let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
        let transaction = session.transaction(Write).await.unwrap();
        let _ = transaction.query().insert(data.as_str());
        transaction.commit().await.unwrap();
        println!("\nData Loaded\n");
    }else {
        println!("\nData Already Loaded\n");
    }


    Ok(())
}

async fn load_schema(connection: Connection)->std::io::Result<()>{
    let schema = fs::read_to_string("./src/schema.tql")?;
    // query_options(con.clone()).await.unwrap();
    let databases = DatabaseManager::new(connection.clone());
    if databases.contains(TEST_DATABASE).await.unwrap()==false {
        databases.create(TEST_DATABASE).await;
        // define schema
        let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Schema).await.unwrap();
        let transaction = session.transaction(Write).await.unwrap();
        transaction.query().define(schema.as_str()).await.unwrap();
        transaction.commit().await.unwrap();
        println!("\nSchema Loaded\n");
    }else {
        println!("\nSchema Already Defined\n");
    }

    Ok(())
}

async fn query1(connection: Connection,q: String)->std::io::Result<()>{
    println!("::Q1::");
    let databases = DatabaseManager::new(connection.clone());
    println!("::Q1::");
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    println!("::Q1::");
    let transaction = session.transaction(Read).await.unwrap();
    println!("::Q1::");
    let mut answer_stream = transaction.query().match_(q.as_str()).unwrap();
    println!("::Q1::");
    while let Some(result) = answer_stream.next().await{
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute { value: Value::String(value), .. }) = concept {
                        println!("{}",value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }
    Ok(())
}
async fn query2(connection: Connection,q: String)->std::io::Result<()>{    
    println!("::Q2::");
    let databases = DatabaseManager::new(connection.clone());
    println!("::Q2::");
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    println!("::Q2::");
    let transaction = session.transaction(Read).await.unwrap();
    println!("::Q2::");
    let mut answer = transaction.query().match_aggregate(q.as_str()).await.unwrap();
    println!("Answe for Q2:  {}\n",answer.into_i64());
    Ok(())
}
async fn query3(connection: Connection,q: String)->std::io::Result<()>{    
    println!("::Q3::");
    let databases = DatabaseManager::new(connection.clone());
    println!("::Q3::");
    let session = Session::new(databases.get(TEST_DATABASE).await.unwrap(), Data).await.unwrap();
    println!("::Q3::");
    let transaction = session.transaction(Read).await.unwrap();
    println!("::Q3::");
    let mut answer_stream = transaction.query().match_(q.as_str()).unwrap();
    println!("::Q3::");
    while let Some(result) = answer_stream.next().await{
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute { value: Value::String(value), .. }) = concept {
                        println!("{}",value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }
    Ok(())
}

async fn runIO(connection: Connection){
    println!("\nQ1->What places could buy raw_food=$x ?\nQ2->Get count of non-vegetarian outlets with vegetarian specialities.\nQ3->Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.\nWhat query would you like to make? Enter 1,2 or 3.\n");
    let qtype: i32=read!();
    if qtype==1{
        println!("Enter the name of raw_food x : ");
        let inp: String=read!();
        let mut x=inp.as_str();
        let mut p0="match $rf isa raw_food, has name \"";
        let mut p1="\";
        $is_i (raw_food: $rf,dish: $d) isa is_ingredient;
        $sl (seller: $m,product: $d) isa sells;
        $m has name $n,has call_number $c;
        get $n;";
        let mut f="".to_owned();
        f=f+p0+x+p1;
        query1(connection,f).await;
    }else if qtype==2{
        let mut p0="match 
        $m2 isa menu, has is_vegetarian false,has name $n2;
        $d2 isa dish, has is_vegetarian false;
        $sp2 (restaurant: $m2,$d2) isa speciality;
        get $m2;count;";
        let mut f="".to_owned();
        f=f+p0;
        query2(connection,f).await;
    }else if qtype==3{
        println!("Enter the avg_rating of Restraunt r : ");
        let inp0: String=read!();
        let mut r=inp0.as_str();
        println!("Enter the threshold price for raw_ingredient p : ");
        let inp1: String=read!();
        let mut p=inp1.as_str();
        let mut p0="match
        $m3 isa menu, has avg_rating>";
        let mut p1=",has name $mn3;
        $d3 isa dish, has name $dn3;
        $sl3 (seller: $m3,product: $d3) isa sells, has price $p3;
        $p3>";
        let mut p2=";
        $rf3 isa raw_food,has name $rfn3;
        $isn3 (raw_food: $rf3,$d3) isa is_ingredient;
        get $rfn3;";
        let mut f="".to_owned();
        f=f+p0+r+p1+p+p2;

        query3(connection,f).await;
    }else{
        println!("Query entered is not 1,2 or 3\n");
    }
}

async fn mymain()->std::io::Result<()>{

    let con=new_core_connection().expect("Line: 74");
    load_schema(con.clone()).await?;
    load_data(con.clone()).await?;

    runIO(con.clone()).await;

    Ok(())
}

#[tokio::main]
async fn main(){
    // let r=mymain();
    // block_on(r);
    mymain().await;
}