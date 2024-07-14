function order(words:string): any {
    const arr: string[] = [];
    let s = words.split(" ");
    for (let i = 1; arr.length<s.length;) {
        let find: number = parseInt(s[i].match(/\d+/)?.[0] || "0");
        if (find == arr.length+1) {
            arr.push(s[i]);
            i++
        } else {
            i++
        }
        if (i==s.length) i=0;
    }
    return arr.join(" ");
}

console.log(order("4of Fo1r pe6ople g3ood th5e the2"));