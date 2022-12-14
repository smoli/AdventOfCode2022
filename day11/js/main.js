const fs = require("fs");

function parse(input) {
    const rmonkey = /^\s*Monkey (\d+):/;
    const ritems = /^\s*Starting items: (.*)/;
    const roperation = /^\s*Operation: new = old (.) (.*)/;
    const rtest = /^\s*Test: divisible by (\d+)/;
    const rif_true = /^\s*If true: throw to monkey (\d+)/;
    const rif_false = /^\s*If false: throw to monkey (\d+)/;

    let res = [];
    let p = 0;

    while (p < input.length) {
        res.push({
            num: 1 * input[p++].match(rmonkey)[1],
            items: input[p++].match(ritems)[1].split(", ").map(i => Number(i)),
            operation: (() => {
                let m = input[p++].match(roperation);
                return {
                    op: m[1],
                    v: m[2]
                }
            })(),
            test: 1 * input[p++].match(rtest)[1],
            ifTrue: 1 * input[p++].match(rif_true)[1],
            ifFalse: 1 * input[p++].match(rif_false)[1],
            inspections: 0
        });
        p++;
    }

    return res;
}

function performOp(op, value) {
    // console.log(value, op);
    switch (op.op) {
        case '+': return value + 1 * op.v;
        case '-': return value - 1 * op.v;
        case '/': return value / 1 * op.v;
        case '*': return op.v === "old" ? value * value : value * 1 * op.v;

        default:
            break;

    }
}

function stuffSlingingSimianShenanigans(monkeys) {

    for(let i = 0; i < monkeys.length; i++) {

        const m = monkeys[i];
        console.log(m.num, m.items.length, "->", `[${m.items.join(", ")}]`);

        m.inspections += m.items.length;

        if (m.num === 6) {
            console.log(m.items.join(", "))
        }

        m.items = m.items.map(i => performOp(m.operation, i));

        if (m.num === 6) {
            console.log(m.items.join(", "))
        }
        m.items.forEach(i => {
            let ni = Math.floor(i / 3);

            if (ni === 26019075) {
                console.log(m.operation)
                console.log(i);
                console.log("---------->", m.num, m.test);
            }

            if (ni % m.test === 0) {
                monkeys[m.ifTrue].items.push(ni);
            } else {
                monkeys[m.ifFalse].items.push(ni);
            }
        });

        m.items = [];
    }

    return monkeys;
}


let input = fs.readFileSync("../input.txt");

let monkeys = parse(("" + input).split("\r\n"));

for (i = 0; i < 20; i++) {
    console.log("\n\nRound", i + 1);
    monkeys = stuffSlingingSimianShenanigans(monkeys);
}


console.log(monkeys.map(m => {
    return { num: m.num, inspections: m.inspections}
}))

monkeys.sort((a, b) => b.inspections - a.inspections);

console.log(monkeys[0].inspections * monkeys[1].inspections);