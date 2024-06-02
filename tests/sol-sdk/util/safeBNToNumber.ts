import BN from "bn.js";

const MAX_SAFE_INTEGER = Number.MAX_SAFE_INTEGER;
const MIN_SAFE_INTEGER = Number.MIN_SAFE_INTEGER;


export function safeBNToNumber(bnValue: BN): number {
    const bigIntValue = BigInt(bnValue.toString());

    if (bigIntValue > BigInt(MAX_SAFE_INTEGER)) {
        return MAX_SAFE_INTEGER;
    } else if (bigIntValue < BigInt(MIN_SAFE_INTEGER)) {
        return MIN_SAFE_INTEGER;
    } else {
        return Number(bigIntValue);
    }
}