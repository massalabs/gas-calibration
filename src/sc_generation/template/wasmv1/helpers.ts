/**
 * Helper function to transform a string to a Uint8Array
 * @param {string} str
 * @return {Uint8Array}
 */
export function toBytes(str: string): Uint8Array {
    let arr = Uint8Array.wrap(String.UTF8.encode(str));
    return arr;
}

/**
 * Helper function to transform a StaticArray<u8> to a string
 * @param {StaticArray<u8>} arr
 * @return {string}
 */
export function fromBytes(arr: StaticArray<u8>): string {
    let str = changetype<string>(__new(arr.length, idof<string>()));
    memory.copy(changetype<usize>(str), changetype<usize>(arr), arr.length);
    return str;
}
