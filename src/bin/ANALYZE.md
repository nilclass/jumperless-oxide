
```
Expected: [
(1, [Top6, Bottom16, Gnd, Gnd]),
(2, [Bottom9, Bottom26, Supply5V, Supply5V]),
(3, [Top3, Supply3V3]),
(4, [Bottom2, Bottom4, Dac05V, Dac05V]),
(5, [Top13, Bottom13, Adc05V, Dac18V, Adc05V, Dac18V, Top30]),
(7, [Top11, Bottom17, Bottom18]),
(8, [Bottom19]), (9, [Bottom3, Bottom15])
]
Actual:   [
(1, [Top6, Bottom16, Gnd]),
(2, [Bottom9, Bottom26, Supply5V]),
(3, [Top3, Bottom15, Supply3V3]),
(4, [Bottom2, Bottom4, Dac05V]),
(5, [Top13, Top30, Bottom13, Dac18V]),
(6, [Top2, Bottom1, Bottom22, CurrentSensePlus]),
(7, [Top11, Bottom17, Bottom18, CurrentSenseMinus]),
(8, [Bottom3, Bottom19]), (9, [Bottom3, Bottom15])]
```

```
[
  {
    "index": 1,
    "number": 1,
    "nodes": [
      "GND",
      46,
      6
    ],
    "special": true,
    "color": "#00ff30",
    "machine": false,
    "name": "GND"
  },
  {
    "index": 2,
    "number": 2,
    "nodes": [
      "SUPPLY_5V",
      56,
      39
    ],
    "special": true,
    "color": "#ff4114",
    "machine": false,
    "name": "+5V"
  },
  {
    "index": 3,
    "number": 3,
    "nodes": [
      "SUPPLY_3V3",
      45,
      3
    ],
    "special": true,
    "color": "#ff1040",
    "machine": false,
    "name": "+3.3V"
  },
  {
    "index": 4,
    "number": 4,
    "nodes": [
      "DAC0",
      32,
      34
    ],
    "special": true,
    "color": "#ef787a",
    "machine": false,
    "name": "DAC 0"
  },
  {
    "index": 5,
    "number": 5,
    "nodes": [
      "DAC1",
      13,
      43,
      30
    ],
    "special": true,
    "color": "#ef407f",
    "machine": false,
    "name": "DAC 1"
  },
  {
    "index": 6,
    "number": 6,
    "nodes": [
      "ISENSE_PLUS",
      2,
      31,
      52
    ],
    "special": true,
    "color": "#ffffff",
    "machine": false,
    "name": "I Sense +"
  },
  {
    "index": 7,
    "number": 7,
    "nodes": [
      "ISENSE_MINUS",
      11,
      48,
      47
    ],
    "special": true,
    "color": "#ffffff",
    "machine": false,
    "name": "I Sense -"
  },
  {
    "index": 8,
    "number": 8,
    "nodes": [
      33,
      49
    ],
    "special": false,
    "color": "#c0d4a7",
    "machine": true,
    "name": "Net 8"
  },
  {
    "index": 9,
    "number": 9,
    "nodes": [
      45,
      33
    ],
    "special": false,
    "color": "#bd287b",
    "machine": true,
    "name": "Net 9"
  }
]
```

####################################

```
[
  {
    "index": 1,
    "number": 1,
    "nodes": [
      "GND",
      48,
      29,
      9
    ],
    "special": true,
    "color": "#00ff30",
    "machine": false,
    "name": "GND"
  },
  {
    "index": 2,
    "number": 2,
    "nodes": [
      "SUPPLY_5V",
      20,
      10
    ],
    "special": true,
    "color": "#ff4114",
    "machine": false,
    "name": "+5V"
  },
  {
    "index": 3,
    "number": 3,
    "nodes": [
      "SUPPLY_3V3",
      50,
      36,
      42
    ],
    "special": true,
    "color": "#ff1040",
    "machine": false,
    "name": "+3.3V"
  },
  {
    "index": 4,
    "number": 4,
    "nodes": [
      "DAC0",
      32,
      19,
      47
    ],
    "special": true,
    "color": "#ef787a",
    "machine": false,
    "name": "DAC 0"
  },
  {
    "index": 5,
    "number": 5,
    "nodes": [
      "DAC1",
      22,
      5,
      4
    ],
    "special": true,
    "color": "#ef407f",
    "machine": false,
    "name": "DAC 1"
  },
  {
    "index": 6,
    "number": 6,
    "nodes": [
      "ISENSE_PLUS",
      27,
      25
    ],
    "special": true,
    "color": "#ffffff",
    "machine": false,
    "name": "I Sense +"
  },
  {
    "index": 7,
    "number": 7,
    "nodes": [
      "ISENSE_MINUS",
      50,
      28,
      17
    ],
    "special": true,
    "color": "#ffffff",
    "machine": false,
    "name": "I Sense -"
  },
  {
    "index": 8,
    "number": 8,
    "nodes": [
      12,
      42
    ],
    "special": false,
    "color": "#6c8830",
    "machine": true,
    "name": "Net 8"
  },
  {
    "index": 9,
    "number": 9,
    "nodes": [
      53,
      13,
      15
    ],
    "special": false,
    "color": "#e7aaad",
    "machine": true,
    "name": "Net 9"
  }
]
```

