pragma circom 2.1.2;
//answer from circom. This is more a CS class with bitwise logic than a focus on ZK. main point is by activating the most significant bit (MBS) before substracting
// in[1] from i[0], one can be sure than if i[0] > i[i], the substraction will deactivate this bit. Which is ultimately what is tested on the last line, is the bit still active?

template LessThan(n) {
    assert(n <= 252);
    signal input in[2];
    signal output out;

    component n2b = Num2Bits(n+1);

    n2b.in <== in[0]+ (1<<n) - in[1];

    out <== 1-n2b.out[n];
}

template Num2Bits(n) {
    signal input in;
    signal output out[n];
    var lc1=0;

    var e2=1;
    for (var i = 0; i<n; i++) {
        out[i] <-- (in >> i) & 1;
        out[i] * (out[i] -1 ) === 0;
        lc1 += out[i] * e2;
        e2 = e2+e2;
    }

    lc1 === in;
}

component main = LessThan(252);

/* INPUT = {
    "in": ["1", "2"]
} */