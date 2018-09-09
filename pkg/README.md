# rust-calculator

This is a very bare bones calculator made for the purposes of exploring Web Assembly. I don't think there is any viable reason for using it. So don't.

But if you want to...

## Installation

`npm install rust-calculator`

Importing a web assembly module is a little different than normals JS modules. They must be imported asynchronously. 
```
const wasm = import('rust-calculator');

wasm.then(calculator => {
  calculator.calculate(4.0, 3.0, '*');
});
```

I believe you can get around the asynchronous syntax with some configuring in Webpack 4.

## API

### .calculate(number, number, operator) -> number

The operator must be a string, and one of `+`, `-`, `/`, `*`. Returns a number.


