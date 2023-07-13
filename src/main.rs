/*
 * Copyright (C) 2022 Vaticle
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use futures::StreamExt;
use std::io;
use typedb_client::{
    Connection, DatabaseManager, Session, SessionType::Data, TransactionType::Read,
};
mod common;

#[derive(Debug)]
enum HandleError {
    Io(io::Error),
    TypeDB(typedb_client::error::Error),
}

const MENU_DATABASE: &str = "menuDB1";

async fn get_sellers(connection: Connection) -> Result<(), HandleError> {
    let databases: DatabaseManager = DatabaseManager::new(connection.clone());
    let session: Session = Session::new(
        databases
            .get(MENU_DATABASE)
            .await
            .map_err(HandleError::TypeDB)?,
        Data,
    )
    .await
    .map_err(HandleError::TypeDB)?;
    let transaction = session
        .transaction(Read)
        .await
        .map_err(HandleError::TypeDB)?;

    println!("Enter the name of raw_food x : ");
    let x: String = common::read_input().unwrap();
    let q: String = format!(
        "match $rf isa raw_food, has name \"{x}\";
        $is_i (raw_food: $rf,dish: $d) isa is_ingredient;
        $sl (seller: $m,product: $d) isa sells;
        $m has name $n;
        get $n;"
    );
    let mut answer_stream = transaction
        .query()
        .match_(q.as_str())
        .map_err(HandleError::TypeDB)?;
    while let Some(result) = answer_stream.next().await {
        common::print_concept_map(result).map_err(HandleError::TypeDB)?;
    }
    Ok(())
}

async fn get_strange_menu(connection: Connection) -> Result<(), HandleError> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(
        databases
            .get(MENU_DATABASE)
            .await
            .map_err(HandleError::TypeDB)?,
        Data,
    )
    .await
    .unwrap();
    let transaction = session
        .transaction(Read)
        .await
        .map_err(HandleError::TypeDB)?;

    println!("::Q2::");
    let q: String = "
    match $m isa menu, has is_vegetarian false,has name $n;
    $d isa dish, has is_vegetarian true;
    $sp (restaurant: $m,$d) isa speciality;
    get $m;count;"
        .to_string();
    let answer: typedb_client::answer::Numeric = transaction
        .query()
        .match_aggregate(q.as_str())
        .await
        .map_err(HandleError::TypeDB)?;
    println!("Answer for Q2:  {}\n", answer.into_i64());
    Ok(())
}

async fn get_raw_items(connection: Connection) -> Result<(), HandleError> {
    let databases: DatabaseManager = DatabaseManager::new(connection.clone());
    let session: Session = Session::new(
        databases
            .get(MENU_DATABASE)
            .await
            .map_err(HandleError::TypeDB)?,
        Data,
    )
    .await
    .unwrap();
    let transaction: typedb_client::Transaction<'_> = session
        .transaction(Read)
        .await
        .map_err(HandleError::TypeDB)?;

    println!("Enter the avg_rating of Restraunt r : ");
    let avg_rating: String = common::read_input().unwrap();
    println!("Enter the threshold price for raw_ingredient p : ");
    let price: String = common::read_input().unwrap();

    println!("::Q3::");
    let q: String = format!(
        "match $m isa menu, has avg_rating>{avg_rating},has name $mn;
        $d isa dish, has name $dn;
        $sl (seller: $m,product: $d) isa sells, has price $p;
        $p>{price};
        $rf isa raw_food,has name $rfn;
        $isn (raw_food: $rf,$d) isa is_ingredient;
        get $rfn;"
    );
    let mut answer_stream = transaction
        .query()
        .match_(q.as_str())
        .map_err(HandleError::TypeDB)?;
    while let Some(result) = answer_stream.next().await {
        common::print_concept_map(result).map_err(HandleError::TypeDB)?;
    }
    Ok(())
}

async fn query_runner(connection: Connection) -> Result<(), HandleError> {
    println!("Q1->What places could buy raw_food=$x ?");
    println!("Q2->Get count of non-vegetarian outlets with vegetarian specialities.");
    println!("Q3->Get raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.");
    println!("What query would you like to make? Enter 1,2 or 3.\n");
    let qtype: String = common::read_input().unwrap();
    match qtype.as_str() {
        "1" => get_sellers(connection).await?,
        "2" => get_strange_menu(connection).await?,
        "3" => get_raw_items(connection).await?,
        _ => println!("Query entered is not 1,2 or 3\n"),
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), HandleError> {
    let connection: Connection = common::new_core_connection().expect(&line!().to_string());
    let databases: DatabaseManager = DatabaseManager::new(connection.clone());
    if !databases
        .contains(MENU_DATABASE)
        .await
        .map_err(HandleError::TypeDB)?
    {
        databases
            .create(MENU_DATABASE)
            .await
            .map_err(HandleError::TypeDB)?;
        common::load_schema(connection.clone(), MENU_DATABASE, "./src/schema.tql")
            .await
            .expect("load_schema function failed");
        common::load_data(connection.clone(), MENU_DATABASE, "./src/data.tql")
            .await
            .expect("load_data function failed");
    }

    loop {
        query_runner(connection.clone()).await?;
        println!("Enter 0 to exit or 1 to continue:");
        let query_again = common::read_input().map_err(HandleError::Io)?;
        if query_again == "0" {
            break;
        }
    }
    Ok(())
}
