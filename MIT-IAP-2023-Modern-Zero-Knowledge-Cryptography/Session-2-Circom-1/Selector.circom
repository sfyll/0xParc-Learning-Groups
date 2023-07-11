pragma circom 2.1.2;

template Selector (nChoices) {
    signal input in[nChoices];
    signal input index;

    signal temp;

    signal output out;

    var withinBounds = isWithinBounds(index, nChoices);

    temp <-- withinBounds!=1 ? 0 : in[index];

    out <== temp;
}

function isWithinBounds(index, higherBound) {
    if (index >= 0 && index < higherBound) {return 1;}
    else {return 0;}
}

component main = Selector(2);

/* INPUT = {
    "in": ["0", "2"],
    "index": "1"
} */