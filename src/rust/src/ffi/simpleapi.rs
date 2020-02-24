//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

// Generated by zkgroup/codegen/codegen.py - do not edit

#![allow(non_snake_case)]

use crate::api;
use crate::common::constants::*;
use crate::common::simple_types;
use crate::ffi::constants::*;

pub fn ProfileKey_getCommitment(profileKeyIn: &[u8], profileKeyCommitmentOut: &mut [u8]) -> i32 {
    let profile_key: api::profiles::ProfileKey = match bincode::deserialize(profileKeyIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INTERNAL_ERROR,
    };
    let profile_key_commitment = profile_key.get_commitment();
    profileKeyCommitmentOut.copy_from_slice(&bincode::serialize(&profile_key_commitment).unwrap());
    FFI_RETURN_OK
}

pub fn ProfileKey_getProfileKeyVersion(
    profileKeyIn: &[u8],
    profileKeyVersionOut: &mut [u8],
) -> i32 {
    let profile_key: api::profiles::ProfileKey = match bincode::deserialize(profileKeyIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INTERNAL_ERROR,
    };
    let profile_key_version = profile_key.get_profile_key_version();
    profileKeyVersionOut.copy_from_slice(&bincode::serialize(&profile_key_version).unwrap());
    FFI_RETURN_OK
}

