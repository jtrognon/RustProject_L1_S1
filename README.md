Mini jeu venant de *Fallout*.

Contexte :
<br>
Des terminaux sont disseminés dans le monde et il faut les hacker pour pouvoir accéder à leurs contenu et
débloquer les fonctionnalités du lieu.


Exemple de Terminal :
<img src="TerminalImage.jpg">

But du jeu :
<br>
Trouver le mot de passe parmi les mots disponibles pour débloquer le terminal.

Règles :
- Il faut choisit un mot ou caractère à chaque tour de jeu
- Il y a un nombre d'essais maximum
    - le choix d'un mot utilise un essai
    - les '{}', '[]', '()', '<>', n'utilise pas d'essai
- A chaque mot choisi, les possibilités sont :
    - le mot est le bon -> C'est gagné
    - le mot n'est pas bon -> Le terminal indique le nombre de lettres à la bonne place (il ne dit pas les quelles)
- les caractères '{}', '[]', '()' et '<>' permettent de : 
    - Ajouter un essai si il n'y a rien entre eux
    - Retirer un mot sinon
- /!\ Ils faut que l'ouverture match avec la fermeture pour les caractères spéciaux
  