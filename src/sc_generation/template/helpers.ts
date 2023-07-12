/**
 * Helper function to transform a string to a Uint8Array
 * @param {string} str
 * @return {Uint8Array}
 */
export function toBytes(str: string): Uint8Array {
    let arr = new Uint8Array(str.length << 1);
    memory.copy(changetype<usize>(arr), changetype<usize>(str), arr.length);
    return arr;
}

/**
 * Helper function to transform a Uint8Array to a string
 * @param {Uint8Array} arr
 * @return {string}
 */
export function fromBytes(arr: Uint8Array): string {
    let str = changetype<string>(__new(arr.length, idof<string>()));
    memory.copy(changetype<usize>(str), changetype<usize>(arr), arr.length);
    return str;
}
