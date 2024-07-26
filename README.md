Initial:

0 ---- ? ---- D

En test, O -> D:
Si pas d'obstacle -> OK FIN
Si non:
    Cree deux nodes P1 (Point de l'obstacle le plus eloigne du segment O -> D par la gauche) et P2 Pareil par la droite.

On a:

        _- P1 -_
       /        \
      ?          ?
     /            \
0 -<                >- D
     \            /
      ?          ?
       \_      _/
         - P2 -

Si O -> P1 OK

let origin = P(...);
let dest = P(---);


