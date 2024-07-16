// function stringTransformer(str: string) {
//     let current: string[] = []
//     const s: string[] = str.split("");
//     for (let i: number = 0; i<s.length; i++) {
//         if (s[i] == s[i].toUpperCase()) {
//             current.push(s[i].toLowerCase());
//         } else {
//             current.push(s[i].toUpperCase())
//         }
//     }
//     return current.reverse().join('');
// }

// console.log(stringTransformer('example Input'))

function stringTransformer(str: string) {
    let current: string[] = []
    const arraymax: string[] = str.split(" ");
    for (let i: number = 0; i<arraymax.length; i++) {
        let arraymix = arraymax[i].split('');
        let wordArray: string[] = [];
        for (let k: number = 0; k<arraymix.length; k++) {
            if (arraymix[k] == arraymix[k].toUpperCase()) {
                wordArray.push(arraymix[k].toLowerCase())
            } else {
                wordArray.push(arraymix[k].toUpperCase());
            }
        }
        current.push(wordArray.join(''));
    }
    return current.reverse().join(' ');
}

console.log(stringTransformer('Example Input'))