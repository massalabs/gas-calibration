pub fn write_sc_wasmv1(calls: &[String]) -> String {
    format!(
        "import {{ decimalCount32 }} from 'util/number';
import {{ env }} from '../../wasmv1/env_wasmv1';
import {{ toBytes }} from '../../wasmv1/helpers';

let shared_mem: ArrayBuffer = new ArrayBuffer(0);

export function myabort(
  message: string | null,
  fileName: string | null,
  lineNumber: i32,
  columnNumber: i32
): void {{
  const lenPtr: usize = 0;
  const bufPtr: usize = lenPtr + sizeof<usize>();
  var ptr = bufPtr;

  store<u64>(ptr, 0x203a74726f6261);
  ptr += 7; // 'abort: '

  if (message != null) {{
    ptr += String.UTF8.encodeUnsafe(
      changetype<usize>(message),
      message.length,
      ptr
    );
  }}
  store<u32>(ptr, 0x206e6920);
  ptr += 4; // ' in '
  if (fileName != null) {{
    ptr += String.UTF8.encodeUnsafe(
      changetype<usize>(fileName),
      fileName.length,
      ptr
    );
  }}

  store<u8>(ptr++, 0x28); // (

  var len = decimalCount32(lineNumber);
  ptr += len;
  do {{
    let t = lineNumber / 10;
    store<u8>(--ptr, 0x30 + (lineNumber % 10));
    lineNumber = t;
  }} while (lineNumber);
  ptr += len;

  store<u8>(ptr++, 0x3a); // :

  len = decimalCount32(columnNumber);
  ptr += len;
  do {{
    let t = columnNumber / 10;
    store<u8>(--ptr, 0x30 + (columnNumber % 10));
    columnNumber = t;
  }} while (columnNumber);
  ptr += len;

  store<u8>(ptr, 0x29);
  ptr++; // )

  const msgLen = ptr - bufPtr;
  store<u8>(lenPtr, msgLen & 0xff);
  store<u8>(lenPtr + 1, (msgLen >> 8) & 0xff);
  store<u8>(lenPtr + 2, (msgLen >> 16) & 0xff);
  store<u8>(lenPtr + 3, (msgLen >> 24) & 0xff);

  env.abi_abort(changetype<i32>(lenPtr));

  unreachable();
}}

export function __alloc(size: i32): ArrayBuffer {{
  shared_mem = new ArrayBuffer(size);
  return shared_mem;
}}

export function main(_args: ArrayBuffer): ArrayBuffer {{
{}
  shared_mem = env.encode_length_prefixed(new Uint8Array(0)).buffer;
  return shared_mem;
}}",
        calls.join("\n")
    )
}
