/**
 * Helper function to transform a string to a Uint8Array
 * @param {string} str
 * @return {Uint8Array}
 */
export function toBytes(str: string): Uint8Array {
    let arr = Uint8Array.wrap(String.UTF8.encode(str));
    return arr;
}
