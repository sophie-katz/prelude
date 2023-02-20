use db::entities::{configuration_reference, configuration_type_reference};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

async fn initialize_unit_database() -> Result<DatabaseConnection, db::Error> {
    let connection = db::connect_db(db::DatabaseInstance::Unit)?;

    // Order is important here due to foreign keys

    configuration_reference::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_type_reference::Entity::delete_many()
        .exec(&connection)
        .await?;

    Ok(connection)
}

#[test]
fn connect_db_development() -> Result<(), db::Error> {
    db::connect_db(db::DatabaseInstance::Development).map(|_| ())
}

#[test]
fn connect_db_unit() -> Result<(), db::Error> {
    db::connect_db(db::DatabaseInstance::Unit).map(|_| ())
}

#[async_std::test]
async fn configuration_type_references() -> Result<(), db::Error> {
    // Initialize unit testing database
    let connection = initialize_unit_database().await?;

    // Make sure there are no entries already in the table
    assert_eq!(
        configuration_type_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![]
    );

    // Insert one
    let first_inserted_id =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("integer".to_owned()),
            description: Set("A signed integer value".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

    assert!(first_inserted_id > 0);

    // Try and insert a duplicate - expected to fail
    assert!(configuration_type_reference::Entity::insert(
        configuration_type_reference::ActiveModel {
            name: Set("integer".to_owned()),
            description: Set("A signed integer value".to_owned()),
            ..Default::default()
        }
    )
    .exec(&connection)
    .await
    .is_err());

    // Select the one entry
    assert_eq!(
        configuration_type_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![configuration_type_reference::Model {
            id: first_inserted_id,
            name: "integer".to_owned(),
            description: "A signed integer value".to_owned()
        }]
    );

    // Insert another unique entry
    assert_eq!(
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("boolean".to_owned()),
            description: Set("A true or false value".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id,
        first_inserted_id + 2
    );

    // Select both entries
    assert_eq!(
        configuration_type_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![
            configuration_type_reference::Model {
                id: first_inserted_id,
                name: "integer".to_owned(),
                description: "A signed integer value".to_owned()
            },
            configuration_type_reference::Model {
                id: first_inserted_id + 2,
                name: "boolean".to_owned(),
                description: "A true or false value".to_owned()
            }
        ]
    );

    // Select one entry by id
    assert_eq!(
        configuration_type_reference::Entity::find_by_id(first_inserted_id)
            .all(&connection)
            .await?,
        vec![configuration_type_reference::Model {
            id: first_inserted_id,
            name: "integer".to_owned(),
            description: "A signed integer value".to_owned()
        }]
    );

    Ok(())
}

#[async_std::test]
async fn configuration_references() -> Result<(), db::Error> {
    // Initialize unit testing database
    let connection = initialize_unit_database().await?;

    // Add in dependency entries
    let configuration_type_id =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("integer".to_owned()),
            description: Set("A signed integer value".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

    assert!(configuration_type_id > 0);

    // Make sure there are no entries already in the table
    assert_eq!(
        configuration_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![]
    );

    // Insert one
    let first_inserted_id =
        configuration_reference::Entity::insert(configuration_reference::ActiveModel {
            name: Set("a".to_owned()),
            description: Set("A".to_owned()),
            type_id: Set(configuration_type_id),
            optional: Set(false),
            allows_multiple: Set(false),
            allows_user_override: Set(false),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

    assert!(first_inserted_id > 0);

    // Try and insert a duplicate - expected to fail
    assert!(
        configuration_reference::Entity::insert(configuration_reference::ActiveModel {
            name: Set("a".to_owned()),
            description: Set("A".to_owned()),
            type_id: Set(configuration_type_id),
            optional: Set(false),
            allows_multiple: Set(false),
            allows_user_override: Set(false),
            ..Default::default()
        })
        .exec(&connection)
        .await
        .is_err()
    );

    // Try and insert an entry that violates a foreign key - expected to fail
    assert!(
        configuration_reference::Entity::insert(configuration_reference::ActiveModel {
            name: Set("b".to_owned()),
            description: Set("B".to_owned()),
            type_id: Set(configuration_type_id + 1),
            optional: Set(false),
            allows_multiple: Set(false),
            allows_user_override: Set(false),
            ..Default::default()
        })
        .exec(&connection)
        .await
        .is_err()
    );

    // Select the one entry
    assert_eq!(
        configuration_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![configuration_reference::Model {
            id: first_inserted_id,
            name: "a".to_owned(),
            description: "A".to_owned(),
            type_id: configuration_type_id,
            optional: false,
            allows_multiple: false,
            allows_user_override: false,
        }]
    );

    // Insert another unique entry
    assert_eq!(
        configuration_reference::Entity::insert(configuration_reference::ActiveModel {
            name: Set("b".to_owned()),
            description: Set("B".to_owned()),
            type_id: Set(configuration_type_id),
            optional: Set(false),
            allows_multiple: Set(false),
            allows_user_override: Set(false),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id,
        first_inserted_id + 3
    );

    // Select both entries
    assert_eq!(
        configuration_reference::Entity::find()
            .all(&connection)
            .await?,
        vec![
            configuration_reference::Model {
                id: first_inserted_id,
                name: "a".to_owned(),
                description: "A".to_owned(),
                type_id: configuration_type_id,
                optional: false,
                allows_multiple: false,
                allows_user_override: false,
            },
            configuration_reference::Model {
                id: first_inserted_id + 3,
                name: "b".to_owned(),
                description: "B".to_owned(),
                type_id: configuration_type_id,
                optional: false,
                allows_multiple: false,
                allows_user_override: false,
            }
        ]
    );

    // Select one entry by id
    assert_eq!(
        configuration_reference::Entity::find_by_id(first_inserted_id)
            .all(&connection)
            .await?,
        vec![configuration_reference::Model {
            id: first_inserted_id,
            name: "a".to_owned(),
            description: "A".to_owned(),
            type_id: configuration_type_id,
            optional: false,
            allows_multiple: false,
            allows_user_override: false,
        }]
    );

    Ok(())
}
