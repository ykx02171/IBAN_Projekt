# Protokoll
## Verständnis der Aufgabe
##### Wie interpretierst du die Anforderungen?
- Ohne Verwendung bereits existierender Drittbibliotheken oder APIs einen Endpunkt zur IBAN-Validierung implementieren.
- Der Endpunkt soll eine IBAN als Eingabe erhalten und das Ergebnis der Validierung im JSON-Format zurückgeben.

##### Welche Annahmen triffst du?
- Der Fokus liegt auf der korrekten Implementierung der Validierungslogik

## Planung
##### Welche Schritte planst du?
- Zuerst die grundlegende Syntax von Rust lernen.
- Danach anhand der Axum-Dokumentation und Tutorials verstehen, wie man einen einfachen HTTP-Server implementiert.
- Den Beispielcode anpassen, um einen Endpunkt zur IBAN-Validierung zu erstellen.
- Während der Umsetzung festgestellt, dass die Eingabe einer IBAN über die Browser-URL umständlich ist.
- Deshalb zusätzlich ein einfaches Frontend erstellt, um die IBAN über ein Eingabefeld zu senden.

##### Welche offenen Fragen ergeben sich?
- Ich habe Rust und Axum nicht gelernt

## Recherche
##### Wie gehst du an das Thema IBAN-Validierung heran?
- Über Wikipedia den Algorithmus zur IBAN-Prüfung (MOD-97-Verfahren) gefunden.
- Die einzelnen Schritte der Validierung wird im README.md gezeigt。

##### Welche Quellen nutzt du?
- Rust-Dokumentation
- Axum-Dokumentation
- YouTube-Tutorials zur Verwendung von Axum
- Wikipedia zur Beschreibung des IBAN-Validierungsalgorithmus
- ChatGPT zur Generierung der Tuple von Ländercodes und zugehörigen IBAN-Längen 

## Umsetzung
##### Beschreibung deiner Lösungsidee
- Prüfung der Länge der IBAN
- Prüfung des Ländercodes
- Prüfung, ob die IBAN nur aus gültigen Zeichen (Großbuchstaben und Ziffern) besteht
- Umordnung der IBAN
- Umwandlung der Buchstaben in Zahlen
- Durchführung der MOD-97-Prüfung, um die Gültigkeit zu bestimmen.

##### Entscheidungen, zum Beispiel zur Struktur oder zum Validierungsansatz

## Probleme & Lösungen
##### Welche Schwierigkeiten sind aufgetreten?
- Der Datentyp i128 reicht nicht aus, um die gesamte Zahl direkt zu speichern.

##### Wie hast du sie gelöst?
- Verwendung des Rolling-MOD-97-Verfahrens.

## Reflexion
##### Was würdest du beim nächsten Mal anders machen?
- Die Prüfung, ob die IBAN nur aus Ziffern und Großbuchstaben besteht, könnte früher im Validierungsprozess erfolgen
- Ein Teil der Validierung kann bereits im Frontend erfolgen, um Serverressourcen zu sparen.
- Im Frontend verschlüsselt und im Backend entschlüsslt.
- Die Fehlerbehandlung könnte verbessert werden, indem mehre Fehlertypen definiert werden
---
# Chronologische Schritte
1. Vom Bekommen der Aufgabe bis Freitagabend lernte ich zunächst die Grundlage von Rust.
2. Am Sonntag habe ich dann mit der Umsetzung der Aufgabe begonnen.
Zuerst habe ich durch das Lesen der Dokumentation einen Überblick über Axum bekommen.
Danach habe ich auf Youtube ein Video gesucht, um die grundlegende Verwendung von Axum zu lernen.
3. Auf Basis dieses Videos habe ich den Beispielcode angepasst und erweitert, um ihn für mein Projekt zu verwenden.
4. Zuerst habe ich den Server gestartet.
Ich habe folgenden Router implementiert:
``` 
Router::new().route("/iban_validation/{iban}", get(handler))
```
Allerdings konnte damit der Pfad
```
localhost:8080/iban_validation/
``` 
nicht korrekt verarbeitet werden.
Deshalb habe ich den Router wie folgt erweitert:
```
Router::new().route("/iban_validation/{iban}", get(handler))
        .route("/iban_validation/", get(empty_handler))
```
5. Danach war es möglich, eine IBAN einzugeben und eine Antwort vom Server zu erhalten.
6. Durch Recherche habe ich die Regeln und den Validierungsalgorithmus der IBAN untersucht.
7. Danach habe ich zunächst eine Längenprüfung implementiert.
Beim Testen mit kopierten IBANs habe ich festgestellt, dass diese häufig Leerzeichen enthalten.
Deshalb habe ich recherchiert, wie man in Rust Leerzeichen aus Strings entfernt, und dafür die replace-Funktion verwendet.
8. Danach habe ich die Prüfung des Ländercodes implementiert.
Dabei ist mir aufgefallen, dass nicht jede Kombination aus zwei Großbuchstaben ein gültiger Ländercode ist.
Außerdem haben verschiedene Länder unterschiedliche IBAN-Längen.
Daher benötigte ich eine Datenstruktur, die Ländercode und IBAN-Länge enthält.
Diese habe ich als Tuple-Struktur umgesetzt, um sowohl die Gültigkeit des Ländercodes als auch die passende Länge zu prüfen.
Da diese Liste relativ komplex ist, habe ich ChatGPT verwendet, um die entsprechenden Tupel zu generieren.
Diese Daten befinden sich in der Datei /rust_axum/src/data.rs.
9. Der letzte Schritt war die Implementierung der MOD-97-Prüfung, basierend auf der Beschreibung aus Wikipedia.
Dabei trat das Problem auf, dass die IBAN nach der Umwandlung der Buchstaben in Zahlen sehr lang wird.
Dadurch kann die Zahl die üblichen Integer-Grenzen überschreiten.
Deshalb habe ich nach einer Lösung gesucht und das sogenannte Rolling-Modulo-Verfahren (Rolling MOD 97) gefunden, das dieses Problem löst.
10. Beim Verbinden von Frontend und Backend trat ein CORS-Problem auf.
Nach einer Recherche im Internet wird eine Lösung gefunden und umgesetzt.
```
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
let app = create_app().layer(cors);
```