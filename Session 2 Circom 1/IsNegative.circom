pragma circom 2.1.2;

include "circomlib/poseidon.circom";
include "circomlib/compconstant.circom";
include "circomlib/bitify.circom";

template IsNegative (N) {
    signal input in;
    signal output out;

    signal bits_representation[N];

    component num2bits = Num2Bits(N);

    num2bits.in <== in;

    bits_representation <== num2bits.out;

    component compconstant = CompConstant(10944121435919637611123202872628637544274182200208017171849102093287904247808);

    compconstant.in <== bits_representation;

    var i;

    for (i = 0; i<N; i++) {
        bits_representation[i] === compconstant.in[i];
    }

    out <== compconstant.out;
}

component main = IsNegative(254);

/* INPUT = {
    "in": "10944121435919637611123202872628637544274182200208017171849102093287904247810"
} */