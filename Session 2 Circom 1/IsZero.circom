pragma circom 2.1.2;

template IsZero () {
    signal input in;
    
    signal output out;

    signal inverse;

    inverse <-- getInverse(in);

    out <== -in * inverse + 1;

    in * out === 0;
}

function getInverse(n) {
    if (n == 0) {return 0;}
    else {return 1 / n;}
}

component main = IsZero();

/* INPUT = {
    "in": "0"
} */