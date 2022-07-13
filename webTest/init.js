import * as wasm from './pkg';

window.onload = function() {
    console.log("loaded correctly");
    document.getElementById("button3").onclick=function(){test123();};
    //document.getElementById("button3").addEventListener("click", test123);
}

let result = wasm.add(2, 1);

function test123() {
    var x = wasm.add(1,4);
    console.log(x);
    document.getElementById("myVariable").innerHTML=x;
}