```
CHIP STATUS: [
ChipStatus { char: 'A', x_status: [-1, 5, 2, 5, 2, -1, -1, -1, 4, -1, -1, -1, 4, -1, -1, -1], y_status: [5, -1, -1, 5, 5, -1, -1, -1] },
ChipStatus { char: 'B', x_status: [2, 5, -1, 1, -1, -1, -1, -1, -1, -1, 8, -1, -1, -1, 9, 9], y_status: [2, 1, 2, -1, 8, 9, -1, 9] },
ChipStatus { char: 'C', x_status: [2, -1, -1, -1, 4, 5, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1], y_status: [2, -1, 7, -1, 4, 2, -1, 5] },
ChipStatus { char: 'D', x_status: [-1, -1, -1, -1, 7, -1, 7, 1, -1, -1, -1, -1, 7, -1, -1, -1], y_status: [7, -1, -1, -1, -1, -1, -1, 1] },
ChipStatus { char: 'E', x_status: [4, -1, -1, -1, -1, -1, -1, -1, 3, -1, -1, -1, -1, -1, -1, -1], y_status: [4, 4, -1, -1, -1, 3, -1, -1] },
ChipStatus { char: 'F', x_status: [-1, -1, 8, -1, -1, -1, -1, -1, -1, -1, 3, -1, -1, -1, -1, -1], y_status: [-1, -1, -1, -1, 8, -1, -1, -1] }, 
ChipStatus { char: 'G', x_status: [4, -1, -1, -1, -1, -1, 7, -1, -1, -1, -1, -1, 3, 1, -1, -1], y_status: [4, -1, 4, 1, -1, 7, -1, -1] },
ChipStatus { char: 'H', x_status: [-1, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], y_status: [-1, 9, -1, -1, -1, -1, -1, -1] },
ChipStatus { char: 'I', x_status: [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 4, -1, 3, 1], y_status: [-1, -1, 4, -1, 3, 3, 3, -1] },
ChipStatus { char: 'J', x_status: [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, -1, 2, 1], y_status: [5, 1, 5, 1, -1, -1, 1, -1] },
ChipStatus { char: 'K', x_status: [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], y_status: [-1, -1, -1, -1, -1, -1, -1, -1] },
ChipStatus { char: 'L', x_status: [-1, -1, -1, -1, -1, -1, 5, 4, -1, -1, -1, -1, -1, -1, 2, -1], y_status: [5, 2, 2, -1, 4, -1, 4, -1] }]
```

```
Actual: [
(1, [Top9, Top29, Bottom18, Gnd]),
(2, [Top10, Top20, Supply5V]),
(3, [Bottom6, Supply3V3]),
(4, [Top19, Bottom2, Bottom17, Dac05V]),
(5, [Top4, Top5, Top22, Dac18V]),
(7, [Top17, Bottom20]),
(8, [Top12, Bottom12]),
(9, [Top13, Top15, Bottom23])]
Expected:   [
(1, [Top9, Top29, Bottom18, Gnd]),
(2, [Top10, Top20, Supply5V]),
(3, [Bottom6, Bottom12, Bottom20, Supply3V3]),
(4, [Top19, Bottom2, Bottom17, Dac05V]),
(5, [Top4, Top5, Top22, Dac18V]),
(6, [Top25, Top27, CurrentSensePlus]),
(7, [Top17, Top28, Bottom20, CurrentSenseMinus]),
(8, [Top12, Bottom12]),
(9, [Top13, Top15, Bottom23])]

```
