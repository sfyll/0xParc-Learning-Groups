Fibonacci computations where we check the result at a desired step. The output should be equal to our passed in parameter.  
  
Done using halo2-lib. When compared with the halo2-proof implementation, it shows that the API hides most of the complexity in building circuits,
assuming you can use vertical gates of format q * (a + b * c - d) == 0