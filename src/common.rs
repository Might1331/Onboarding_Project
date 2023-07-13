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

use std::{fs, io};
use typedb_client::{
    answer::ConceptMap,
    concept::{Attribute, Concept, Value},
    Connection, DatabaseManager, Error, Session,
    SessionType::{Data, Schema},
    TransactionType::Write,
};

pub fn read_input() -> Result<String, io::Error> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    Ok(input_string.trim().to_string())
}

pub fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

pub async fn load_schema(
    connection: Connection,
    database_name: &str,
    tql_file_path: &str,
) -> Result<(), typedb_client::error::Error> {
    let schema = fs::read_to_string(tql_file_path)?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(database_name).await?, Schema).await?;
    let transaction = session.transaction(Write).await?;
    transaction.query().define(schema.as_str()).await?;
    transaction.commit().await?;
    drop(session);
    println!("\nSchema Defined Successfully\n");
    Ok(())
}

pub async fn load_data(
    connection: Connection,
    database_name: &str,
    tql_file_path: &str,
) -> Result<(), typedb_client::error::Error> {
    let data = fs::read_to_string(tql_file_path)?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(database_name).await?, Data).await?;
    let transaction = session.transaction(Write).await?;
    transaction.query().insert(data.as_str()).ok();
    transaction.commit().await?;
    println!("\nData Loaded Successfully\n");
    Ok(())
}

pub fn print_concept_map(result: Result<ConceptMap, Error>) -> Result<(), Error> {
    match result {
        Ok(concept_map) => {
            for (_, concept) in concept_map {
                if let Concept::Attribute(Attribute {
                    value: Value::String(value),
                    ..
                }) = concept
                {
                    println!("{}", value);
                }
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}