pub fn ProfileKeyCommitment_checkValidContents(profileKeyCommitmentIn: &[u8]) -> i32 {
    let _: api::profiles::ProfileKeyCommitment = match bincode::deserialize(profileKeyCommitmentIn)
    {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn ProfileKeyCommitment_getProfileKeyVersion(
    profileKeyCommitmentIn: &[u8],
    profileKeyVersionOut: &mut [u8],
) -> i32 {
    let profile_key_commitment: api::profiles::ProfileKeyCommitment =
        match bincode::deserialize(profileKeyCommitmentIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let profile_key_version = profile_key_commitment.get_profile_key_version();
    profileKeyVersionOut.copy_from_slice(&bincode::serialize(&profile_key_version).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_generateDeterministic(
    randomnessIn: &[u8],
    groupSecretParamsOut: &mut [u8],
) -> i32 {
    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let group_secret_params = api::groups::GroupSecretParams::generate(randomness);
    groupSecretParamsOut.copy_from_slice(&bincode::serialize(&group_secret_params).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_deriveFromMasterKey(
    groupMasterKeyIn: &[u8],
    groupSecretParamsOut: &mut [u8],
) -> i32 {
    let group_master_key: api::groups::GroupMasterKey = match bincode::deserialize(groupMasterKeyIn)
    {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let group_secret_params =
        api::groups::GroupSecretParams::derive_from_master_key(group_master_key);
    groupSecretParamsOut.copy_from_slice(&bincode::serialize(&group_secret_params).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_checkValidContents(groupSecretParamsIn: &[u8]) -> i32 {
    let _: api::groups::GroupSecretParams = match bincode::deserialize(groupSecretParamsIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn GroupSecretParams_getMasterKey(
    groupSecretParamsIn: &[u8],
    groupMasterKeyOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let group_master_key = group_secret_params.get_master_key();
    groupMasterKeyOut.copy_from_slice(&bincode::serialize(&group_master_key).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_getPublicParams(
    groupSecretParamsIn: &[u8],
    groupPublicParamsOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let group_public_params = group_secret_params.get_public_params();
    groupPublicParamsOut.copy_from_slice(&bincode::serialize(&group_public_params).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_signDeterministic(
    groupSecretParamsIn: &[u8],
    randomnessIn: &[u8],
    messageIn: &[u8],
    changeSignatureOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let message = messageIn;
    let change_signature = match group_secret_params.sign(randomness, message) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    changeSignatureOut.copy_from_slice(&change_signature);
    FFI_RETURN_OK
}

pub fn GroupSecretParams_encryptUuid(
    groupSecretParamsIn: &[u8],
    uuidIn: &[u8],
    uuidCiphertextOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let uuid: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let uuid_ciphertext = group_secret_params.encrypt_uuid(uuid);
    uuidCiphertextOut.copy_from_slice(&bincode::serialize(&uuid_ciphertext).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_decryptUuid(
    groupSecretParamsIn: &[u8],
    uuidCiphertextIn: &[u8],
    uuidOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let uuid_ciphertext: api::groups::UuidCiphertext = match bincode::deserialize(uuidCiphertextIn)
    {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let uuid = match group_secret_params.decrypt_uuid(uuid_ciphertext) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    uuidOut.copy_from_slice(&bincode::serialize(&uuid).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_encryptProfileKeyDeterministic(
    groupSecretParamsIn: &[u8],
    randomnessIn: &[u8],
    profileKeyIn: &[u8],
    profileKeyCiphertextOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let profile_key: api::profiles::ProfileKey = match bincode::deserialize(profileKeyIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let profile_key_ciphertext = group_secret_params.encrypt_profile_key(randomness, profile_key);
    profileKeyCiphertextOut.copy_from_slice(&bincode::serialize(&profile_key_ciphertext).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_decryptProfileKey(
    groupSecretParamsIn: &[u8],
    profileKeyCiphertextIn: &[u8],
    profileKeyOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let profile_key_ciphertext: api::groups::ProfileKeyCiphertext =
        match bincode::deserialize(profileKeyCiphertextIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    let profile_key = match group_secret_params.decrypt_profile_key(profile_key_ciphertext) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    profileKeyOut.copy_from_slice(&bincode::serialize(&profile_key).unwrap());
    FFI_RETURN_OK
}

pub fn GroupSecretParams_encryptBlob(
    groupSecretParamsIn: &[u8],
    plaintextIn: &[u8],
    blobCiphertextOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let plaintext = plaintextIn;
    let blob_ciphertext = group_secret_params.encrypt_blob(plaintext);
    blobCiphertextOut.copy_from_slice(&blob_ciphertext);
    FFI_RETURN_OK
}

pub fn GroupSecretParams_decryptBlob(
    groupSecretParamsIn: &[u8],
    blobCiphertextIn: &[u8],
    plaintextOut: &mut [u8],
) -> i32 {
    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let blob_ciphertext = blobCiphertextIn;
    let plaintext = match group_secret_params.decrypt_blob(blob_ciphertext) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    plaintextOut.copy_from_slice(&plaintext);
    FFI_RETURN_OK
}

pub fn ServerSecretParams_generateDeterministic(
    randomnessIn: &[u8],
    serverSecretParamsOut: &mut [u8],
) -> i32 {
    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let server_secret_params = api::ServerSecretParams::generate(randomness);
    serverSecretParamsOut.copy_from_slice(&bincode::serialize(&server_secret_params).unwrap());
    FFI_RETURN_OK
}

pub fn ServerSecretParams_checkValidContents(serverSecretParamsIn: &[u8]) -> i32 {
    let _: api::ServerSecretParams = match bincode::deserialize(serverSecretParamsIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn ServerSecretParams_getPublicParams(
    serverSecretParamsIn: &[u8],
    serverPublicParamsOut: &mut [u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let server_public_params = server_secret_params.get_public_params();
    serverPublicParamsOut.copy_from_slice(&bincode::serialize(&server_public_params).unwrap());
    FFI_RETURN_OK
}

pub fn ServerSecretParams_signDeterministic(
    serverSecretParamsIn: &[u8],
    randomnessIn: &[u8],
    messageIn: &[u8],
    notarySignatureOut: &mut [u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let message = messageIn;
    let notary_signature = match server_secret_params.sign(randomness, message) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    notarySignatureOut.copy_from_slice(&notary_signature);
    FFI_RETURN_OK
}

pub fn ServerPublicParams_receiveAuthCredential(
    serverPublicParamsIn: &[u8],
    uuidIn: &[u8],
    redemptionTimeIn: u32,
    authCredentialResponseIn: &[u8],
    authCredentialOut: &mut [u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let uuid: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let redemption_time = redemptionTimeIn;

    let auth_credential_response: api::auth::AuthCredentialResponse =
        match bincode::deserialize(authCredentialResponseIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    let auth_credential = match server_public_params.receive_auth_credential(
        uuid,
        redemption_time,
        &auth_credential_response,
    ) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    authCredentialOut.copy_from_slice(&bincode::serialize(&auth_credential).unwrap());
    FFI_RETURN_OK
}

pub fn ServerPublicParams_createAuthCredentialPresentationDeterministic(
    serverPublicParamsIn: &[u8],
    randomnessIn: &[u8],
    groupSecretParamsIn: &[u8],
    authCredentialIn: &[u8],
    authCredentialPresentationOut: &mut [u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let auth_credential: api::auth::AuthCredential = match bincode::deserialize(authCredentialIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let auth_credential_presentation = server_public_params.create_auth_credential_presentation(
        randomness,
        group_secret_params,
        auth_credential,
    );
    authCredentialPresentationOut
        .copy_from_slice(&bincode::serialize(&auth_credential_presentation).unwrap());
    FFI_RETURN_OK
}

pub fn ServerPublicParams_createProfileKeyCredentialRequestContextDeterministic(
    serverPublicParamsIn: &[u8],
    randomnessIn: &[u8],
    uuidIn: &[u8],
    profileKeyIn: &[u8],
    profileKeyCredentialRequestContextOut: &mut [u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let uuid: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let profile_key: api::profiles::ProfileKey = match bincode::deserialize(profileKeyIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let profile_key_credential_request_context = server_public_params
        .create_profile_key_credential_request_context(randomness, uuid, profile_key);
    profileKeyCredentialRequestContextOut
        .copy_from_slice(&bincode::serialize(&profile_key_credential_request_context).unwrap());
    FFI_RETURN_OK
}

pub fn ServerPublicParams_receiveProfileKeyCredential(
    serverPublicParamsIn: &[u8],
    profileKeyCredentialRequestContextIn: &[u8],
    profileKeyCredentialResponseIn: &[u8],
    profileKeyCredentialOut: &mut [u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let profile_key_credential_request_context: api::profiles::ProfileKeyCredentialRequestContext =
        match bincode::deserialize(profileKeyCredentialRequestContextIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let profile_key_credential_response: api::profiles::ProfileKeyCredentialResponse =
        match bincode::deserialize(profileKeyCredentialResponseIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    let profile_key_credential = match server_public_params.receive_profile_key_credential(
        &profile_key_credential_request_context,
        &profile_key_credential_response,
    ) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    profileKeyCredentialOut.copy_from_slice(&bincode::serialize(&profile_key_credential).unwrap());
    FFI_RETURN_OK
}

pub fn ServerPublicParams_createProfileKeyCredentialPresentationDeterministic(
    serverPublicParamsIn: &[u8],
    randomnessIn: &[u8],
    groupSecretParamsIn: &[u8],
    profileKeyCredentialIn: &[u8],
    profileKeyCredentialPresentationOut: &mut [u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let group_secret_params: api::groups::GroupSecretParams =
        match bincode::deserialize(groupSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let profile_key_credential: api::profiles::ProfileKeyCredential =
        match bincode::deserialize(profileKeyCredentialIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    let profile_key_credential_presentation = server_public_params
        .create_profile_key_credential_presentation(
            randomness,
            group_secret_params,
            profile_key_credential,
        );
    profileKeyCredentialPresentationOut
        .copy_from_slice(&bincode::serialize(&profile_key_credential_presentation).unwrap());
    FFI_RETURN_OK
}

pub fn ServerSecretParams_issueAuthCredentialDeterministic(
    serverSecretParamsIn: &[u8],
    randomnessIn: &[u8],
    uuidIn: &[u8],
    redemptionTimeIn: u32,
    authCredentialResponseOut: &mut [u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let uuid: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    let redemption_time = redemptionTimeIn;
    let auth_credential_response =
        server_secret_params.issue_auth_credential(randomness, uuid, redemption_time);
    authCredentialResponseOut
        .copy_from_slice(&bincode::serialize(&auth_credential_response).unwrap());
    FFI_RETURN_OK
}

pub fn ServerSecretParams_verifyAuthCredentialPresentation(
    serverSecretParamsIn: &[u8],
    groupPublicParamsIn: &[u8],
    authCredentialPresentationIn: &[u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let group_public_params: api::groups::GroupPublicParams =
        match bincode::deserialize(groupPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let auth_credential_presentation: api::auth::AuthCredentialPresentation =
        match bincode::deserialize(authCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    match server_secret_params
        .verify_auth_credential_presentation(group_public_params, &auth_credential_presentation)
    {
        Ok(_) => (),
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    }
    FFI_RETURN_OK
}

pub fn ServerSecretParams_issueProfileKeyCredentialDeterministic(
    serverSecretParamsIn: &[u8],
    randomnessIn: &[u8],
    profileKeyCredentialRequestIn: &[u8],
    uuidIn: &[u8],
    profileKeyCommitmentIn: &[u8],
    profileKeyCredentialResponseOut: &mut [u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let randomness: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let profile_key_credential_request: api::profiles::ProfileKeyCredentialRequest =
        match bincode::deserialize(profileKeyCredentialRequestIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let uuid: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    let profile_key_commitment: api::profiles::ProfileKeyCommitment =
        match bincode::deserialize(profileKeyCommitmentIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    let profile_key_credential_response = match server_secret_params.issue_profile_key_credential(
        randomness,
        &profile_key_credential_request,
        uuid,
        profile_key_commitment,
    ) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };
    profileKeyCredentialResponseOut
        .copy_from_slice(&bincode::serialize(&profile_key_credential_response).unwrap());
    FFI_RETURN_OK
}

pub fn ServerSecretParams_verifyProfileKeyCredentialPresentation(
    serverSecretParamsIn: &[u8],
    groupPublicParamsIn: &[u8],
    profileKeyCredentialPresentationIn: &[u8],
) -> i32 {
    let server_secret_params: api::ServerSecretParams =
        match bincode::deserialize(serverSecretParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };

    let group_public_params: api::groups::GroupPublicParams =
        match bincode::deserialize(groupPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    let profile_key_credential_presentation: api::profiles::ProfileKeyCredentialPresentation =
        match bincode::deserialize(profileKeyCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };
    match server_secret_params.verify_profile_key_credential_presentation(
        group_public_params,
        &profile_key_credential_presentation,
    ) {
        Ok(_) => (),
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    }
    FFI_RETURN_OK
}

pub fn GroupPublicParams_checkValidContents(groupPublicParamsIn: &[u8]) -> i32 {
    let _: api::groups::GroupPublicParams = match bincode::deserialize(groupPublicParamsIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn GroupPublicParams_getGroupIdentifier(
    groupPublicParamsIn: &[u8],
    groupIdentifierOut: &mut [u8],
) -> i32 {
    let group_public_params: api::groups::GroupPublicParams =
        match bincode::deserialize(groupPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let group_identifier = group_public_params.get_group_identifier();
    groupIdentifierOut.copy_from_slice(&bincode::serialize(&group_identifier).unwrap());
    FFI_RETURN_OK
}

pub fn GroupPublicParams_verifySignature(
    groupPublicParamsIn: &[u8],
    messageIn: &[u8],
    changeSignatureIn: &[u8],
) -> i32 {
    let group_public_params: api::groups::GroupPublicParams =
        match bincode::deserialize(groupPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let message = messageIn;
    let mut change_signature: simple_types::ChangeSignatureBytes = [0u8; SIGNATURE_LEN];
    change_signature.copy_from_slice(changeSignatureIn);
    match group_public_params.verify_signature(message, change_signature) {
        Ok(_) => (),
        _ => return FFI_RETURN_INPUT_ERROR,
    };
    FFI_RETURN_OK
}

pub fn ServerPublicParams_checkValidContents(serverPublicParamsIn: &[u8]) -> i32 {
    let _: api::ServerPublicParams = match bincode::deserialize(serverPublicParamsIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn ServerPublicParams_verifySignature(
    serverPublicParamsIn: &[u8],
    messageIn: &[u8],
    notarySignatureIn: &[u8],
) -> i32 {
    let server_public_params: api::ServerPublicParams =
        match bincode::deserialize(serverPublicParamsIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let message = messageIn;
    let mut notary_signature: simple_types::NotarySignatureBytes = [0u8; SIGNATURE_LEN];
    notary_signature.copy_from_slice(notarySignatureIn);
    match server_public_params.verify_signature(message, notary_signature) {
        Ok(_) => (),
        _ => return FFI_RETURN_INPUT_ERROR,
    };
    FFI_RETURN_OK
}

pub fn AuthCredentialResponse_checkValidContents(authCredentialResponseIn: &[u8]) -> i32 {
    let _: api::auth::AuthCredentialResponse = match bincode::deserialize(authCredentialResponseIn)
    {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn AuthCredential_checkValidContents(authCredentialIn: &[u8]) -> i32 {
    let _: api::auth::AuthCredential = match bincode::deserialize(authCredentialIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn AuthCredentialPresentation_checkValidContents(authCredentialPresentationIn: &[u8]) -> i32 {
    let _: api::auth::AuthCredentialPresentation =
        match bincode::deserialize(authCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    FFI_RETURN_OK
}

pub fn AuthCredentialPresentation_getUuidCiphertext(
    authCredentialPresentationIn: &[u8],
    uuidCiphertextOut: &mut [u8],
) -> i32 {
    let auth_credential_presentation: api::auth::AuthCredentialPresentation =
        match bincode::deserialize(authCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let uuid_ciphertext = auth_credential_presentation.get_uuid_ciphertext();
    uuidCiphertextOut.copy_from_slice(&bincode::serialize(&uuid_ciphertext).unwrap());
    FFI_RETURN_OK
}

pub fn AuthCredentialPresentation_getRedemptionTime(
    authCredentialPresentationIn: &[u8],
    redemptionTimeOut: &mut [u8],
) -> i32 {
    let auth_credential_presentation: api::auth::AuthCredentialPresentation =
        match bincode::deserialize(authCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let redemption_time = auth_credential_presentation.get_redemption_time();
    redemptionTimeOut.copy_from_slice(&redemption_time.to_be_bytes());
    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialRequestContext_checkValidContents(
    profileKeyCredentialRequestContextIn: &[u8],
) -> i32 {
    let _: api::profiles::ProfileKeyCredentialRequestContext =
        match bincode::deserialize(profileKeyCredentialRequestContextIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialRequestContext_getRequest(
    profileKeyCredentialRequestContextIn: &[u8],
    profileKeyCredentialRequestOut: &mut [u8],
) -> i32 {
    let profile_key_credential_request_context: api::profiles::ProfileKeyCredentialRequestContext =
        match bincode::deserialize(profileKeyCredentialRequestContextIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let profile_key_credential_request = profile_key_credential_request_context.get_request();
    profileKeyCredentialRequestOut
        .copy_from_slice(&bincode::serialize(&profile_key_credential_request).unwrap());
    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialRequest_checkValidContents(profileKeyCredentialRequestIn: &[u8]) -> i32 {
    let _: api::profiles::ProfileKeyCredentialRequest =
        match bincode::deserialize(profileKeyCredentialRequestIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialResponse_checkValidContents(
    profileKeyCredentialResponseIn: &[u8],
) -> i32 {
    let _: api::profiles::ProfileKeyCredentialResponse =
        match bincode::deserialize(profileKeyCredentialResponseIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    FFI_RETURN_OK
}

pub fn ProfileKeyCredential_checkValidContents(profileKeyCredentialIn: &[u8]) -> i32 {
    let _: api::profiles::ProfileKeyCredential = match bincode::deserialize(profileKeyCredentialIn)
    {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialPresentation_checkValidContents(
    profileKeyCredentialPresentationIn: &[u8],
) -> i32 {
    let _: api::profiles::ProfileKeyCredentialPresentation =
        match bincode::deserialize(profileKeyCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INPUT_ERROR,
        };

    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialPresentation_getUuidCiphertext(
    profileKeyCredentialPresentationIn: &[u8],
    uuidCiphertextOut: &mut [u8],
) -> i32 {
    let profile_key_credential_presentation: api::profiles::ProfileKeyCredentialPresentation =
        match bincode::deserialize(profileKeyCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let uuid_ciphertext = profile_key_credential_presentation.get_uuid_ciphertext();
    uuidCiphertextOut.copy_from_slice(&bincode::serialize(&uuid_ciphertext).unwrap());
    FFI_RETURN_OK
}

pub fn ProfileKeyCredentialPresentation_getProfileKeyCiphertext(
    profileKeyCredentialPresentationIn: &[u8],
    profileKeyCiphertextOut: &mut [u8],
) -> i32 {
    let profile_key_credential_presentation: api::profiles::ProfileKeyCredentialPresentation =
        match bincode::deserialize(profileKeyCredentialPresentationIn) {
            Ok(result) => result,
            Err(_) => return FFI_RETURN_INTERNAL_ERROR,
        };
    let profile_key_ciphertext = profile_key_credential_presentation.get_profile_key_ciphertext();
    profileKeyCiphertextOut.copy_from_slice(&bincode::serialize(&profile_key_ciphertext).unwrap());
    FFI_RETURN_OK
}

pub fn UuidCiphertext_checkValidContents(uuidCiphertextIn: &[u8]) -> i32 {
    let _: api::groups::UuidCiphertext = match bincode::deserialize(uuidCiphertextIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn ProfileKeyCiphertext_checkValidContents(profileKeyCiphertextIn: &[u8]) -> i32 {
    let _: api::groups::ProfileKeyCiphertext = match bincode::deserialize(profileKeyCiphertextIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn Randomness_checkValidContents(randomnessIn: &[u8]) -> i32 {
    let _: simple_types::RandomnessBytes = match bincode::deserialize(randomnessIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}

pub fn Uuid_checkValidContents(uuidIn: &[u8]) -> i32 {
    let _: simple_types::UidBytes = match bincode::deserialize(uuidIn) {
        Ok(result) => result,
        Err(_) => return FFI_RETURN_INPUT_ERROR,
    };

    FFI_RETURN_OK
}
