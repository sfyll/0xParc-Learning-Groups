pragma circom 2.1.2;


template Num2Bits (nBits) {
    signal input in;

    signal output b[nBits];

    for (var i = 0; i < nBits; i++) {
        b[i] <-- ( in \ 2 ** i ) % 2;
    }

    var accum = 0;
    for (var i = 0; i < nBits; i++) {
        accum += (2 ** i) * b[i];
    }

    in === accum;

    for (var i = 0; i < nBits; i++) {
        0 === b[i] * (b[i] - 1);
    }
}


component main = Num2Bits(5);

/* INPUT = {
    "in": "11"
} */