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
use std::fs;
use typedb_client::{
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::{Write},
};

pub fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

pub async fn load_schema(connection: Connection,database_name: &str) -> std::io::Result<()> {
    let schema = fs::read_to_string("./src/schema.tql")?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(database_name).await.unwrap(), Schema)
        .await
        .unwrap();
    let transaction = session.transaction(Write).await.unwrap();
    transaction.query().define(schema.as_str()).await.unwrap();
    transaction.commit().await.unwrap();
    println!("\nSchema Loaded\n");
    Ok(())
}

pub async fn load_data(connection: Connection,database_name: &str) -> std::io::Result<()> {
    let data = fs::read_to_string("./src/data.tql")?;
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(database_name).await.unwrap(), Data)
        .await
        .unwrap();
    let transaction = session.transaction(Write).await.unwrap();
    let _ = transaction.query().insert(data.as_str());
    transaction.commit().await.unwrap();
    println!("\nData Loaded\n");
    Ok(())
}