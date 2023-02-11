// MIT License
//
// Copyright (c) 2023 Sophie Katz
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use db::{connect_db, entities::user};
use futures::executor;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

async fn seed_user(db: &DatabaseConnection) -> Result<(), DbErr> {
    let user_admin = user::ActiveModel {
        username: Set("admin".to_owned()),
        ..Default::default()
    };

    user_admin.insert(db).await?;

    Ok(())
}

async fn seed(db: &DatabaseConnection) -> Result<(), DbErr> {
    seed_user(db).await?;

    Ok(())
}

fn main() {
    let db = connect_db().expect("unable to connect to database");

    executor::block_on(seed(&db)).expect("error while seeding");
}
