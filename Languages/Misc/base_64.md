### Base 64 encoding.

```
    ex) the string  "hey"....

        1.    "hey" ==> ASCII = 72 101 121.
            convert 72 101 121 into bits....

        2.    01001000 01100101 01111001

        then break these bits into 6 bit chucks ...(base64)

        3.   010010 000110 010101 111001
                18    6       21    57         these are mapped to a base64 chart

                S     G       V      5


                "hey" == SGV5
```
