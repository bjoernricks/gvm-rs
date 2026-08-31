// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{
    AuthAlgorithm, CertificateTimeStatus, Credential, CredentialFormat, CredentialType,
    GetCredentialsRequest, GetCredentialsResponse, PackageFormat, PrivacyAlgorithm,
};
use crate::commands::entity::KeywordRelation;

fn sample_credential_xml(credential_type: &str) -> String {
    format!(
        "<credential id=\"c33864a9-d3fd-44b3-8717-972bfb01dfcf\"><owner><name>admin</name></owner><name>bob</name><comment>Bob on the web server.</comment><creation_time>2026-04-08T07:15:09Z</creation_time><modification_time>2026-04-08T07:15:09Z</modification_time><writable>1</writable><in_use>0</in_use><permissions><permission><name>Everything</name></permission></permissions><allow_insecure>0</allow_insecure><login>bob</login><type>{credential_type}</type><full_type>username + password</full_type><formats><format>exe</format></formats></credential>"
    )
}

fn sample_credential_xml_prefix() -> String {
    sample_credential_xml("up")
        .trim_end_matches("</credential>")
        .to_string()
}

#[test]
fn serialize_get_credentials_request() {
    let request = GetCredentialsRequest::new();

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize get_credentials request");

    assert_eq!(xml, "<get_credentials/>");
}

#[test]
fn serialize_get_credentials_request_with_options() {
    let request = GetCredentialsRequest::new()
        .with_credential_id("c33864a9-d3fd-44b3-8717-972bfb01dfcf")
        .with_filter("rows=10")
        .with_filter_id("abc")
        .with_details()
        .with_scanners()
        .with_trash()
        .with_targets()
        .with_oci_image_targets()
        .with_format(CredentialFormat::Deb);

    let xml = quick_xml::se::to_string(&request)
        .expect("failed to serialize get_credentials request with options");

    assert_eq!(
        xml,
        "<get_credentials credential_id=\"c33864a9-d3fd-44b3-8717-972bfb01dfcf\" filter=\"rows=10\" filt_id=\"abc\" details=\"1\" scanners=\"1\" trash=\"1\" targets=\"1\" oci_image_targets=\"1\" format=\"deb\"/>"
    );
}

#[test]
fn deserialize_get_credentials_response_filter() {
    let xml = format!(
        "<get_credentials_response status=\"200\" status_text=\"OK\">{}<filters id=\"\"><term>first=1 rows=10 sort=name</term><keywords><keyword><column>first</column><relation>=</relation><value>1</value></keyword><keyword><column>rows</column><relation>=</relation><value>10</value></keyword></keywords></filters></get_credentials_response>",
        sample_credential_xml("up")
    );

    let response: GetCredentialsResponse =
        quick_xml::de::from_str(&xml).expect("failed to deserialize get_credentials_response");

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.filter.id, None);
    assert_eq!(response.filter.name, None);
    assert_eq!(response.filter.term, "first=1 rows=10 sort=name");
    assert_eq!(response.filter.keywords.len(), 2);
    assert_eq!(response.filter.keywords[0].column, "first");
    assert_eq!(response.filter.keywords[0].relation, KeywordRelation::Eq);
    assert_eq!(response.filter.keywords[0].value, "1");

    assert_eq!(response.credential.len(), 1);
    let credential = &response.credential[0];
    assert_eq!(credential.name, "bob");
    assert_eq!(credential.login, "bob");
    assert_eq!(credential.owner.name, "admin");
    assert!(credential.writable);
    assert!(!credential.in_use);
    assert!(!credential.allow_insecure);
    assert_eq!(credential.permissions.len(), 1);
    assert_eq!(credential.permissions[0].name, "Everything");
    assert!(matches!(credential.credential_type, CredentialType::Up));
    assert_eq!(credential.full_type, "username + password");
    assert_eq!(credential.formats, vec![CredentialFormat::Exe]);
    assert!(credential.scanners.is_empty());
    assert!(credential.targets.is_empty());
    assert!(credential.oci_image_targets.is_empty());
    assert!(credential.kdcs.is_empty());
    assert!(credential.public_key.is_none());
    assert!(credential.package.is_none());
    assert!(credential.certificate.is_none());
}

#[test]
fn deserialize_get_credentials_response_collection_counts() {
    let xml = format!(
        "<get_credentials_response status=\"200\" status_text=\"OK\">{}<filters id=\"\"><term></term></filters><credentials start=\"7\" max=\"42\"/><credential_count>9<filtered>5</filtered><page>2</page></credential_count></get_credentials_response>",
        sample_credential_xml("up")
    );

    let response: GetCredentialsResponse =
        quick_xml::de::from_str(&xml).expect("failed to deserialize get_credentials_response");

    use crate::commands::entity::CollectionCounts;
    assert_eq!(
        *response.counts,
        CollectionCounts {
            first: 7,
            rows: 42,
            all: 9,
            filtered: 5,
            length: 2
        }
    );
}

#[test]
fn deserialize_credential_with_unknown_type() {
    let xml = sample_credential_xml("unexpected");

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    assert!(matches!(
        credential.credential_type,
        CredentialType::Unknown
    ));
}

#[test]
fn deserialize_credential_with_snmp_algorithms() {
    let xml = format!(
        "{}<auth_algorithm>sha1</auth_algorithm><privacy><algorithm>aes</algorithm></privacy></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    assert!(matches!(
        credential.auth_algorithm,
        Some(AuthAlgorithm::Sha1)
    ));
    let privacy = credential.privacy.expect("expected privacy to be present");
    assert!(matches!(privacy.algorithm, PrivacyAlgorithm::Aes));
}

