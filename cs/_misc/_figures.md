

hex| bin       | dec
---|-----------|---:
00 | 0000 0000 |  0
04 | 0000 0100 |  4
08 | 0000 1000 |  8
0C | 0000 1100 | 12
.  |           |
10 | 0001 0000 | 16
14 | 0001 0100 | 20
18 | 0001 1000 | 24
1C | 0001 1100 | 28
.  |           |
20 | 0010 0000 | 32
24 | 0010 0100 | 36
28 | 0010 1000 | 40
2C | 0010 1100 | 44
.  |           |
30 | 0011 0000 | 48
34 | 0011 0100 | 52
38 | 0011 1000 | 56
3C | 0011 1100 | 60




hex         | bin (x86)                                | dec
------------|------------------------------------------|-----
00 00 00 00 | 0000 0000 0000 0000  0000 0000 0000 0000 | 0
00 00 00 01 | 0000 0000 0000 0000  0000 0000 0000 0001 | 1
00 00 00 02 | 0000 0000 0000 0000  0000 0000 0000 0010 | 2
00 00 00 04 | 0000 0000 0000 0000  0000 0000 0000 0100 | 4
00 00 00 08 | 0000 0000 0000 0000  0000 0000 0000 1000 | 8
00 00 00 0C | 0000 0000 0000 0000  0000 0000 0000 1100 | 12
00 00 00 10 | 0000 0000 0000 0000  0000 0000 0001 0000 | 16
00 00 00 14 | 0000 0000 0000 0000  0000 0000 0001 0000 | 20
00 00 00 18 | 0000 0000 0000 0000  0000 0000 0001 1000 | 24
00 00 00 1C | 0000 0000 0000 0000  0000 0000 0001 0000 | 28
00 00 00 20 | 0000 0000 0000 0000  0000 0000 0010 0000 | 32
00 00 00 40 | 0000 0000 0000 0000  0000 0000 0100 0000 | 64
00 00 00 80 | 0000 0000 0000 0000  0000 0000 1000 0000 | 128
00 00 00 FF | 0000 0000 0000 0000  0000 0000 1111 1111 | 255
00 00 01 00 | 0000 0000 0000 0000  0000 0001 0000 0000 | 256


canonical address

hex                 | exp     | dec
--------------------|---------|-------------------------:
lower half:         |         |
0000 0000 0000 0000 | 2^0 -1  |                         0
0000 0000 0000 000F | 2^4 -1  |                        15
0000 0000 0000 00FF | 2^8 -1  |                       255
0000 0000 0000 FFFF | 2^16-1  |                    ‭65.535‬
0000 0000 FFFF FFFF | 2^32-1  |             4.294.967.295‬
0000 000F FFFF FFFF | 2^36-1  |            68.719.476.735
0000 00FF FFFF FFFF | 2^40-1  |         ‭1.099.511.627.775‬
0000 7FFF FFFF FFFF | 2^47-1  |       ‭140.737.488.355.327‬
0000 FFFF FFFF FFFF | 2^48-1  |       ‭281.474.976.710.655‬
higher half:        |         |
FFFF 8000 0000 0000 |         |
FFFF FFFF FFFF FFFF | 2^64-1  |