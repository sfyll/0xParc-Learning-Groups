pragma circom 2.1.2;

template IsZero () {
    signal input in;
    
    signal output out;

    signal inverse;

    inverse <-- getInverse(in);

    out <== -in * inverse + 1;
}

function getInverse(n) {
    if (n == 0) {return 0;}
    else {return 1 / n;}
}

template IsEqual () {
    signal input in[2];
    
    signal output out;

    signal to_test;

    to_test <-- in[0] - in[1];

    log(to_test);

    component is_zero = IsZero();

    is_zero.in <== to_test;

    out <== is_zero.out;
}


component main = IsEqual();

/* INPUT = {
    "in": ["0", "2"]
} */