#[test]
fn deserialize_credential_with_empty_privacy_algorithm() {
    let xml = format!(
        "{}<privacy><algorithm></algorithm></privacy></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    let privacy = credential.privacy.expect("expected privacy to be present");
    assert!(matches!(privacy.algorithm, PrivacyAlgorithm::Unknown));
}

#[test]
fn deserialize_credential_with_targets_scanners_and_public_key() {
    let xml = format!(
        "{}<scanners><scanner id=\"08b69003-5fc2-4037-a479-93b440211c73\"><name>OpenVAS Default</name></scanner></scanners><targets><target id=\"1f28d970-17ef-4c69-ba8a-13827059f2b9\"><name>Web server</name></target></targets><oci_image_targets><oci_image_target id=\"3db527c4-c3eb-41d8-b0e8-3f9752ac67f4\"><name>Image</name></oci_image_target></oci_image_targets><public_key>ssh-rsa AAAAB3...Z64IcQ== Key generated by GVM</public_key></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    assert_eq!(credential.scanners.len(), 1);
    assert_eq!(credential.scanners[0].name, "OpenVAS Default");
    assert_eq!(credential.targets.len(), 1);
    assert_eq!(credential.targets[0].name, "Web server");
    assert_eq!(
        credential.targets[0].id,
        Some(
            "1f28d970-17ef-4c69-ba8a-13827059f2b9"
                .parse::<uuid::Uuid>()
                .expect("invalid test uuid")
        )
    );
    assert_eq!(credential.oci_image_targets.len(), 1);
    assert_eq!(credential.oci_image_targets[0].name, "Image");
    assert_eq!(
        credential.public_key,
        Some("ssh-rsa AAAAB3...Z64IcQ== Key generated by GVM".to_string())
    );
}

#[test]
fn deserialize_credential_with_package() {
    let xml = format!(
        "{}<package format=\"deb\">ITxhcmNoPgpk...DmvF0AKAAACg==</package></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    let package = credential.package.expect("expected package to be present");
    assert_eq!(package.format, PackageFormat::Deb);
    assert_eq!(package.data, "ITxhcmNoPgpk...DmvF0AKAAACg==");
}

#[test]
fn deserialize_credential_with_key_infos() {
    let xml = format!(
        "{}<private_key_info><type>RSA</type><sha256_hash>abcd</sha256_hash></private_key_info><public_key_info><fingerprint>SHA256:1234</fingerprint></public_key_info></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    let private_key_info = credential
        .private_key_info
        .expect("expected private key info to be present");
    assert_eq!(private_key_info.key_type, Some("RSA".to_string()));
    assert_eq!(private_key_info.sha256_hash, Some("abcd".to_string()));

    let public_key_info = credential
        .public_key_info
        .expect("expected public key info to be present");
    assert_eq!(public_key_info.fingerprint, Some("SHA256:1234".to_string()));
}

#[test]
fn deserialize_credential_with_certificate_info() {
    let xml = format!(
        "{}<certificate_info><time_status>valid</time_status><activation_time>2026-04-08T07:15:09Z</activation_time><expiration_time>unlimited</expiration_time><issuer>CN=Example</issuer><md5_fingerprint>11:22</md5_fingerprint><sha256_fingerprint>33:44</sha256_fingerprint><subject>CN=Client</subject><serial>01</serial></certificate_info></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    let certificate_info = credential
        .certificate_info
        .expect("expected certificate info to be present");
    assert!(matches!(
        certificate_info.time_status,
        CertificateTimeStatus::Valid
    ));
    assert_eq!(
        certificate_info.activation_time,
        Some(
            "2026-04-08T07:15:09Z"
                .parse::<chrono::DateTime<chrono::Utc>>()
                .expect("invalid test time")
        )
    );
    assert_eq!(certificate_info.expiration_time, None);
    assert_eq!(certificate_info.issuer, "CN=Example");
    assert_eq!(certificate_info.md5_fingerprint, "11:22");
    assert_eq!(
        certificate_info.sha256_fingerprint,
        Some("33:44".to_string())
    );
    assert_eq!(certificate_info.subject, Some("CN=Client".to_string()));
    assert_eq!(certificate_info.serial, Some("01".to_string()));
}

#[test]
fn deserialize_krb5_credential() {
    let xml = format!(
        "{}<kdc>kdc.example.com</kdc><kdcs><kdc>kdc1.example.com</kdc><kdc>kdc2.example.com</kdc></kdcs><realm>EXAMPLE.COM</realm></credential>",
        sample_credential_xml("krb5").trim_end_matches("</credential>")
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    assert!(matches!(credential.credential_type, CredentialType::Krb5));
    assert_eq!(credential.kdc, Some("kdc.example.com".to_string()));
    assert_eq!(
        credential.kdcs,
        vec![
            "kdc1.example.com".to_string(),
            "kdc2.example.com".to_string()
        ]
    );
    assert_eq!(credential.realm, Some("EXAMPLE.COM".to_string()));
}

#[test]
fn deserialize_credential_with_user_tags() {
    let xml = format!(
        "{}<user_tags><count>1</count><tags id=\"3db527c4-c3eb-41d8-b0e8-3f9752ac67f4\"><name>env</name><value>prod</value><comment>production systems</comment></tags></user_tags></credential>",
        sample_credential_xml_prefix()
    );

    let credential: Credential =
        quick_xml::de::from_str(&xml).expect("failed to deserialize credential");

    let user_tags = credential
        .user_tags
        .expect("expected user tags to be present");
    assert_eq!(user_tags.count, 1);
    assert_eq!(user_tags.tags.len(), 1);
    assert_eq!(user_tags.tags[0].name, "env");
}
