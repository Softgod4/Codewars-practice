function findShort(s: string): any {
    let lengthWord: number[] = [];
    const arrayWord = s.split(" ");
    for (const word in arrayWord) {
        lengthWord.push(arrayWord[word].length)
    }
    for (const n in lengthWord) {
        console.log(lengthWord[n])
    }
    return Math.min(...lengthWord)
}

console.log(findShort("Lets all go on holiday somewhere very cold"))