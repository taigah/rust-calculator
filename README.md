# Une calculatrice simple en Rust

Un simple projet pour prendre en main rust et m'entraîner aux concepts de lexing et parsing. Certaines parties du code seraient à réorganiser, en particulier les structures de `Token`, `TokenType` et `TokenFamily`. En effet, on se retrouve avec quelques problèmes quand il s'agit par exemple de calculer la priorité opératoire d'un opérateur étant donné qu'il n'y a pas de type propre aux opérateurs...

## Utilisation

```bash
cargo run --release
```
