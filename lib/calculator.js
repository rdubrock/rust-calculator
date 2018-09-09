/* tslint:disable */
import * as wasm from './calculator_bg';

/**
* @param {number} arg0
* @param {number} arg1
* @param {string} arg2
* @returns {number}
*/
export function calculate(arg0, arg1, arg2) {
    return wasm.calculate(arg0, arg1, arg2.codePointAt(0));
}

