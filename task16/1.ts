export function ipToInt32(ip: string): any {
    const current: number[] = [];
    let smartStr = ip.split('.');
    let result = Number(smartStr[0])*2**24 + Number(smartStr[1])*2**16 + Number(smartStr[2])*2**8 + Number(smartStr[3]);
    return result;
}
// "128.32.10.1" => 2149583361
console.log(ipToInt32("128.32.10.1"));