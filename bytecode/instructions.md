Rev: 3
Auto builded from `instructions.csv` by `reassembler.js`
| Instruction | Implicit | Immediate | Absolute | Absolute Index | Absolute Property | IndirectA | IndirectB | IndirectC | IndirectX | IndirectY | Parameter |
| ----------- | -------- | --------- | -------- | -------------- | ----------------- | --------- | --------- | --------- | --------- | --------- | --------- |
| lda         |    --    |    0x01   |   0x02   |      0x07      |        0x08       |     --    |    0x03   |    0x04   |    0x05   |    0x06   |    0x09   |
| ldb         |    --    |    0x0a   |   0x0b   |      0x10      |        0x11       |    0x0c   |     --    |    0x0d   |    0x0e   |    0x0f   |    0x12   |
| ldc         |    --    |    0x13   |   0x14   |      0x19      |        0x1a       |    0x15   |    0x16   |     --    |    0x17   |    0x18   |    0x1b   |
| ldx         |    --    |    0x1c   |   0x1d   |      0x22      |        0x23       |    0x1e   |    0x1f   |    0x20   |     --    |    0x21   |    0x24   |
| ldy         |    --    |    0x25   |   0x26   |      0x2b      |        0x2c       |    0x27   |    0x28   |    0x29   |    0x2a   |     --    |    0x2d   |
| sta         |   0x2e   |    0x2f   |   0x30   |      0x31      |        0x32       |     --    |     --    |     --    |     --    |     --    |    0x33   |
| stb         |   0x34   |    0x35   |   0x36   |      0x37      |        0x38       |     --    |     --    |     --    |     --    |     --    |    0x39   |
| stc         |   0x3a   |    0x3b   |   0x3c   |      0x3d      |        0x3e       |     --    |     --    |     --    |     --    |     --    |    0x3f   |
| stx         |   0x40   |    0x41   |   0x42   |      0x43      |        0x44       |     --    |     --    |     --    |     --    |     --    |     --    |
| sty         |   0x45   |    0x46   |   0x47   |      0x48      |        0x49       |     --    |     --    |     --    |     --    |     --    |    0x4a   |
| eq          |   0x4b   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| ne          |   0x4c   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| gt          |   0x4d   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| lt          |   0x4e   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| gq          |   0x4f   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| lq          |   0x50   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| and         |   0x51   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| or          |   0x52   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| add         |   0x53   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| sub         |   0x54   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| mul         |   0x55   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| exp         |   0x56   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| div         |   0x57   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| mod         |   0x58   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| inc         |   0x59   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| dec         |   0x5a   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| jmp         |    --    |     --    |   0x5b   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| call        |    --    |     --    |   0x5c   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| ret         |   0x5d   |    0x5e   |   0x5f   |      0x65      |        0x66       |    0x60   |    0x61   |    0x62   |    0x63   |    0x64   |     --    |
| ugr         |   0x67   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| ulr         |   0x68   |     --    |   0x69   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| push        |    --    |     --    |   0x6a   |      0x70      |        0x71       |    0x6b   |    0x6c   |    0x6d   |    0x6e   |    0x6f   |     --    |
| len         |   0x72   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2i         |   0x73   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2f         |   0x74   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2d         |   0x75   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2b         |   0x76   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2s         |   0x77   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2c         |   0x78   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| a2o         |   0x79   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| jmpa        |    --    |     --    |   0x7a   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| pops        |   0x7b   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| acp         |    --    |     --    |   0x7c   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| brk         |   0x7d   |     --    |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| calln       |    --    |    0x7e   |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| co          |    --    |     --    |   0x7f   |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |
| fn          |    --    |    0x80   |    --    |       --       |         --        |     --    |     --    |     --    |     --    |     --    |     --    |