{{#invoke:Mathe für Nicht-Freaks/Seite|oben}}
Mit der Ableitung wirst du eines der wichtigsten Konzepte der Analysis kennen lernen. Diese entspricht der aktuellen Änderungsrate einer Funktion. und wird in den Naturwissenschaften oft genutzt, um in mathematischen Modellen die Veränderung eines Systems zu modellieren. Auch kann mit Hilfe der Ableitung eine Funktion auf ihre Eigenschaften untersucht werden.

== Intuitionen der Ableitung ==
Für die Ableitung gibt es mehrere Intuitionen, die alle eng zusammenhängen. Diese sind:

* ''Ableitung als momentane Änderungsrate:'' Die Ableitung entspricht dem, was wir intuitiv als momentane Änderungsrate einer Funktion verstehen. Eine Änderungsrate beschreibt dabei, wie stark sich eine Größe bezüglich einer anderen Bezugsgröße ändert. Bei der momentanen Änderungsrate wird die Bezugsgröße als „unendlich klein“ angenommen. Ein Beispiel ist die Geschwindigkeit. Diese ist die momentane Änderungsrate des Ortes bezüglich der Zeit und gibt an, wie stark sich der Ort eines Objekts mit der Zeit zu einem bestimmten Zeitpunkt ändert.
* ''Ableitung als Tangentensteigung:'' Die Ableitung entspricht der Steigung, die die Tangente des Graphen an der Stelle der Ableitung besitzt. Damit löst die Ableitung das geometrische Problem, eine Tangente an einem Punkt eines Graphen zu bestimmen.
* ''Ableitung als Steigung der lokal besten linearen Approximation:'' Jede an einer Stelle ableitbare Funktion, kann in einer Umgebung um diesen Punkt gut durch eine lineare Funktion approximiert werden. Die Ableitung entspricht der Steigung dieser linearen Funktion. Damit kann die Ableitung genutzt werden, um Funktionen lokal durch lineare Funktion anzunähern.
* ''Ableitung als verallgemeinerte Steigung:'' Normalerweise ist die Steigung einer Funktion nur für lineare Funktionen definiert. Nun kann die Ableitung benutzt werden, um diese Steigung auch für andere Funktionen zu definieren. Hier entspricht die Ableitung der momentanen Steigung einer Abbildung.

Diese '''fett''' Intuitionen werden wir im Folgenden detailliert besprechen und aus ihnen werden wir eine formale Definition der Ableitung herleiten. Außerdem werden wir sehen, dass ableitbare Funktionen „knickfrei“ sind. Ableitbare Funktionen werden deswegen auch ''glatte Funktionen'' genannt.

== Ableitung als momentane Änderungsrate ==

Test123

=== Berechnung der Ableitung ===
Die Ableitung entspricht der momentanen Änderungsrate einer Funktion <math>f</math>. Wie kann diese momentane Änderungsrate einer Funktion bestimmt oder definiert werden? Sei zum Beispiel <math>f</math> eine reellwertige Funktion, die folgenden Graph besitzt:

templateargument 

{{Liste
 |item1=
Trichotomie der Positivität:

{{Formel|<math>\forall x: 0 < x \ \dot\or\  0 = x \ \dot\or\  0 < -x</math>}}
 |item2=
Abgeschlossenheit bezüglich Addition:

{{Formel|<math>\forall a,b: 0 < a \and 0 < b \Rightarrow 0 < a + b</math>}}
 |item3=
Abgeschlossenheit bezüglich Multiplikation:

{{Formel|<math>\forall a,b: 0 < a \and 0 < b \Rightarrow 0 < ab</math>}}
}}

# Element 1
# Element 2
# Element 3

==== Überschrift {{Anker|Testüberschrift}} ====
; nach oben beschränkte Folge
: Eine Folge ist nach oben beschränkt, wenn sie irgendeine obere Schranke besitzt.
; nach unten beschränkte Folge
: Eine Folge ist nach unten beschränkt, wenn sie irgendeine untere Schranke besitzt.
; beschränkte Folge
: Eine Folge ist beschränkt, wenn sie sowohl nach oben als auch nach unten beschränkt ist.

== Test2 ==

Test456

{{#invoke:Mathe für Nicht-Freaks/Seite|oben}}

Das Integral ist neben der Ableitung eines der wichtigsten Konzepte der Analysis. Es spielt eine wesentliche Rolle in vielen Anwendungen der Analysis.

== Wichtige Grundvorstellungen des Integrals ==
=== Integral als orientierte Fläche ===
Möglicherweise kennst du bereits die Vorstellung, dass das Integral einer Funktion gleich dem orientiertem Flächeninhalt unter dem Graphen dieser Funktion ist. So entspricht in der folgenden Abbildung das Integral <math>\int_a^b f(x)\,\mathrm d x</math> der positiven und stetigen Funktion <math>f:[a,b]\to\R</math> dem Inhalt der grauen Fläche, die nach oben durch den Funktionsgraphen, nach unten durch die <math>x</math>-Achse, nach links und rechts durch die Senkrechten <math>x=a</math> und <math>x=b</math> begrenzt wird:

templateargument 

In dieser Vorstellung ist die Betonung auf dem ''orientiertem'' Flächeninhalt wichtig. „Orientiert“ bedeutet hier, dass Flächeninhalte, die unter der <math>x</math>-Achse liegen (wo die Funktion negative Funktionswerte annimmt), negativ zum Flächeninhalt beitragen. Unterhalb der <math>x</math>-Achse liegende Flächeninhalt werden negativ gezählt. In der folgenden Abbildung entspricht das Integral <math>\int_a^b f(x)\,\mathrm d x</math> der Differenz zwischen der blauen Fläche minus der gelben Fläche: 

templateargument

Während also in der Geometrie der Inhalt einer Fläche immer positiv ist, kann das Integral als orientierte Fläche auch negativ werden (wenn die Funktion beispielsweise nur negative Funktionswerte besitzt).

=== Integral als Veränderung der Stammfunktion ===
{{:Mathe für Nicht-Freaks: Vorlage:Hauptartikel|Hauptsatz der Differential- und Integralrechnung}}

Möglicherweise kennst du auch eine andere Grundvorstellung des Integrals: Für stetige Funktionen <math>f</math> entspricht das Integral <math>\int_a^b f(x) \,\mathrm{d}x</math> der Differenz <math>F(b)-F(a)</math>, wobei die Funktion <math>F</math> eine beliebige Stammfunktion von <math>f</math> ist. Eine Stammfunktion von <math>f</math> ist dabei eine Funktion, deren Ableitung gleich <math>f</math> ist. Für Stammfunktionen <math>F</math> von <math>f</math> gilt also <math>F'(x)=f(x)</math> für alle Argumente <math>x</math> aus dem Definitionsbereich von <math>f</math>. Wenn nun <math>F</math> eine Stammfunktion von <math>f</math> ist, dann gilt

{{Formel|<math>\int_a^b f(x) \,\mathrm{d} x = F(b)-F(a)</math>}}

Das Integral <math>\int_a^b f(x) \,\mathrm{d}x</math> ist damit gleich der Veränderung <math>F(b)-F(a)</math> der Stammfunktion zwischen den Argumenten <math>a</math> und <math>b</math>. Diese Vorstellung ist wichtig, da so Integrale ausgerechnet werden können.

== Motivation und Herleitung des Integralbegriffs ==
Nun stellt sich die Frage: Was hat die orientierte Fläche zwischen dem Graphen und der <math>x</math>-Achse mit der Stammfunktion zu tun? Wieso sind diese beiden Vorstellungen zum Integral gleichwertig und wieso führen beide Vorstellungen zum selben Integralbegriff? Auch fehlt dir vielleicht noch eine Motivation des Integralbegriffs. Wieso solltest du lernen, was ein Integral ist? Was sind die Vorteile, die du daraus für dein Leben ziehen kannst? Es ist zwar interessant den Flächeninhalt von gekrümmten Flächen mit Hilfe des Integrals zu bestimmen. Doch erklärt dies nicht, warum in so vielen Studiengängen das Integral in den Mathevorlesungen unterrichtet wird. 

=== Grundmotivation des Integrals ===
Um die Frage nach dem Sinn des Integrals beantworten zu können, müssen wir einen Schritt zurückgehen. Betrachten wir hierzu folgendes Grundproblem:

{{-|Sei <math>F</math> eine unbekannte reellwertige Funktion, die auf einem Intervall <math>[a,b]</math> definiert ist. Sei die Funktion <math>f:[a,b]\to\R</math> die Ableitung von <math>F</math>, welche uns bekannt ist. Das bedeutet, dass wir an jeder Stelle <math>x\in [a,b]</math> die momentane Änderungsrate <math>f(x)</math> von der Funktion <math>F</math> (bezüglich der Variablen <math>x</math>) kennen. Wie können wir nun von diesem Wissen über die aktuelle Änderungsrate an jeder Stelle im Intervall <math>[a,b]</math> auf die globale Änderung von <math>F</math> in diesem Intervall schließen? Wenn wir doch an jeder Stelle aus dem Intervall <math>[a,b]</math> die aktuelle Änderungsrate von <math>F</math> kennen, dann sollte es möglich sein, die Gesamtänderung von <math>F</math> in diesem Intervall zu bestimmen. Aber wie groß ist diese?}}

Mit Hilfe des Integrals können wir das obige Problem lösen. Wir sagen hierzu:

{{-|Das Integral <math>\int_a^b f(x) \,\mathrm{d}x</math> ist gleich der Gesamtänderung einer Funktion zwischen den Argumenten <math>a</math> und <math>b</math>, die an jeder Stelle <math>x</math> die Ableitung <math>f(x)</math> besitzt.}}

Das Integral ist damit eine Art der Umkehrung der Ableitung. Während die Ableitung einer Funktion ihre momentane Änderungsrate angibt, können wir mit Hilfe des Integrals die Gesamtänderung einer Funktion bestimmen, wenn wir an jeder Stelle ihre aktuelle Änderungsrate, sprich ihre Ableitung, kennen.

In dieser Intuition des Integrals erkennen wir bereits eine Grundvorstellung des Intregrals: Die Gesamtänderung von <math>F</math> im Intervall <math>[a,b]</math> ist gleich der Differenz <math>F(b)-F(a)</math>. Wenn nun das Integral <math>\int_a^b f(x) \,\mathrm{d}x</math> gleich dieser Gesamtänderung ist, dann muss gelten:

{{Formel|<math>\int_a^b f(x) \,\mathrm{d}x = F(b)-F(a)</math>}}

Wenn nun an jeder Stelle <math>x</math> aus dem Intervall <math>[a,b]</math> die Ableitung von <math>F</math> gleich <math>f(x)</math> ist (wenn also <math>F'(x)=f(x)</math> ist), dann ist <math>F</math> eine Stammfunktion von <math>f</math>. Damit entspricht unsere obige Intuition der Grundvorstellung, dass das Integral gleich der Veränderung der Stammfunktion ist.

=== Relevanz des Integrals ===
{{todo|Abschnitt ergänzen}}

=== Herleitung des Integralbegriffs ===

{{todo|*Bezeichnung Unterteilung/Stützpunkte prüfen?}}

Wir wissen nun, wozu wir das Integral einsetzen können: Um die Gesamtänderung einer Funktion <math>F</math> über ein Intervall <math>[a,b]</math> zu bestimmen, deren Ableitung wir an jeder Stelle des Definitionsbereichs kennen. Sei <math>f := F'</math> die bekannte Ableitung von <math>F</math>. Als erste Annäherung an dieses Ziel könnten wir die Änderungsrate am Startpunkt (also <math>f(a)</math>) betrachten und mit der Länge des Intervall <math>[a,b]</math> multiplizieren um eine Approximation an die Gesamtänderung <math>F(b) - F(a)</math> zu erhalten. Es sollte also gelten:

templateargument

{{Formel|<math> f(a)(b-a) \approx F(b) - F(a)</math>}}

Für kleine Intervalle <math>[a,b]</math> ist dies auch tatsächlich eine gute Näherung, wie wir aus der templateargument sehen können.  

Zur Wiederholung: Eine differenzierbare Funktion <math>F</math> können wir in der Nähe eines Punktes <math>a</math> durch ihre Ableitung <math>F'(a)</math> an diesem Punkt approximieren:

{{Formel|<math>F(a) + F'(a)(b-a) \approx F(b)</math>}} 

Diese Approximation ist gut, wenn der Abstand zwischen <math>a</math> und <math>b</math> klein ist, und wird im Allgemeinen schlechter, wenn dieser Abstand größer wird. 

Um die Näherung der Gesamtänderung zu verbessern, können wir das Gesamtintervall in zwei kleinere Teilintervalle <math>[a,c]</math> und <math>[c,b]</math> einteilen. Dann verwenden wir für jedes der beiden die obige Approximation und addieren die Ergebnisse. 

templateargument

{{Formel|<math> f(a)(c - a) + f(c)(b-c) \approx F(b) - F(a)</math>}}

Ganz allgemein können wir das Gesamtintervall in <math>n</math> Teilintervalle unterteilen, auf jedem davon eine Näherung für die Gesamtänderung bestimmen und all diese aufsummieren. Dies führt uns zum Begriff der Unterteilung:

{{:Mathe für Nicht-Freaks: Vorlage:Definition
 |titel=(Riemann-)Zerlegung
 |definition=
Eine <dfn title="Zerlegung">(Riemann-)Zerlegung <math>(\mathcal{Z}, \mathcal{T})</math></dfn> eines Intervalles <math>[a,b]</math> ist eine Menge von Intervallen <math>\mathcal{Z} = \{[x_0, x_1], [x_1,x_2], \dots, [x_{n-1},x_n]\}</math> mit <math>a=x_0 < x_1 < \dots < x_n = b</math> zusammen mit einer Menge von Stützstellen <math>\mathcal{T} = \{t_1, \dots, t_n\}</math>, sodass in jedem Intervall genau eine Stützstelle liegt, d.h. <math>t_i \in [x_{i-1},x_i]</math>.

Als <dfn>Feinheit der Zerlegung</dfn> bezeichnen wir dann die Breite des größten Intervalls der Zerlegung, also die Zahl <math>\mu(\mathcal{Z},\mathcal{T}) := \max\{x_1-x_0, x_2-x_1, \dots, x_n-x_{n-1}\}</math>.
}}

Für jede solche Zerlegung können wir dann wie folgt eine Approximation an die Gesamtänderung der Funktion <math>F</math> unter ausschließlicher Verwendung ihrer Ableitung <math>f</math> definieren:

{{:Mathe für Nicht-Freaks: Vorlage:Definition
 |definition=
Sei <math>f:[a,b] \to \mathbb{R}</math> eine Funktion und <math>(\mathcal{Z}, \mathcal{T})</math> eine Zerlegung. Dann ist die <dfn title="Riemannsumme">Riemannsumme</dfn> von <math>f</math> bezüglich <math>(\mathcal{Z}, \mathcal{T})</math> definiert als

{{Formel|<math>S(f,(\mathcal{Z}, \mathcal{T})) := \sum_{i=1}^{n} f(t_i) \cdot (x_i - x_{i-1})</math>}}
|titel=Riemannsumme
 }}

Wir erwarten nun, dass sich die Riemannsumme für immer feiner werdende Zerlegungen immer weiter dem tatsächlichen Wert der Gesamtänderung <math>F(b)-F(a)</math> annähert. Es sollte also gelten:

{{Formel|<math>\lim_{\mu(\mathcal{Z},\mathcal{T})\to 0} S(f,(\mathcal{Z}, \mathcal{T})) = F(b) - F(a)</math>}}

Dies ist - wie wir gleich sehen werden - auch tatsächlich der Fall. Und wie du siehst benötigen wir zum Berechnen der linken Seite ausschließlich die Ableitung <math>f</math>, nicht jedoch die ursprüngliche Funktion <math>F</math>. Daher können wir versuchen die linke Seite ganz allgemein für beliebige Funktionen <math>f</math> zu definieren. Auch solche, die nicht als Ableitung einer anderen Funktion gegeben sind. Wir nennen diesen Grenzwert - so er denn existiert - dann das ''Integral von <math>f</math>''. 

Bevor wir diese Definition jedoch formalisieren, sollten wir aber noch kurz über eine andere Frage nachdenken: Warum müssen wir den Grenzwert über ''alle'' Zerlegungen betrachten? Genügt es nicht den Grenzwert über eine feste Folge von immer feiner werdenden Zerlegungen zu betrachten? Etwa Zerlegungen in <math>n</math> gleich große Intervalle, wobei <math>n</math> immer größer wird?

Dazu betrachten wir die sogenannte [https://de.wikipedia.org/wiki/Dirichlet-Funktion Dirichlet-Funktion]:

{{Formel|<math>D:[0,1] \to \mathbb{R}: x \mapsto \begin{cases}1, &x \in \mathbb{Q} \\0, &x \notin \mathbb{Q}\end{cases}</math>}}

Diese Funktion bildet also alle rationalen Zahlen auf <math>1</math> und alle irrationalen Zahlen auf <math>0</math> ab. Für die Zerlegungen 

{{Formel|<math>(\mathcal{Z}, \mathcal{T}) := \left(\{[0,\tfrac{1}{n}], [\tfrac{1}{n}, \tfrac{2}{n}], \dots, [\tfrac{n-1}{n}, 1]\}, \{0, \tfrac{1}{n}, \dots, \tfrac{n-1}{n}\}\right)</math>}}

erhalten wir die Riemannsumme

{{Formel|<math>S(D, \mathcal{Z}, \mathcal{T}) = \sum_{i=0}^{n-1} D\left(\frac{i}{n}\right)\cdot \frac{1}{n} \overset{\color{Gray}\tfrac{i}{n} \in \mathbb{Q}}{=} \sum_{i=0}^{n-1} 1\cdot \frac{1}{n} = 1</math>}}

Der Grenzwert über diese Zerlegungen wäre daher ebenfalls <math>1</math>.

Verwenden wir jedoch Zerlegungen <math>(\mathcal{Z}, \tilde{\mathcal{T}})</math>, deren Stützstellen lauter irrationale Zahlen sind, so gilt:

{{Formel|<math>S(D, \mathcal{Z}, \tilde{\mathcal{T}}) = \sum_{i=0}^{n-1} D\left(t_i\right)\cdot \frac{1}{n} \overset{\color{Gray}t_i \notin \mathbb{Q}}{=} \sum_{i=0}^{n-1} 0\cdot \frac{1}{n} = 0</math>}}

Also wäre der über diese Zerlegungen berechnete Grenzwert <math>0</math>.

In Fall der Dirichlet-Funktion würde also das Integral von der Wahl der Folge von Zerlegungen abhängen, die wir zu seiner Bestimmung verwenden. Um derartige Probleme zu vermeiden, wollen wir das Integral einer Funktion nur in dem Fall definieren, dass ''jede'' Folge immer feiner werdender Zerlegungen zum selben Grenzwert führen.

{{:Mathe für Nicht-Freaks: Vorlage:Definition
 |definition=
Eine Funktion <math>f:[a,b] \to \R</math> heißt <dfn title="Riemann-Integrierbarkeit">Riemann-integrierbar</dfn>, wenn es ein <math>I \in \mathbb{R}</math> gibt, sodass gilt

{{Formel|<math>\lim_{\mu(\mathcal{Z},\mathcal{T})\to 0}\lVert S(f,\mathcal{Z},\mathcal{T}) - I\rVert \to 0</math>}}

d.h. wenn die Riemannsumme für (beliebige) immer feiner werdene Zerlegungen gegen den Wert <math>I</math> konvergiert.

Wir bezeichnen dann <math>\int_a^b f(x) \mathrm{d}x := I</math> als <dfn title="Riemann-Integral">(Riemann-)Integral</dfn> von <math>f</math> von <math>a</math> bis <math>b</math>.
|titel=Riemann-Integral
 }}


{{todo|Zusammenhang hzur Intuition?}}

{{#invoke:Mathe für Nicht-Freaks/Seite|unten|quellen=
* Greefrath, G., Oldenburg, R., Siller, H. S., Ulm, V., & Weigand, H. G. (2016). ''Aspects and “Grundvorstellungen” of the Concepts of Derivative and Integral. Journal für Mathematik-Didaktik, 37''(1), 99-129.
}}
