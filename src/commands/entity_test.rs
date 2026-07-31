// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{Entity, HasId, UserTags};

#[test]
fn deserialize_entity_with_uuid_id() {
    let xml = r#"<Entity id="3db527c4-c3eb-41d8-b0e8-3f9752ac67f4"><name>Localhost</name><trash>0</trash></Entity>"#;

    let entity: Entity = quick_xml::de::from_str(xml).expect("failed to deserialize entity");

    assert_eq!(
        entity.id,
        Some(
            "3db527c4-c3eb-41d8-b0e8-3f9752ac67f4"
                .parse()
                .expect("invalid test uuid")
        )
    );
    assert!(entity.has_id());
    assert_eq!(entity.name, "Localhost");
    assert!(!entity.trash);
    assert_eq!(entity.permissions, None);
}

#[test]
fn deserialize_entity_with_empty_id_and_default_trash() {
    let xml = r#"<Entity id=""><name>NoId</name></Entity>"#;

    let entity: Entity = quick_xml::de::from_str(xml).expect("failed to deserialize entity");

    assert_eq!(entity.id, None);
    assert!(!entity.has_id());
    assert_eq!(entity.name, "NoId");
    assert!(!entity.trash);
}

#[test]
fn deserialize_entity_with_trash_set() {
    let xml = r#"<Entity id=""><name>Trashed</name><trash>1</trash></Entity>"#;

    let entity: Entity = quick_xml::de::from_str(xml).expect("failed to deserialize entity");

    assert_eq!(entity.id, None);
    assert!(entity.trash);
}

#[test]
fn deserialize_entity_with_permissions() {
    let xml = r#"<Entity id=""><name>WithPermissions</name><permissions>read</permissions><permissions>write</permissions></Entity>"#;

    let entity: Entity = quick_xml::de::from_str(xml).expect("failed to deserialize entity");

    assert_eq!(entity.name, "WithPermissions");
    assert_eq!(
        entity.permissions,
        Some(vec!["read".to_string(), "write".to_string()])
    );
}

#[test]
fn deserialize_user_tags_without_tags_defaults_to_empty_list() {
    let xml = r#"<UserTags><count>0</count></UserTags>"#;

    let user_tags: UserTags =
        quick_xml::de::from_str(xml).expect("failed to deserialize user tags");

    assert_eq!(user_tags.count, 0);
    assert!(user_tags.tags.is_empty());
}

#[test]
fn deserialize_user_tags_with_tag_entries() {
    let xml = r#"<UserTags><count>1</count><tags id="3db527c4-c3eb-41d8-b0e8-3f9752ac67f4"><name>env</name><value>prod</value><comment>production systems</comment></tags></UserTags>"#;

    let user_tags: UserTags =
        quick_xml::de::from_str(xml).expect("failed to deserialize user tags");

    assert_eq!(user_tags.count, 1);
    assert_eq!(user_tags.tags.len(), 1);

    let tag = &user_tags.tags[0];
    assert_eq!(
        tag.id,
        "3db527c4-c3eb-41d8-b0e8-3f9752ac67f4"
            .parse::<uuid::Uuid>()
            .expect("invalid test uuid")
    );
    assert_eq!(tag.name, "env");
    assert_eq!(tag.value, "prod");
    assert_eq!(tag.comment, "production systems");
}
