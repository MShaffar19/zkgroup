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
import org.signal.zkgroup.internal.ByteArray;
import org.signal.zkgroup.internal.Native;

public final class ProfileKey extends ByteArray {

  public static final int SIZE = 16;

  public ProfileKey(byte[] contents) throws InvalidInputException {
    super(contents, SIZE);
  }

  public ProfileKeyCommitment getCommitment() {
    byte[] newContents = new byte[ProfileKeyCommitment.SIZE];

    int ffi_return = Native.profileKeyGetCommitmentJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    try {
      return new ProfileKeyCommitment(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }

  }

  public ProfileKeyVersion getProfileKeyVersion() {
    byte[] newContents = new byte[ProfileKeyVersion.SIZE];

    int ffi_return = Native.profileKeyGetProfileKeyVersionJNI(contents, newContents);

    if (ffi_return != Native.FFI_RETURN_OK) {
      throw new ZkGroupError("FFI_RETURN!=OK");
    }

    try {
      return new ProfileKeyVersion(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }

  }

  public byte[] serialize() {
    return contents.clone();
  }

}
