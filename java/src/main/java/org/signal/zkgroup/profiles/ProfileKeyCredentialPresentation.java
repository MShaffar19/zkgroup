//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

// Generated by zkgroup/codegen/codegen.py - do not edit

package org.signal.zkgroup.profiles;

import org.signal.zkgroup.InvalidInputException;
import org.signal.zkgroup.ZkGroupError;
import org.signal.zkgroup.groups.ProfileKeyCiphertext;
import org.signal.zkgroup.groups.UuidCiphertext;
import org.signal.zkgroup.internal.ByteArray;
import org.signal.zkgroup.internal.Native;

public final class ProfileKeyCredentialPresentation extends ByteArray {

  public static final int SIZE = 936;

  public ProfileKeyCredentialPresentation(byte[] contents) throws InvalidInputException {
    super(contents, SIZE);
    
    int ffi_return = Native.profileKeyCredentialPresentationCheckValidContentsJNI(contents);

    if (ffi_return == Native.FFI_RETURN_INPUT_ERROR) {
      throw new InvalidInputException("FFI_RETURN_INPUT_ERROR");
    }

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }
  }

  public UuidCiphertext getUuidCiphertext() {
    byte[] newContents = new byte[UuidCiphertext.SIZE];

    int ffi_return = Native.profileKeyCredentialPresentationGetUuidCiphertextJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    try {
      return new UuidCiphertext(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }

  }

  public ProfileKeyCiphertext getProfileKeyCiphertext() {
    byte[] newContents = new byte[ProfileKeyCiphertext.SIZE];

    int ffi_return = Native.profileKeyCredentialPresentationGetProfileKeyCiphertextJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    try {
      return new ProfileKeyCiphertext(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }

  }

  public byte[] serialize() {
    return contents.clone();
  }

}
