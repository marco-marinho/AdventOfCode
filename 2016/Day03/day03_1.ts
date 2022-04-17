const input: string[] = require('./input');

function check_if_triangle(sides: number[]): boolean{
    let a = sides[0];
    let b = sides[1];
    let c = sides[2];

    return a + b > c && a + c > b && b + c > a;

}

let num_possible: number = 0;
for(let element of input){
    let sides = element.trim().split(/[ ]+/).map(
        function(item){
            return parseInt(item, 10);
    });
    if(check_if_triangle(sides)){
        num_possible++;
    }
}

console.log(num_possible);