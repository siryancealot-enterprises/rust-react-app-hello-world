// example.js
declare var runjs: any

console.log("Hello", "runjs!");
console.error("Boom!");

const path = "./log.txt";
try {
   const contents = await runjs.readFile(path);
   console.log("Read from a file", contents);
} catch (err) {
   console.error("Unable to read file", path, err);
}

await runjs.writeFile(path, "I can write to a file.");
const contents = await runjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
runjs.removeFile(path);
console.log("File removed!");


interface Foo {
   bar: string;
   fizz: number;
}

const person: Foo = {
   bar: "John",
   fizz: 30,
};

console.log("here's a person: ", person);

let content: string;
content = await runjs.fetch("https://deno.land/std@0.177.0/examples/welcome.ts",);
console.log("Content from fetch", content);

export { };
