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
use futures::TryStreamExt;
use std::{fs, io};
use typedb_client::{
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::Write,
};

#[derive(Debug)]
pub enum HandleError {
    Io(io::Error),
    TypeDB(typedb_client::error::Error),
}

pub fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

pub async fn load_schema(
    connection: Connection,
    database_name: &str,
    tql_file_path: &str,
) -> Result<(), HandleError> {
    let schema = fs::read_to_string(tql_file_path).map_err(HandleError::Io)?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(
        databases
            .get(database_name)
            .await
            .map_err(HandleError::TypeDB)?,
        Schema,
    )
    .await
    .map_err(HandleError::TypeDB)?;
    let transaction = session
        .transaction(Write)
        .await
        .map_err(HandleError::TypeDB)?;
    transaction
        .query()
        .define(schema.as_str())
        .await
        .map_err(HandleError::TypeDB)?;
    transaction.commit().await.map_err(HandleError::TypeDB)?;
    drop(session);
    println!("\nSchema Defined Successfully\n");
    Ok(())
}

pub async fn load_data(
    connection: Connection,
    database_name: &str,
    tql_file_path: &str,
) -> Result<(), HandleError> {
    let data = fs::read_to_string(tql_file_path).map_err(HandleError::Io)?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(
        databases
            .get(database_name)
            .await
            .map_err(HandleError::TypeDB)?,
        Data,
    )
    .await
    .map_err(HandleError::TypeDB)?;
    let transaction = session
        .transaction(Write)
        .await
        .map_err(HandleError::TypeDB)?;
    let inserted_query = transaction.query().insert(data.as_str());
    let _ = inserted_query.unwrap().try_collect::<Vec<_>>().await;
    transaction.commit().await.map_err(HandleError::TypeDB)?;
    println!("\nData Loaded Successfully\n");
    Ok(())
}
