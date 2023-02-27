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
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use db::{
    entities::{configuration_type_reference, configuration_type_reference_audit},
    testing::initialize_unit_database,
};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};

#[test]
fn connect_db_development() -> Result<(), db::Error> {
    db::connect_db(db::DatabaseInstance::Development).map(|_| ())
}

#[test]
fn connect_db_unit() -> Result<(), db::Error> {
    db::connect_db(db::DatabaseInstance::Unit).map(|_| ())
}

#[async_std::test]
async fn test_auditing() -> Result<(), db::Error> {
    // Initialize unit testing database
    let connection = initialize_unit_database().await?;

    // Make sure there are no entries already in the source table
    assert_eq!(
        configuration_type_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![]
    );

    // Make sure there are no entries already in the audit table
    assert_eq!(
        configuration_type_reference_audit::Entity::find()
            .all(&connection)
            .await?,
        vec![]
    );

    // Insert one
    let first_inserted_id =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("boolean".to_owned()),
            description: Set("A true/false value".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

    assert!(first_inserted_id > 0);

    // See if it inserted correctly
    assert_eq!(
        configuration_type_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![configuration_type_reference::Model {
            id: first_inserted_id,
            name: "boolean".to_owned(),
            description: "A true/false value".to_owned(),
            deactivate_timestamp: None,
        }]
    );

    // See if it was audited correctly
    let audit_rows = configuration_type_reference_audit::Entity::find()
        .all(&connection)
        .await?;

    assert_eq!(audit_rows.len(), 1);
    assert_eq!(audit_rows[0].id, Some(first_inserted_id));
    assert_eq!(audit_rows[0].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[0].description,
        Some("A true/false value".to_owned())
    );
    assert_eq!(audit_rows[0].deactivate_timestamp, None);

    let first_audit_id = audit_rows[0].audit_id;

    assert!(first_audit_id > 0);
    assert_eq!(audit_rows[0].audit_action, "I");

    // Try and insert a duplicate - expected to fail
    assert!(configuration_type_reference::Entity::insert(
        configuration_type_reference::ActiveModel {
            name: Set("boolean".to_owned()),
            description: Set("A different true/false value".to_owned()),
            ..Default::default()
        }
    )
    .exec(&connection)
    .await
    .is_err());

    // Update one
    let mut first_inserted: configuration_type_reference::ActiveModel =
        configuration_type_reference::Entity::find_by_id(first_inserted_id)
            .one(&connection)
            .await?
            .unwrap()
            .into();

    first_inserted.deactivate_timestamp = Set(Some(chrono::Utc::now().naive_utc()));

    first_inserted.update(&connection).await?;

    // See if it was updated correctly
    let source_rows = configuration_type_reference::Entity::find()
        .all(&connection)
        .await?;

    assert_eq!(source_rows.len(), 1);
    assert_eq!(source_rows[0].id, first_inserted_id);
    assert_eq!(source_rows[0].name, "boolean".to_owned());
    assert_eq!(source_rows[0].description, "A true/false value".to_owned());
    assert!(source_rows[0].deactivate_timestamp.is_some());

    // See if it was audited correctly
    let audit_rows = configuration_type_reference_audit::Entity::find()
        .order_by_asc(configuration_type_reference_audit::Column::AuditId)
        .all(&connection)
        .await?;

    assert_eq!(audit_rows.len(), 2);

    assert_eq!(audit_rows[0].id, Some(first_inserted_id));
    assert_eq!(audit_rows[0].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[0].description,
        Some("A true/false value".to_owned())
    );
    assert_eq!(audit_rows[0].deactivate_timestamp, None);

    let first_audit_id = audit_rows[0].audit_id;

    assert!(first_audit_id > 0);
    assert_eq!(audit_rows[0].audit_action, "I");

    assert_eq!(audit_rows[1].id, Some(first_inserted_id));
    assert_eq!(audit_rows[1].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[1].description,
        Some("A true/false value".to_owned())
    );
    assert_eq!(audit_rows[1].deactivate_timestamp, None);

    let second_audit_id = audit_rows[1].audit_id;

    assert!(first_audit_id < second_audit_id);
    assert_eq!(audit_rows[1].audit_action, "U");

    // Delete one
    configuration_type_reference::Entity::delete_by_id(first_inserted_id)
        .exec(&connection)
        .await?;

    // See if it was deleted correctly
    let source_rows = configuration_type_reference::Entity::find()
        .all(&connection)
        .await?;

    assert_eq!(source_rows, vec![]);

    // See if it was audited correctly
    let audit_rows = configuration_type_reference_audit::Entity::find()
        .order_by_asc(configuration_type_reference_audit::Column::AuditId)
        .all(&connection)
        .await?;

    assert_eq!(audit_rows.len(), 3);

    assert_eq!(audit_rows[0].id, Some(first_inserted_id));
    assert_eq!(audit_rows[0].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[0].description,
        Some("A true/false value".to_owned())
    );
    assert_eq!(audit_rows[0].deactivate_timestamp, None);

    let first_audit_id = audit_rows[0].audit_id;

    assert!(first_audit_id > 0);
    assert_eq!(audit_rows[0].audit_action, "I");

    assert_eq!(audit_rows[1].id, Some(first_inserted_id));
    assert_eq!(audit_rows[1].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[1].description,
        Some("A true/false value".to_owned())
    );
    assert_eq!(audit_rows[1].deactivate_timestamp, None);

    let second_audit_id = audit_rows[1].audit_id;

    assert!(first_audit_id < second_audit_id);
    assert_eq!(audit_rows[1].audit_action, "U");

    assert_eq!(audit_rows[2].id, Some(first_inserted_id));
    assert_eq!(audit_rows[2].name, Some("boolean".to_owned()));
    assert_eq!(
        audit_rows[2].description,
        Some("A true/false value".to_owned())
    );
    assert!(audit_rows[2].deactivate_timestamp.is_some());

    let third_audit_id = audit_rows[2].audit_id;

    assert!(second_audit_id < third_audit_id);
    assert_eq!(audit_rows[2].audit_action, "D");

    Ok(())
}
