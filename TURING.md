# Turing Machine Construction for Rule 110
transliterated from ["Reproducing the cyclic tag system developed by Matthew Cook with Rule 110 using the phases fi_1"](http://www.comunidad.escom.ipn.mx/genaro/Papers/Papers_on_CA_files/repCTSR110.pdf)

## Naming Conventions
I found the names for individual gliders used in the paper quite cryptic and
not very unique, so I've renamed them based on what they do / look like.

Note that 'ether' is still 'ether'.

### Basic Gliders
| Old   | New       |
| ----- | --------- |
| A*n*  | lazer *n* |
| C2    | stacker   |
| Ē     | wiggler   |

### Glider Groups
| Old        | New              |
| ---------- | ---------------- |
| 1Ele_C2    | true stackers    |
| 0Ele_C2    | false stackers   |
| 0Blo_E     | zero wiggler     |
| 1BloP_E    | primary wiggler  |
| 1Blos_E    | standard wiggler |
| SepInit_EE | leading wiggler  |
| 1Add_E     | one add wiggler  |
| 0Add_E     | zero add wiggler |

Hopefully these names are more memorable.

## Layout
- Optional Repeat Number
- Glider / Glider Group
    - Optional Variant
- Optional Comma
    - Fase information

'...' means the below pattern may be repeated arbitrarily many times.
(I think, it's not clear in the original paper.)

## Left
Holds "clock pulses"

- ...
- 217 ether
- 4 lazer 4, F2
- 649 ether
- 4 lazer 4, F1
- 649 ether
- 4 lazer 4, F3
- 649 ether
- 4 lazer 4, F2
- 649 ether
- 4 lazer 4, F1
- 649 ether
- 4 lazer 4, F3
- 216 ether

## Center
Holds the data. Can be changed / extended.

- true stackers A, fase 1 1
- ether
- lazer 3, fase 1 1


## Right
Holds the states.

- repeat twice:
    - leading wiggler C, fase 3 1
    - primary wiggler C, fase 4 1
- zero wiggler C, fase 4 1
- standard wiggler A, fase 4 1
- leading wiggler A, fase 2 1, 2
- primary wiggler F, fase 1 1
- leading wiggler A, fase 3 1, 2
- primary wiggler F, fase 1 1
- zero wiggler E, fase 4 1
- standard wiggler C, fase 4 1
- ether
- leading wiggler B, fase 1 1, 2
- primary wiggler F, fase 3 1
- ether
- leading wiggler B, fase 1 1, 2
- ether 217
...
