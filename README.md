# Stanza delle Necessità

Questo progetto è un server web scritto in Rust utilizzando Actix Web. Il server gestisce una "stanza delle necessità" che può essere occupata e liberata da utenti. Inoltre, tiene traccia di una classifica degli utenti basata sul tempo totale di occupazione della stanza.
Il frontend utilizza React e TypeScript per visualizzare la classifica e permettere agli utenti di occupare e liberare la stanza.

## Funzionalità

- **Occupazione della stanza**: Gli utenti possono occupare la stanza inviando una richiesta POST a `/occupy_room`.
- **Liberazione della stanza**: Gli utenti possono liberare la stanza inviando una richiesta POST a `/free_room`.
- **Classifica**: La classifica degli utenti basata sul tempo totale di occupazione della stanza può essere visualizzata inviando una richiesta GET a `/leaderboard`.
- **Liberazione automatica**: La stanza viene liberata automaticamente se rimane occupata per più di 30 minuti.
