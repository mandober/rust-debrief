# bitwise operations

```shell
x=0xff # 255
echo $((x & x))
# 254
```



```
x & x = x


1010 
1010 &
1010
```

Turn off the rightmost set (1) bit in a word:`x & (x-1)`
- If no bits are set, it produces a 0.
- e.g. `0101'1000` -> `0101'0000`

This can be used to determine if an unsigned integer is a power of 2 or is 0: apply the formula followed by a 0-test on the result.

rms_off () { return $(( $1 & ($1 - 1) )); };


