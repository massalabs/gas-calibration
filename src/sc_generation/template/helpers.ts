/**
 * Helper function to transform a string to a StaticArray<u8>
 * @param {string} str
 * @return {StaticArray<u8>}
 */
export function toBytes(str: string): StaticArray<u8> {
    let arr = new StaticArray<u8>(str.length << 1);
    memory.copy(changetype<usize>(arr), changetype<usize>(str), arr.length);
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
