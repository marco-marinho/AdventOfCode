import {parseLines} from '../Util';

let data = parseLines(__dirname, 'data.txt');
const re_brackets = RegExp('\\[.*?\\]');
const re_brackets_global = RegExp('\\[.*?\\]', 'g');

task_01();
task_02();

function task_01() {
    let count = 0;
    for (let entry of data) {
        let outside_brackets = entry.split(re_brackets);
        let inside_brackets = [...entry.matchAll(re_brackets_global)];
        let status = false;
        for (let subentry of outside_brackets) {
            status ||= check_string(subentry);
        }
        for (let subentry of inside_brackets) {
            status &&= !check_string(subentry[0]);
        }
        if (status) {
            count++;
        }
    }
    console.log('Task 01: ' + count);
}

function task_02() {
    let count = 0;
    for (let entry of data) {
        let abas: string[] = []
        let outside_brackets = entry.split(re_brackets);
        let inside_brackets = [...entry.matchAll(re_brackets_global)];
        for (let subentry of outside_brackets) {
            let buff = get_aba(subentry);
            abas = abas.concat(buff);
        }
        if (abas.length === 0) {
            continue;
        }
        let status = false;
        for (let subentry of inside_brackets) {
            status ||= check_aba(subentry[0], abas);
        }
        if (status) {
            count++;
        }
    }
    console.log('Task 02: ' + count);
}

function check_string(input: string): boolean {
    const len = input.length
    for (let index of Array(len - 3).keys()) {
        const slice = Array.from(input.slice(index, index + 4));
        if (slice[0] === slice[3] && slice[1] === slice[2] && slice[0] !== slice[1]) {
            return true;
        }
    }
    return false;
}

function get_aba(input: string): string[] {
    const len = input.length
    let output: string[] = [];
    for (let index of Array(len - 2).keys()) {
        const slice = Array.from(input.slice(index, index + 3));
        if (slice[0] === slice[2] && slice[0] !== slice[1]) {
            output.push([slice[1], slice[0], slice[1]].join(''));
        }
    }
    return output;
}

function check_aba(input: string, abas: string[]): boolean {
    const len = input.length
    for (let index of Array(len - 2).keys()) {
        const slice = Array.from(input.slice(index, index + 3));
        if (abas.includes(slice.join(''))) {
            return true;
        }
    }
    return false;
}

