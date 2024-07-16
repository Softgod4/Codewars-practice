export function alphanumeric(string: string): any {
    return /^[a-zA-Z0-9]+[\s]*$/gi.test(string)
}

console.log(alphanumeric("Helloworld"))
console.log(alphanumeric("Hello world"))
console.log(alphanumeric("Hello_world"))
console.log(alphanumeric("Hello_worl4d"))
console.log(alphanumeric(" "))
console.log(alphanumeric(""))