
import * as proto from "../../../../massa-abi-metaproject/massa-proto-as/assembly";

export namespace env {
  @external("massa", "abi_get_current_slot")
  export declare function abi_get_current_slot(arg: ArrayBuffer): ArrayBuffer;
  @external("massa", "abi_abort")
  export declare function abi_abort(arg: i32): i32;

  export function get_current_slot(): proto.Slot {
    const req = new proto.GetCurrentSlotRequest();
    const req_bytes = proto.encodeGetCurrentSlotRequest(req);
    const resp_bytes = Uint8Array.wrap(
      abi_get_current_slot(encode_length_prefixed(req_bytes).buffer)
    );
    const resp = proto.decodeAbiResponse(resp_bytes);
    assert(resp.error === null);
    assert(resp.res !== null);
    assert(resp.res!.getCurrentSlotResult !== null);
    return assert(
      resp.res!.getCurrentSlotResult!.slot,
      "Could not get current slot"
    );
  }

  export function myabort(
    message: string | null,
    fileName: string | null,
    lineNumber: i32,
    columnNumber: i32
  ): void {
    // 0: len
    // 4: buf...
    const lenPtr: usize = 0;
    const bufPtr: usize = lenPtr + sizeof<usize>();
    var ptr = bufPtr;
  
    store<u64>(ptr, 0x203a74726f6261);
    ptr += 7; // 'abort: '
  
    if (message != null) {
      ptr += String.UTF8.encodeUnsafe(
        changetype<usize>(message),
        message.length,
        ptr
      );
    }
    store<u32>(ptr, 0x206e6920);
    ptr += 4; // ' in '
    if (fileName != null) {
      ptr += String.UTF8.encodeUnsafe(
        changetype<usize>(fileName),
        fileName.length,
        ptr
      );
    }
  
    store<u8>(ptr++, 0x28); // (
  
    var len = decimalCount32(lineNumber);
    ptr += len;
    do {
      let t = lineNumber / 10;
      store<u8>(--ptr, 0x30 + (lineNumber % 10));
      lineNumber = t;
    } while (lineNumber);
    ptr += len;
  
    store<u8>(ptr++, 0x3a); // :
  
    len = decimalCount32(columnNumber);
    ptr += len;
    do {
      let t = columnNumber / 10;
      store<u8>(--ptr, 0x30 + (columnNumber % 10));
      columnNumber = t;
    } while (columnNumber);
    ptr += len;
  
    store<u8>(ptr, 0x29);
    ptr++; // )
  
    const msgLen = ptr - bufPtr;
    store<u8>(lenPtr, msgLen & 0xff);
    store<u8>(lenPtr + 1, (msgLen >> 8) & 0xff);
    store<u8>(lenPtr + 2, (msgLen >> 16) & 0xff);
    store<u8>(lenPtr + 3, (msgLen >> 24) & 0xff);
  
    abi_abort(changetype<i32>(lenPtr));
  
    unreachable();
  }

  export function encode_length_prefixed(data: Uint8Array): Uint8Array {
    const len: i32 = data.length;
    const result = new Uint8Array(4 + len);
    result[0] = len & 0xff;
    result[1] = (len >> 8) & 0xff;
    result[2] = (len >> 16) & 0xff;
    result[3] = (len >> 24) & 0xff;
    result.set(data, 4);
    return result;
  }
}
