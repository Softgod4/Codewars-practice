// function DNAtoRNA(dna: string): any {
//     let index: number = dna.indexOf('T');
//     let str = dna.substring(0, index) + "U" + dna.substring(index+1, dna.length);
//     return str;
// }

function DNAtoRNA(dna: string): any {
    return dna.replace(/U/g, "T");
}

console.log(DNAtoRNA("UUdTU"))